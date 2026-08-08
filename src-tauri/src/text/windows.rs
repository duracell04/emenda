use super::{CapturedSelection, SourceApplication, TextSurfaceAdapter, TextSurfaceError};
use active_win_pos_rs::get_active_window;
use clipboard_rs::{Clipboard, ClipboardContent, ClipboardContext, ContentFormat};
use enigo::{
    Direction::{Click, Press, Release},
    Enigo, Key, Keyboard, Settings,
};
use std::{
    collections::HashSet,
    process::Command,
    sync::{
        atomic::{AtomicU64, Ordering},
        Mutex, MutexGuard,
    },
    thread,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

const UNKNOWN_CLIPBOARD_FORMAT: &str = "unknown format";
const RTF_CLIPBOARD_FORMAT: &str = "Rich Text Format";
const HTML_CLIPBOARD_FORMAT: &str = "HTML Format";
const PNG_CLIPBOARD_FORMAT: &str = "PNG";
const CREATE_NO_WINDOW: u32 = 0x0800_0000;

static NEXT_MARKER: AtomicU64 = AtomicU64::new(1);

/// Windows V0.1 clipboard-assisted text transport.
///
/// A mutex serializes access because the Windows clipboard is a global,
/// single-writer resource and overlapping hotkey/apply operations could
/// otherwise restore snapshots out of order.
pub struct WindowsTextSurfaceAdapter {
    gate: Mutex<()>,
    timing: TransportTiming,
}

impl WindowsTextSurfaceAdapter {
    pub fn new() -> Result<Self, TextSurfaceError> {
        ClipboardContext::new().map_err(|error| {
            TextSurfaceError::Capture(format!(
                "The Windows clipboard could not be initialised: {error}"
            ))
        })?;
        Enigo::new(&Settings::default()).map_err(|error| {
            TextSurfaceError::Capture(format!(
                "Windows input simulation could not be initialised: {error}"
            ))
        })?;

        Ok(Self {
            gate: Mutex::new(()),
            timing: TransportTiming::default(),
        })
    }

    fn lock_for_capture(&self) -> Result<MutexGuard<'_, ()>, TextSurfaceError> {
        self.gate.lock().map_err(|_| {
            TextSurfaceError::Capture(
                "The desktop text adapter is unavailable after an internal failure".into(),
            )
        })
    }

    fn lock_for_replacement(&self) -> Result<MutexGuard<'_, ()>, TextSurfaceError> {
        self.gate.lock().map_err(|_| {
            TextSurfaceError::Replacement(
                "The desktop text adapter is unavailable after an internal failure".into(),
            )
        })
    }
}

impl TextSurfaceAdapter for WindowsTextSurfaceAdapter {
    fn capture_selection(&self) -> Result<CapturedSelection, TextSurfaceError> {
        let _guard = self.lock_for_capture()?;
        let mut clipboard = WindowsClipboard::new(CaptureOrReplace::Capture)?;
        let mut input = WindowsInput::new(CaptureOrReplace::Capture)?;
        let mut windows = WindowsWindowControl;
        let mut delay = ThreadDelay;
        let marker = unique_clipboard_marker();

        capture_with_ports(
            &mut clipboard,
            &mut input,
            &mut windows,
            &mut delay,
            &self.timing,
            &marker,
        )
    }

    fn focus_source(&self, source: &SourceApplication) -> Result<(), TextSurfaceError> {
        let _guard = self.lock_for_replacement()?;
        let mut windows = WindowsWindowControl;
        let mut delay = ThreadDelay;
        focus_source_with_ports(&mut windows, &mut delay, source, &self.timing)
    }

    fn replace_selection(
        &self,
        source: &SourceApplication,
        replacement: &str,
    ) -> Result<(), TextSurfaceError> {
        if replacement.is_empty() {
            return Err(TextSurfaceError::Replacement(
                "Clipboard-assisted replacement cannot apply an empty passage".into(),
            ));
        }

        let _guard = self.lock_for_replacement()?;
        let mut clipboard = WindowsClipboard::new(CaptureOrReplace::Replace)?;
        let mut input = WindowsInput::new(CaptureOrReplace::Replace)?;
        let mut windows = WindowsWindowControl;
        let mut delay = ThreadDelay;

        replace_with_ports(
            &mut clipboard,
            &mut input,
            &mut windows,
            &mut delay,
            &self.timing,
            source,
            replacement,
        )
    }
}

#[derive(Clone, Copy)]
struct TransportTiming {
    hotkey_release_delay: Duration,
    clipboard_poll_attempts: usize,
    clipboard_poll_interval: Duration,
    focus_poll_attempts: usize,
    focus_poll_interval: Duration,
    paste_settle_delay: Duration,
}

impl Default for TransportTiming {
    fn default() -> Self {
        Self {
            // The global-shortcut callback can run before the user's physical
            // modifier keys are released. Letting them settle prevents an
            // accidental Ctrl+Alt+C (or similar) instead of plain Ctrl+C.
            hotkey_release_delay: Duration::from_millis(60),
            // Give slower Electron/web text surfaces just under one second to
            // render their delayed clipboard data.
            clipboard_poll_attempts: 60,
            clipboard_poll_interval: Duration::from_millis(15),
            focus_poll_attempts: 20,
            focus_poll_interval: Duration::from_millis(25),
            // Some applications consume WM_PASTE asynchronously. Restoring
            // before that message is handled can paste the user's old data.
            paste_settle_delay: Duration::from_millis(120),
        }
    }
}

#[derive(Clone, Copy)]
enum CaptureOrReplace {
    Capture,
    Replace,
}

impl CaptureOrReplace {
    fn error(self, message: impl Into<String>) -> TextSurfaceError {
        match self {
            Self::Capture => TextSurfaceError::Capture(message.into()),
            Self::Replace => TextSurfaceError::Replacement(message.into()),
        }
    }
}

trait ClipboardPort {
    type Snapshot;

    fn snapshot(&mut self) -> Result<Self::Snapshot, String>;
    fn set_text(&mut self, text: &str) -> Result<(), String>;
    fn read_text(&mut self) -> Result<String, String>;
    fn restore(&mut self, snapshot: Self::Snapshot) -> Result<(), String>;
}

trait InputPort {
    fn copy(&mut self) -> Result<(), String>;
    fn paste(&mut self) -> Result<(), String>;
}

trait WindowPort {
    fn active_source(&mut self) -> Result<SourceApplication, String>;
    fn request_focus(&mut self, source: &SourceApplication) -> Result<(), String>;
    fn is_active(&mut self, source: &SourceApplication) -> Result<bool, String>;
}

trait DelayPort {
    fn sleep(&mut self, duration: Duration);
}

fn capture_with_ports<C, I, W, D>(
    clipboard: &mut C,
    input: &mut I,
    windows: &mut W,
    delay: &mut D,
    timing: &TransportTiming,
    marker: &str,
) -> Result<CapturedSelection, TextSurfaceError>
where
    C: ClipboardPort,
    I: InputPort,
    W: WindowPort,
    D: DelayPort,
{
    let source = windows.active_source().map_err(|error| {
        TextSurfaceError::Capture(format!(
            "The source application could not be identified: {error}"
        ))
    })?;

    let text = with_restored_clipboard(clipboard, CaptureOrReplace::Capture, |clipboard| {
        clipboard.set_text(marker).map_err(|error| {
            TextSurfaceError::Capture(format!(
                "The clipboard could not be prepared for selected-text capture: {error}"
            ))
        })?;

        if !timing.hotkey_release_delay.is_zero() {
            delay.sleep(timing.hotkey_release_delay);
        }

        let still_active = windows.is_active(&source).map_err(|error| {
            TextSurfaceError::Capture(format!(
                "The source application could not be checked before capture: {error}"
            ))
        })?;
        if !still_active {
            return Err(TextSurfaceError::Capture(
                "The source application lost focus before its selection could be captured".into(),
            ));
        }

        input.copy().map_err(|error| {
            TextSurfaceError::Capture(format!(
                "Windows could not send the copy shortcut to the source application: {error}"
            ))
        })?;

        wait_for_copied_text(clipboard, delay, timing, marker)
    })?;

    Ok(CapturedSelection { text, source })
}

fn replace_with_ports<C, I, W, D>(
    clipboard: &mut C,
    input: &mut I,
    windows: &mut W,
    delay: &mut D,
    timing: &TransportTiming,
    source: &SourceApplication,
    replacement: &str,
) -> Result<(), TextSurfaceError>
where
    C: ClipboardPort,
    I: InputPort,
    W: WindowPort,
    D: DelayPort,
{
    with_restored_clipboard(clipboard, CaptureOrReplace::Replace, |clipboard| {
        clipboard.set_text(replacement).map_err(|error| {
            TextSurfaceError::Replacement(format!(
                "The clipboard could not be prepared for replacement: {error}"
            ))
        })?;

        focus_source_with_ports(windows, delay, source, timing)?;
        input.paste().map_err(|error| {
            TextSurfaceError::Replacement(format!(
                "Windows could not send the paste shortcut to the source application: {error}"
            ))
        })?;
        delay.sleep(timing.paste_settle_delay);
        Ok(())
    })
}

fn focus_source_with_ports<W, D>(
    windows: &mut W,
    delay: &mut D,
    source: &SourceApplication,
    timing: &TransportTiming,
) -> Result<(), TextSurfaceError>
where
    W: WindowPort,
    D: DelayPort,
{
    windows.request_focus(source).map_err(|error| {
        TextSurfaceError::ProtectedSurface(format!(
            "Windows could not return focus to {}: {error}. The window may have closed or be running elevated above Emenda",
            source.application_name
        ))
    })?;

    for attempt in 0..=timing.focus_poll_attempts {
        if windows.is_active(source).unwrap_or(false) {
            return Ok(());
        }
        if attempt < timing.focus_poll_attempts {
            delay.sleep(timing.focus_poll_interval);
        }
    }

    Err(TextSurfaceError::ProtectedSurface(format!(
        "Windows did not activate the original {} window. It may have closed, changed identity, or be running elevated above Emenda",
        source.application_name
    )))
}

fn wait_for_copied_text<C, D>(
    clipboard: &mut C,
    delay: &mut D,
    timing: &TransportTiming,
    marker: &str,
) -> Result<String, TextSurfaceError>
where
    C: ClipboardPort,
    D: DelayPort,
{
    for attempt in 0..=timing.clipboard_poll_attempts {
        if let Ok(text) = clipboard.read_text() {
            if text != marker {
                if text.is_empty() {
                    return Err(TextSurfaceError::Capture(
                        "The selected passage is empty".into(),
                    ));
                }
                return Ok(text);
            }
        }
        if attempt < timing.clipboard_poll_attempts {
            delay.sleep(timing.clipboard_poll_interval);
        }
    }

    Err(TextSurfaceError::Capture(
        "The source application did not expose selected text before capture timed out. Ensure editable text is selected and the source is not an elevated or protected surface"
            .into(),
    ))
}

/// Runs a clipboard-assisted operation and always attempts restoration.
///
/// When both the operation and restoration fail, the original typed failure is
/// retained and augmented. This matters for replacement: callers must not
/// retry blindly because the paste may already have reached the source.
fn with_restored_clipboard<C, T>(
    clipboard: &mut C,
    operation_kind: CaptureOrReplace,
    operation: impl FnOnce(&mut C) -> Result<T, TextSurfaceError>,
) -> Result<T, TextSurfaceError>
where
    C: ClipboardPort,
{
    let snapshot = clipboard.snapshot().map_err(|error| {
        operation_kind.error(format!(
            "The existing clipboard could not be preserved: {error}"
        ))
    })?;

    let operation_result = operation(clipboard);
    let restoration_result = clipboard.restore(snapshot);

    match (operation_result, restoration_result) {
        (Ok(value), Ok(())) => Ok(value),
        (Ok(_), Err(restore_error)) => Err(operation_kind.error(format!(
            "The desktop operation completed, but the previous clipboard could not be restored: {restore_error}"
        ))),
        (Err(operation_error), Ok(())) => Err(operation_error),
        (Err(operation_error), Err(restore_error)) => Err(append_restoration_failure(
            operation_error,
            &restore_error,
        )),
    }
}

fn append_restoration_failure(
    operation_error: TextSurfaceError,
    restore_error: &str,
) -> TextSurfaceError {
    let append = |message: String| {
        format!("{message}; the previous clipboard also could not be restored: {restore_error}")
    };

    match operation_error {
        TextSurfaceError::Capture(message) => TextSurfaceError::Capture(append(message)),
        TextSurfaceError::Replacement(message) => TextSurfaceError::Replacement(append(message)),
        TextSurfaceError::ProtectedSurface(message) => {
            TextSurfaceError::ProtectedSurface(append(message))
        }
        TextSurfaceError::UnsupportedPlatform => TextSurfaceError::UnsupportedPlatform,
    }
}

struct WindowsClipboard {
    context: ClipboardContext,
}

struct ClipboardSnapshot {
    contents: Vec<ClipboardContent>,
}

impl WindowsClipboard {
    fn new(operation: CaptureOrReplace) -> Result<Self, TextSurfaceError> {
        let context = ClipboardContext::new().map_err(|error| {
            operation.error(format!("The Windows clipboard is unavailable: {error}"))
        })?;
        Ok(Self { context })
    }
}

impl ClipboardPort for WindowsClipboard {
    type Snapshot = ClipboardSnapshot;

    fn snapshot(&mut self) -> Result<Self::Snapshot, String> {
        let available = self
            .context
            .available_formats()
            .map_err(|error| error.to_string())?;

        let mut requested = Vec::new();
        // Image must be restored first: clipboard-rs clears the clipboard while
        // setting an image, then adds the remaining formats without clearing.
        if self.context.has(ContentFormat::Image) {
            requested.push(ContentFormat::Image);
        }
        if self.context.has(ContentFormat::Text) {
            requested.push(ContentFormat::Text);
        }
        if self.context.has(ContentFormat::Rtf) {
            requested.push(ContentFormat::Rtf);
        }
        if self.context.has(ContentFormat::Html) {
            requested.push(ContentFormat::Html);
        }
        if self.context.has(ContentFormat::Files) {
            requested.push(ContentFormat::Files);
        }

        let mut named_formats = HashSet::new();
        for format in &available {
            if format == UNKNOWN_CLIPBOARD_FORMAT
                || format == RTF_CLIPBOARD_FORMAT
                || format == HTML_CLIPBOARD_FORMAT
                || format == PNG_CLIPBOARD_FORMAT
                || !named_formats.insert(format.clone())
            {
                continue;
            }
            requested.push(ContentFormat::Other(format.clone()));
        }

        let contents = self
            .context
            .get(&requested)
            .map_err(|error| error.to_string())?;
        if contents.is_empty() && !available.is_empty() {
            return Err(
                "the clipboard contains data, but none of its formats could be read safely".into(),
            );
        }

        Ok(ClipboardSnapshot { contents })
    }

    fn set_text(&mut self, text: &str) -> Result<(), String> {
        self.context
            .set_text(text.to_owned())
            .map_err(|error| error.to_string())
    }

    fn read_text(&mut self) -> Result<String, String> {
        self.context.get_text().map_err(|error| error.to_string())
    }

    fn restore(&mut self, snapshot: Self::Snapshot) -> Result<(), String> {
        if snapshot.contents.is_empty() {
            self.context.clear().map_err(|error| error.to_string())
        } else {
            self.context
                .set(snapshot.contents)
                .map_err(|error| error.to_string())
        }
    }
}

struct WindowsInput {
    enigo: Enigo,
}

impl WindowsInput {
    fn new(operation: CaptureOrReplace) -> Result<Self, TextSurfaceError> {
        let enigo = Enigo::new(&Settings::default()).map_err(|error| {
            operation.error(format!(
                "Windows input simulation could not be initialised: {error}"
            ))
        })?;
        Ok(Self { enigo })
    }

    fn control_shortcut(&mut self, key: Key) -> Result<(), String> {
        self.enigo
            .key(Key::Control, Press)
            .map_err(|error| error.to_string())?;

        let key_result = self
            .enigo
            .key(key, Click)
            .map_err(|error| error.to_string());
        let release_result = self
            .enigo
            .key(Key::Control, Release)
            .map_err(|error| error.to_string());

        key_result.and(release_result)
    }
}

impl InputPort for WindowsInput {
    fn copy(&mut self) -> Result<(), String> {
        self.control_shortcut(Key::C)
    }

    fn paste(&mut self) -> Result<(), String> {
        self.control_shortcut(Key::V)
    }
}

struct WindowsWindowControl;

impl WindowPort for WindowsWindowControl {
    fn active_source(&mut self) -> Result<SourceApplication, String> {
        active_source()
    }

    fn request_focus(&mut self, source: &SourceApplication) -> Result<(), String> {
        request_process_focus(source.process_id)
    }

    fn is_active(&mut self, source: &SourceApplication) -> Result<bool, String> {
        let active = active_source()?;
        Ok(active.process_id == source.process_id && active.window_id == source.window_id)
    }
}

fn active_source() -> Result<SourceApplication, String> {
    let active = get_active_window().map_err(|()| "no foreground window was available")?;
    let executable = if active.process_path.as_os_str().is_empty() {
        None
    } else {
        Some(active.process_path.to_string_lossy().into_owned())
    };

    Ok(SourceApplication {
        process_id: active.process_id,
        application_name: active.app_name,
        executable,
        window_title: active.title,
        window_id: active.window_id,
    })
}

fn request_process_focus(process_id: u64) -> Result<(), String> {
    let process_id = i32::try_from(process_id)
        .map_err(|_| "the source process identifier is invalid".to_owned())?;
    // WScript.Shell.AppActivate is a safe, built-in Windows automation API.
    // Only the validated numeric PID is interpolated into this constant script.
    let script = format!(
        "$shell = New-Object -ComObject WScript.Shell; if (-not $shell.AppActivate([int]{process_id})) {{ exit 20 }}"
    );

    use std::os::windows::process::CommandExt;
    let status = Command::new("powershell.exe")
        .args([
            "-NoLogo",
            "-NoProfile",
            "-NonInteractive",
            "-WindowStyle",
            "Hidden",
            "-Command",
            &script,
        ])
        .creation_flags(CREATE_NO_WINDOW)
        .status()
        .map_err(|error| format!("the Windows focus helper could not start: {error}"))?;

    if status.success() {
        Ok(())
    } else {
        Err("the operating system rejected the focus request".into())
    }
}

struct ThreadDelay;

impl DelayPort for ThreadDelay {
    fn sleep(&mut self, duration: Duration) {
        thread::sleep(duration);
    }
}

fn unique_clipboard_marker() -> String {
    let sequence = NEXT_MARKER.fetch_add(1, Ordering::Relaxed);
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos();
    format!(
        "__emenda_selection_{}_{}_{}__",
        std::process::id(),
        nanos,
        sequence
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{cell::RefCell, collections::VecDeque, rc::Rc};

    type Events = Rc<RefCell<Vec<String>>>;

    struct MockClipboard {
        events: Events,
        current: String,
        reads: VecDeque<Result<String, String>>,
        fail_restore: bool,
    }

    impl ClipboardPort for MockClipboard {
        type Snapshot = String;

        fn snapshot(&mut self) -> Result<Self::Snapshot, String> {
            self.events.borrow_mut().push("snapshot".into());
            Ok(self.current.clone())
        }

        fn set_text(&mut self, text: &str) -> Result<(), String> {
            self.events.borrow_mut().push(format!("set:{text}"));
            self.current = text.into();
            Ok(())
        }

        fn read_text(&mut self) -> Result<String, String> {
            self.events.borrow_mut().push("read".into());
            self.reads
                .pop_front()
                .unwrap_or_else(|| Ok(self.current.clone()))
        }

        fn restore(&mut self, snapshot: Self::Snapshot) -> Result<(), String> {
            self.events.borrow_mut().push("restore".into());
            if self.fail_restore {
                Err("restore unavailable".into())
            } else {
                self.current = snapshot;
                Ok(())
            }
        }
    }

    struct MockInput {
        events: Events,
        fail_copy: bool,
        paste_count: usize,
    }

    impl InputPort for MockInput {
        fn copy(&mut self) -> Result<(), String> {
            self.events.borrow_mut().push("copy".into());
            if self.fail_copy {
                Err("copy denied".into())
            } else {
                Ok(())
            }
        }

        fn paste(&mut self) -> Result<(), String> {
            self.events.borrow_mut().push("paste".into());
            self.paste_count += 1;
            Ok(())
        }
    }

    struct MockWindows {
        events: Events,
        source: SourceApplication,
        active: bool,
    }

    impl WindowPort for MockWindows {
        fn active_source(&mut self) -> Result<SourceApplication, String> {
            self.events.borrow_mut().push("active-source".into());
            Ok(self.source.clone())
        }

        fn request_focus(&mut self, _source: &SourceApplication) -> Result<(), String> {
            self.events.borrow_mut().push("focus".into());
            self.active = true;
            Ok(())
        }

        fn is_active(&mut self, _source: &SourceApplication) -> Result<bool, String> {
            self.events.borrow_mut().push("is-active".into());
            Ok(self.active)
        }
    }

    struct NoDelay {
        events: Events,
    }

    impl DelayPort for NoDelay {
        fn sleep(&mut self, _duration: Duration) {
            self.events.borrow_mut().push("sleep".into());
        }
    }

    fn source() -> SourceApplication {
        SourceApplication {
            process_id: 123,
            application_name: "Editor".into(),
            executable: Some("editor.exe".into()),
            window_title: "Draft".into(),
            window_id: "HWND(456)".into(),
        }
    }

    fn timing() -> TransportTiming {
        TransportTiming {
            hotkey_release_delay: Duration::ZERO,
            clipboard_poll_attempts: 2,
            clipboard_poll_interval: Duration::ZERO,
            focus_poll_attempts: 1,
            focus_poll_interval: Duration::ZERO,
            paste_settle_delay: Duration::ZERO,
        }
    }

    fn clipboard(events: &Events) -> MockClipboard {
        MockClipboard {
            events: events.clone(),
            current: "user clipboard".into(),
            reads: VecDeque::new(),
            fail_restore: false,
        }
    }

    fn input(events: &Events) -> MockInput {
        MockInput {
            events: events.clone(),
            fail_copy: false,
            paste_count: 0,
        }
    }

    fn windows(events: &Events, active: bool) -> MockWindows {
        MockWindows {
            events: events.clone(),
            source: source(),
            active,
        }
    }

    #[test]
    fn capture_restores_the_clipboard_after_polling_for_copy() {
        let events = Events::default();
        let mut clipboard = clipboard(&events);
        clipboard.reads = VecDeque::from([Ok("marker".into()), Ok("selected text".into())]);
        let mut input = input(&events);
        let mut windows = windows(&events, true);
        let mut delay = NoDelay {
            events: events.clone(),
        };

        let captured = capture_with_ports(
            &mut clipboard,
            &mut input,
            &mut windows,
            &mut delay,
            &timing(),
            "marker",
        )
        .expect("capture should succeed");

        assert_eq!(captured.text, "selected text");
        assert_eq!(captured.source, source());
        assert_eq!(clipboard.current, "user clipboard");
        assert_eq!(
            events.borrow().as_slice(),
            [
                "active-source",
                "snapshot",
                "set:marker",
                "is-active",
                "copy",
                "read",
                "sleep",
                "read",
                "restore",
            ]
        );
    }

    #[test]
    fn operation_failure_still_restores_the_clipboard() {
        let events = Events::default();
        let mut clipboard = clipboard(&events);
        let mut input = input(&events);
        input.fail_copy = true;
        let mut windows = windows(&events, true);
        let mut delay = NoDelay {
            events: events.clone(),
        };

        let error = capture_with_ports(
            &mut clipboard,
            &mut input,
            &mut windows,
            &mut delay,
            &timing(),
            "marker",
        )
        .expect_err("copy should fail");

        assert!(matches!(error, TextSurfaceError::Capture(_)));
        assert_eq!(clipboard.current, "user clipboard");
        assert_eq!(events.borrow().last().map(String::as_str), Some("restore"));
    }

    #[test]
    fn reports_both_operation_and_restoration_failures() {
        let events = Events::default();
        let mut clipboard = clipboard(&events);
        clipboard.fail_restore = true;

        let error =
            with_restored_clipboard(&mut clipboard, CaptureOrReplace::Capture, |_clipboard| {
                Err::<(), _>(TextSurfaceError::Capture("copy failed".into()))
            })
            .expect_err("transaction should fail");

        assert!(error.to_string().contains("copy failed"));
        assert!(error.to_string().contains("could not be restored"));
    }

    #[test]
    fn replacement_refocuses_and_uses_exactly_one_paste() {
        let events = Events::default();
        let mut clipboard = clipboard(&events);
        let mut input = input(&events);
        let mut windows = windows(&events, false);
        let mut delay = NoDelay {
            events: events.clone(),
        };

        replace_with_ports(
            &mut clipboard,
            &mut input,
            &mut windows,
            &mut delay,
            &timing(),
            &source(),
            "corrected text",
        )
        .expect("replacement should succeed");

        assert_eq!(input.paste_count, 1);
        assert_eq!(clipboard.current, "user clipboard");
        let events = events.borrow();
        let set_position = events
            .iter()
            .position(|event| event == "set:corrected text")
            .expect("replacement should reach clipboard");
        let focus_position = events
            .iter()
            .position(|event| event == "focus")
            .expect("source should be focused");
        let paste_position = events
            .iter()
            .position(|event| event == "paste")
            .expect("source should receive paste");
        let restore_position = events
            .iter()
            .position(|event| event == "restore")
            .expect("clipboard should be restored");

        assert!(set_position < focus_position);
        assert!(focus_position < paste_position);
        assert!(paste_position < restore_position);
    }

    #[test]
    fn marker_is_unique_between_capture_attempts() {
        assert_ne!(unique_clipboard_marker(), unique_clipboard_marker());
    }
}
