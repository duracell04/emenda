#![cfg(target_os = "windows")]

use active_win_pos_rs::get_active_window;
use emenda_lib::{
    error::CoreError,
    inference::{InferenceProvider, OpenRouterProvider},
    settings::{ApiKey, CredentialStore, SettingsStore, OPENROUTER_API_KEY_ENV},
    text::{platform_adapter, TextSurfaceAdapter},
    workflow::{WorkflowController, WorkflowState},
};
use enigo::{
    Direction::{Click, Press, Release},
    Enigo, Key, Keyboard, Settings as EnigoSettings,
};
use std::{
    env, fs, io,
    path::{Path, PathBuf},
    process::{Child, Command, Stdio},
    sync::{Arc, Mutex},
    thread,
    time::{Duration, Instant},
};

const ORIGINAL_TEXT: &str = "I liek this sentence.";
const CORRECTED_TEXT: &str = "I like this sentence.";
const WINDOW_TIMEOUT: Duration = Duration::from_secs(30);
const SAVE_TIMEOUT: Duration = Duration::from_secs(10);
const MAX_CHECK_ATTEMPTS: usize = 4;
const CHECK_RETRY_DELAY: Duration = Duration::from_millis(500);
const CREATE_NO_WINDOW: u32 = 0x0800_0000;
static DESKTOP_SMOKE_LOCK: Mutex<()> = Mutex::new(());
const WINDOW_MARKER_ENV: &str = "EMENDA_SMOKE_WINDOW_MARKER";
const WINDOW_PROCESS_ID_ENV: &str = "EMENDA_SMOKE_WINDOW_PROCESS_ID";
const WINDOW_PROCESS_START_TICKS_ENV: &str = "EMENDA_SMOKE_WINDOW_PROCESS_START_TICKS";
const WINDOW_PROCESS_NAME_ENV: &str = "EMENDA_SMOKE_WINDOW_PROCESS_NAME";
const NOTEPAD_FILE_ENV: &str = "EMENDA_SMOKE_NOTEPAD_FILE";
const LAUNCH_NOTEPAD_SCRIPT: &str = r#"
$ErrorActionPreference = 'Stop'
$file = $env:EMENDA_SMOKE_NOTEPAD_FILE
if (-not $file) { exit 20 }
Start-Process notepad.exe -ArgumentList @($file) -PassThru -Wait
"#;
const FIND_AND_ACTIVATE_WINDOW_SCRIPT: &str = r#"
$ErrorActionPreference = 'Stop'
$marker = $env:EMENDA_SMOKE_WINDOW_MARKER
$shell = New-Object -ComObject WScript.Shell
$match = Get-Process | Where-Object {
    $_.MainWindowTitle -and
    $_.MainWindowTitle.IndexOf($marker, [System.StringComparison]::OrdinalIgnoreCase) -ge 0
} | Select-Object -First 1
if ($null -eq $match) { exit 20 }
if (-not $shell.AppActivate([int]$match.Id)) { exit 21 }
[Console]::Out.WriteLine("$($match.Id)|$($match.StartTime.Ticks)|$($match.ProcessName)")
"#;
const PROCESS_IDENTITY_MATCHES_SCRIPT: &str = r#"
$ErrorActionPreference = 'Stop'
$processId = [int]$env:EMENDA_SMOKE_WINDOW_PROCESS_ID
$expectedStartTicks = [long]$env:EMENDA_SMOKE_WINDOW_PROCESS_START_TICKS
$expectedProcessName = $env:EMENDA_SMOKE_WINDOW_PROCESS_NAME
$process = Get-Process -Id $processId -ErrorAction SilentlyContinue
if ($null -eq $process) { exit 20 }
if ($process.StartTime.Ticks -ne $expectedStartTicks) { exit 20 }
if (-not [string]::Equals($process.ProcessName, $expectedProcessName, [System.StringComparison]::OrdinalIgnoreCase)) { exit 20 }
exit 0
"#;
const ACTIVATE_RECORDED_WINDOW_SCRIPT: &str = r#"
$ErrorActionPreference = 'Stop'
$marker = $env:EMENDA_SMOKE_WINDOW_MARKER
$processId = [int]$env:EMENDA_SMOKE_WINDOW_PROCESS_ID
$expectedStartTicks = [long]$env:EMENDA_SMOKE_WINDOW_PROCESS_START_TICKS
$expectedProcessName = $env:EMENDA_SMOKE_WINDOW_PROCESS_NAME
$process = Get-Process -Id $processId -ErrorAction SilentlyContinue
if ($null -eq $process) { exit 20 }
if ($process.StartTime.Ticks -ne $expectedStartTicks) { exit 20 }
if (-not [string]::Equals($process.ProcessName, $expectedProcessName, [System.StringComparison]::OrdinalIgnoreCase)) { exit 20 }
if (-not $process.MainWindowTitle) { exit 20 }
if ($process.MainWindowTitle.IndexOf($marker, [System.StringComparison]::OrdinalIgnoreCase) -lt 0) { exit 20 }
$shell = New-Object -ComObject WScript.Shell
if (-not $shell.AppActivate([int]$process.Id)) { exit 21 }
"#;
const TERMINATE_RECORDED_WINDOW_PROCESS_SCRIPT: &str = r#"
$ErrorActionPreference = 'Stop'
$marker = $env:EMENDA_SMOKE_WINDOW_MARKER
$processId = [int]$env:EMENDA_SMOKE_WINDOW_PROCESS_ID
$expectedStartTicks = [long]$env:EMENDA_SMOKE_WINDOW_PROCESS_START_TICKS
$expectedProcessName = $env:EMENDA_SMOKE_WINDOW_PROCESS_NAME
$process = Get-Process -Id $processId -ErrorAction SilentlyContinue
if ($null -eq $process) { exit 0 }
if ($process.StartTime.Ticks -ne $expectedStartTicks) { exit 20 }
if (-not [string]::Equals($process.ProcessName, $expectedProcessName, [System.StringComparison]::OrdinalIgnoreCase)) { exit 20 }
if ($process.MainWindowTitle -and $process.MainWindowTitle.IndexOf($marker, [System.StringComparison]::OrdinalIgnoreCase) -lt 0) { exit 20 }
Stop-Process -Id $process.Id -Force -ErrorAction Stop
"#;
const FOCUS_EDITOR_CONTROL_SCRIPT: &str = r#"
$ErrorActionPreference = 'Stop'
$marker = $env:EMENDA_SMOKE_WINDOW_MARKER
$processId = [int]$env:EMENDA_SMOKE_WINDOW_PROCESS_ID
$expectedStartTicks = [long]$env:EMENDA_SMOKE_WINDOW_PROCESS_START_TICKS
$expectedProcessName = $env:EMENDA_SMOKE_WINDOW_PROCESS_NAME
$process = Get-Process -Id $processId -ErrorAction SilentlyContinue
if ($null -eq $process) { exit 20 }
if ($process.StartTime.Ticks -ne $expectedStartTicks) { exit 20 }
if (-not [string]::Equals($process.ProcessName, $expectedProcessName, [System.StringComparison]::OrdinalIgnoreCase)) { exit 20 }
if (-not $process.MainWindowTitle) { exit 20 }
if ($process.MainWindowTitle.IndexOf($marker, [System.StringComparison]::OrdinalIgnoreCase) -lt 0) { exit 20 }

$shell = New-Object -ComObject WScript.Shell
if (-not $shell.AppActivate([int]$process.Id)) { exit 21 }
Start-Sleep -Milliseconds 100

Add-Type -AssemblyName UIAutomationClient
Add-Type -AssemblyName UIAutomationTypes
$root = [System.Windows.Automation.AutomationElement]::FromHandle([IntPtr]$process.MainWindowHandle)
if ($null -eq $root) { exit 22 }

$editCondition = [System.Windows.Automation.PropertyCondition]::new(
    [System.Windows.Automation.AutomationElement]::ControlTypeProperty,
    [System.Windows.Automation.ControlType]::Edit
)
$target = $root.FindFirst(
    [System.Windows.Automation.TreeScope]::Descendants,
    $editCondition
)
if ($null -eq $target) {
    $documentCondition = [System.Windows.Automation.PropertyCondition]::new(
        [System.Windows.Automation.AutomationElement]::ControlTypeProperty,
        [System.Windows.Automation.ControlType]::Document
    )
    $target = $root.FindFirst(
        [System.Windows.Automation.TreeScope]::Descendants,
        $documentCondition
    )
}
if ($null -eq $target) { exit 23 }
$target.SetFocus()
"#;

/// Forces the live provider through the explicitly required development
/// environment variable, independent of any credential saved by the desktop
/// application. The credential itself is never read or formatted by the test.
struct EnvironmentCredentials;

impl CredentialStore for EnvironmentCredentials {
    fn get_api_key(&self) -> Result<Option<ApiKey>, CoreError> {
        Ok(None)
    }

    fn set_api_key(&self, _api_key: &ApiKey) -> Result<(), CoreError> {
        Ok(())
    }

    fn delete_api_key(&self) -> Result<(), CoreError> {
        Ok(())
    }
}

#[derive(Clone, Copy)]
enum EditorKind {
    Notepad,
    VsCode,
}

impl EditorKind {
    const fn name(self) -> &'static str {
        match self {
            Self::Notepad => "Notepad",
            Self::VsCode => "VS Code",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct WindowProcessIdentity {
    process_id: u64,
    start_time_ticks: i64,
    process_name: String,
}

struct EditorSession {
    child: Child,
    kind: EditorKind,
    title_marker: String,
    window_process: Option<WindowProcessIdentity>,
    closed: bool,
}

impl EditorSession {
    fn launch_notepad(file: &Path) -> io::Result<Self> {
        // Start-Process uses the packaged application's ShellExecute path more
        // reliably than direct CreateProcess invocation. The file path is
        // passed as data through a dedicated environment variable; the script
        // is constant and performs no string interpolation.
        let child = hidden_powershell(LAUNCH_NOTEPAD_SCRIPT)
            .env(NOTEPAD_FILE_ENV, file.as_os_str())
            .stdout(Stdio::null())
            .spawn()?;
        Ok(Self::new(child, EditorKind::Notepad, file))
    }

    fn launch_vscode(executable: &Path, file: &Path, profile: &Path) -> io::Result<Self> {
        let extensions = profile.join("extensions");
        // The installed bin\code.cmd launcher is preferred by discovery.
        // Every argument below is fixed or points into this test's temporary
        // workspace, so Rust's Windows batch-command invocation remains scoped
        // to trusted local inputs.
        let child = Command::new(executable)
            .arg("--new-window")
            .arg("--wait")
            .arg("--disable-extensions")
            .arg("--disable-workspace-trust")
            .arg("--skip-welcome")
            // A dedicated profile prevents the smoke-test window from sharing
            // a process or lifecycle with the user's normal VS Code session.
            .arg("--user-data-dir")
            .arg(profile)
            .arg("--extensions-dir")
            .arg(extensions)
            .arg(file)
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()?;
        Ok(Self::new(child, EditorKind::VsCode, file))
    }

    fn new(child: Child, kind: EditorKind, file: &Path) -> Self {
        let title_marker = file
            .file_stem()
            .expect("the smoke-test file must have a stem")
            .to_string_lossy()
            .into_owned();
        Self {
            child,
            kind,
            title_marker,
            window_process: None,
            closed: false,
        }
    }

    fn wait_until_active(&mut self) -> Result<(), String> {
        let deadline = Instant::now() + WINDOW_TIMEOUT;
        while Instant::now() < deadline {
            if let Ok(Some(window_process)) = find_and_activate_window(&self.title_marker) {
                thread::sleep(Duration::from_millis(100));
                if get_active_window()
                    .is_ok_and(|active| title_matches(&active.title, &self.title_marker))
                {
                    // Packaged applications such as modern Notepad can return
                    // a short-lived launcher PID. Persist the identity of the
                    // process owning the unique marker-matched window instead.
                    self.window_process = Some(window_process);
                    // Let the editor finish painting/loading the file before
                    // selecting its document contents.
                    thread::sleep(Duration::from_millis(500));
                    return Ok(());
                }
            }
            thread::sleep(Duration::from_millis(150));
        }
        Err(format!(
            "the editor window containing '{}' did not become active within {:?}",
            self.title_marker, WINDOW_TIMEOUT
        ))
    }

    fn assert_still_active(&self) -> Result<(), String> {
        let active = get_active_window()
            .map_err(|()| format!("no active window was available for '{}'", self.title_marker))?;
        if title_matches(&active.title, &self.title_marker) {
            Ok(())
        } else {
            Err(format!(
                "expected the '{}' editor window to be active, but the foreground title was '{}'",
                self.title_marker, active.title
            ))
        }
    }

    fn focus_editor_control(&self) -> Result<(), String> {
        let process = self.window_process.as_ref().ok_or_else(|| {
            format!(
                "the '{}' editor window does not have a recorded process",
                self.title_marker
            )
        })?;
        match focus_marker_scoped_editor_control(process, &self.title_marker) {
            Ok(true) => {
                thread::sleep(Duration::from_millis(150));
                Ok(())
            }
            Ok(false) => Err(format!(
                "Windows UI Automation could not focus the '{}' editor control",
                self.title_marker
            )),
            Err(error) => Err(format!(
                "Windows UI Automation could not focus the '{}' editor control: {error}",
                self.title_marker
            )),
        }
    }

    fn close(&mut self) {
        if self.closed {
            return;
        }

        if self.window_process.is_none() {
            self.window_process = find_and_activate_window(&self.title_marker).ok().flatten();
        }
        if let Some(process) = self.window_process.clone() {
            let activated = activate_recorded_window(&process, &self.title_marker).unwrap_or(false);
            if activated
                && wait_for_matching_active_window(&self.title_marker, Duration::from_secs(3))
            {
                if let Ok(mut input) = Enigo::new(&EnigoSettings::default()) {
                    let _ = match self.kind {
                        // Modern Notepad closes only the active test tab. On
                        // classic Notepad this closes its dedicated window.
                        EditorKind::Notepad => control_shortcut(&mut input, Key::W),
                        // The dedicated profile owns this VS Code window.
                        EditorKind::VsCode => control_shift_shortcut(&mut input, Key::W),
                    };
                    thread::sleep(Duration::from_millis(500));
                }
            }

            if !wait_for_recorded_process_to_exit(&process, Duration::from_secs(5)) {
                // The UI action above required the unique title marker. After
                // Ctrl+W, modern Notepad may linger with an empty title, so
                // cleanup revalidates PID, process start time, and process name
                // atomically before stopping only that recorded process.
                let _ = terminate_recorded_window_process(&process, &self.title_marker);
            }
        }

        let deadline = Instant::now() + Duration::from_secs(2);
        while Instant::now() < deadline {
            match self.child.try_wait() {
                Ok(Some(_)) => {
                    self.closed = true;
                    return;
                }
                Ok(None) | Err(_) => thread::sleep(Duration::from_millis(100)),
            }
        }

        // Reap or stop only the exact launcher handle created above. No process
        // tree operation is used.
        let _ = self.child.kill();
        let _ = self.child.wait();
        self.closed = true;
    }
}

impl Drop for EditorSession {
    fn drop(&mut self) {
        self.close();
    }
}

#[test]
#[ignore = "requires a Windows desktop, Notepad, network access, and OPENROUTER_API_KEY"]
fn corrects_selected_text_in_notepad() {
    run_surface_smoke(EditorKind::Notepad);
}

#[test]
#[ignore = "requires a Windows desktop, VS Code, network access, and OPENROUTER_API_KEY"]
fn corrects_selected_text_in_vscode() {
    run_surface_smoke(EditorKind::VsCode);
}

fn run_surface_smoke(kind: EditorKind) {
    // Clipboard transport and foreground-window input are machine-global. The
    // tests own no shared application state, but serialize native interaction
    // if a caller deliberately runs both ignored cases in one test process.
    let _desktop_guard = DESKTOP_SMOKE_LOCK
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    assert!(
        env::var_os(OPENROUTER_API_KEY_ENV).is_some_and(|value| !value.is_empty()),
        "set OPENROUTER_API_KEY before running the ignored Windows desktop smoke test"
    );

    let workspace = tempfile::tempdir().expect("create smoke-test workspace");
    let settings = Arc::new(SettingsStore::new(workspace.path().join("settings.json")));
    let credentials: Arc<dyn CredentialStore> = Arc::new(EnvironmentCredentials);
    let provider: Arc<dyn InferenceProvider> = Arc::new(
        OpenRouterProvider::new(credentials).expect("initialise the live OpenRouter provider"),
    );
    let adapter: Arc<dyn TextSurfaceAdapter> =
        Arc::from(platform_adapter().expect("initialise the real Windows text-surface adapter"));
    let workflow = WorkflowController::new(adapter, provider, settings);

    let editor_name = kind.name();
    let slug = match kind {
        EditorKind::Notepad => "notepad",
        EditorKind::VsCode => "vscode",
    };
    let file = workspace
        .path()
        .join(format!("emenda-{slug}-smoke-{}.txt", std::process::id()));
    fs::write(&file, ORIGINAL_TEXT)
        .unwrap_or_else(|error| panic!("{editor_name}: seed source file: {error}"));

    let mut editor = match kind {
        EditorKind::Notepad => EditorSession::launch_notepad(&file),
        EditorKind::VsCode => {
            let vscode = find_vscode().expect("find an installed Visual Studio Code launcher");
            let profile = workspace.path().join("vscode-profile");
            EditorSession::launch_vscode(&vscode, &file, &profile)
        }
    }
    .unwrap_or_else(|error| panic!("{editor_name}: launch editor: {error}"));
    editor
        .wait_until_active()
        .unwrap_or_else(|error| panic!("{editor_name}: wait for editor window: {error}"));
    tauri::async_runtime::block_on(exercise_surface(&workflow, &mut editor, &file));
    editor.close();
}

async fn exercise_surface(workflow: &WorkflowController, editor: &mut EditorSession, file: &Path) {
    let editor_name = editor.kind.name();
    let mut input = Enigo::new(&EnigoSettings::default()).expect("initialise Windows input");
    let state = check_with_bounded_retries(workflow, editor, &mut input).await;
    let correction_index = match &state {
        WorkflowState::Suggestions {
            source_text,
            source_application,
            corrections,
            ..
        } => {
            assert_eq!(
                source_text, ORIGINAL_TEXT,
                "{editor_name}: captured source text did not match the seeded selection"
            );
            assert!(title_matches(
                &source_application.window_title,
                &editor.title_marker
            ), "{editor_name}: capture came from an unexpected source window");
            corrections
                .iter()
                .position(|correction| {
                    correction.original() == "liek" && correction.replacement() == "like"
                })
                .unwrap_or_else(|| {
                    panic!(
                        "{editor_name}: OpenRouter must return an exact 'liek' -> 'like' suggestion"
                    )
                })
        }
        other => panic!(
            "{editor_name}: expected validated OpenRouter suggestions after capture, received {other:?}"
        ),
    };

    let accepted = workflow.apply_correction(correction_index, |_| {}).await;
    let applied = match accepted {
        clean @ WorkflowState::Clean { applied: true, .. } => clean,
        WorkflowState::Suggestions {
            accepted_count: 1, ..
        } => workflow.finish_or_dismiss(|_| {}).await,
        other => panic!(
            "{editor_name}: accepting the typo correction produced an unexpected state: {other:?}"
        ),
    };
    match applied {
        WorkflowState::Clean {
            working_text,
            applied: true,
            ..
        } => assert_eq!(
            working_text, CORRECTED_TEXT,
            "{editor_name}: applied working text was unexpected"
        ),
        other => {
            panic!("{editor_name}: the accepted subset was not applied to the source: {other:?}")
        }
    }

    control_shortcut(&mut input, Key::S)
        .unwrap_or_else(|error| panic!("{editor_name}: save corrected contents: {error}"));
    wait_for_file_contents(file, CORRECTED_TEXT).unwrap_or_else(|error| {
        panic!("{editor_name}: the editor must persist Emenda's replacement: {error}")
    });
}

async fn check_with_bounded_retries(
    workflow: &WorkflowController,
    editor: &EditorSession,
    input: &mut Enigo,
) -> WorkflowState {
    let editor_name = editor.kind.name();
    let mut vscode_accessibility_enabled = false;

    for attempt in 1..=MAX_CHECK_ATTEMPTS {
        prepare_selection(editor, input, &mut vscode_accessibility_enabled, attempt);
        let state = workflow.check_current_selection(|_| {}).await;
        match &state {
            WorkflowState::Suggestions { .. } => return state,
            WorkflowState::Error { .. } if attempt < MAX_CHECK_ATTEMPTS => {
                thread::sleep(CHECK_RETRY_DELAY);
            }
            WorkflowState::Error { .. } => {
                panic!(
                    "{editor_name}: OpenRouter did not produce suggestions after {MAX_CHECK_ATTEMPTS} attempts; last state: {state:?}"
                );
            }
            _ => {
                panic!(
                    "{editor_name}: expected validated OpenRouter suggestions after capture attempt {attempt}; last state: {state:?}"
                );
            }
        }
    }

    unreachable!("the bounded correction loop always returns or reports its last typed state")
}

fn prepare_selection(
    editor: &EditorSession,
    input: &mut Enigo,
    vscode_accessibility_enabled: &mut bool,
    attempt: usize,
) {
    let editor_name = editor.kind.name();
    editor.focus_editor_control().unwrap_or_else(|error| {
        panic!("{editor_name}: focus the editor control before attempt {attempt}: {error}")
    });

    if matches!(editor.kind, EditorKind::VsCode) {
        if !*vscode_accessibility_enabled {
            // Monaco exposes a stable editable accessibility surface only
            // while screen-reader mode is enabled. Do not toggle it on every
            // retry because the shortcut would turn the mode back off.
            shift_alt_shortcut(input, Key::F1)
                .expect("VS Code: enable screen-reader accessibility mode");
            thread::sleep(Duration::from_millis(750));
            *vscode_accessibility_enabled = true;
        }
        // Every attempt repeats the marker/PID-scoped UIA focus after any
        // accessibility preparation so selection lands in Monaco itself.
        editor.focus_editor_control().unwrap_or_else(|error| {
            panic!("VS Code: refocus Monaco before attempt {attempt}: {error}")
        });
    }

    editor.assert_still_active().unwrap_or_else(|error| {
        panic!(
            "{editor_name}: editor must own the foreground before selection attempt {attempt}: {error}"
        )
    });
    control_shortcut(input, Key::A).unwrap_or_else(|error| {
        panic!("{editor_name}: select source text for attempt {attempt}: {error}")
    });
    thread::sleep(Duration::from_millis(250));
}

fn control_shortcut(input: &mut Enigo, key: Key) -> Result<(), String> {
    chord(input, &[Key::Control], key)
}

fn control_shift_shortcut(input: &mut Enigo, key: Key) -> Result<(), String> {
    chord(input, &[Key::Control, Key::Shift], key)
}

fn shift_alt_shortcut(input: &mut Enigo, key: Key) -> Result<(), String> {
    chord(input, &[Key::Shift, Key::Alt], key)
}

fn chord(input: &mut Enigo, modifiers: &[Key], key: Key) -> Result<(), String> {
    let mut pressed = Vec::with_capacity(modifiers.len());
    for modifier in modifiers {
        if let Err(error) = input.key(*modifier, Press) {
            for pressed_modifier in pressed.into_iter().rev() {
                let _ = input.key(pressed_modifier, Release);
            }
            return Err(error.to_string());
        }
        pressed.push(*modifier);
    }

    let click_result = input.key(key, Click).map_err(|error| error.to_string());
    let mut release_result = Ok(());
    for modifier in pressed.into_iter().rev() {
        if let Err(error) = input.key(modifier, Release) {
            release_result = Err(error.to_string());
        }
    }
    click_result.and(release_result)
}

fn wait_for_file_contents(file: &Path, expected: &str) -> Result<(), String> {
    let deadline = Instant::now() + SAVE_TIMEOUT;
    let mut last_observed = String::new();
    while Instant::now() < deadline {
        if let Ok(contents) = fs::read_to_string(file) {
            if contents == expected {
                return Ok(());
            }
            last_observed = contents;
        }
        thread::sleep(Duration::from_millis(100));
    }
    Err(format!(
        "the file was not saved as expected; last observed contents were {last_observed:?}"
    ))
}

fn wait_for_matching_active_window(marker: &str, timeout: Duration) -> bool {
    let deadline = Instant::now() + timeout;
    while Instant::now() < deadline {
        if get_active_window().is_ok_and(|active| title_matches(&active.title, marker)) {
            return true;
        }
        thread::sleep(Duration::from_millis(100));
    }
    false
}

fn title_matches(title: &str, marker: &str) -> bool {
    title.to_lowercase().contains(&marker.to_lowercase())
}

fn find_and_activate_window(marker: &str) -> io::Result<Option<WindowProcessIdentity>> {
    let output = hidden_powershell(FIND_AND_ACTIVATE_WINDOW_SCRIPT)
        .env(WINDOW_MARKER_ENV, marker)
        .stdout(Stdio::piped())
        .output()?;
    if !output.status.success() {
        return Ok(None);
    }

    let encoded = String::from_utf8(output.stdout)
        .map_err(|error| io::Error::new(io::ErrorKind::InvalidData, error))?;
    let mut fields = encoded.trim().splitn(3, '|');
    let process_id = parse_identity_field::<u64>(&mut fields, "process ID")?;
    let start_time_ticks = parse_identity_field::<i64>(&mut fields, "process start time")?;
    let process_name = fields
        .next()
        .filter(|value| !value.is_empty())
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "missing process name"))?
        .to_owned();
    Ok(Some(WindowProcessIdentity {
        process_id,
        start_time_ticks,
        process_name,
    }))
}

fn parse_identity_field<'a, T>(
    fields: &mut impl Iterator<Item = &'a str>,
    field_name: &str,
) -> io::Result<T>
where
    T: std::str::FromStr,
    T::Err: std::error::Error + Send + Sync + 'static,
{
    fields
        .next()
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, format!("missing {field_name}")))?
        .parse::<T>()
        .map_err(|error| io::Error::new(io::ErrorKind::InvalidData, error))
}

fn recorded_process_identity_matches(process: &WindowProcessIdentity) -> io::Result<bool> {
    run_identity_script(PROCESS_IDENTITY_MATCHES_SCRIPT, process, None)
}

fn wait_for_recorded_process_to_exit(process: &WindowProcessIdentity, timeout: Duration) -> bool {
    let deadline = Instant::now() + timeout;
    while Instant::now() < deadline {
        match recorded_process_identity_matches(process) {
            Ok(false) => return true,
            Ok(true) | Err(_) => thread::sleep(Duration::from_millis(200)),
        }
    }
    false
}

fn activate_recorded_window(process: &WindowProcessIdentity, marker: &str) -> io::Result<bool> {
    run_identity_script(ACTIVATE_RECORDED_WINDOW_SCRIPT, process, Some(marker))
}

fn terminate_recorded_window_process(
    process: &WindowProcessIdentity,
    marker: &str,
) -> io::Result<bool> {
    run_identity_script(
        TERMINATE_RECORDED_WINDOW_PROCESS_SCRIPT,
        process,
        Some(marker),
    )
}

fn focus_marker_scoped_editor_control(
    process: &WindowProcessIdentity,
    marker: &str,
) -> io::Result<bool> {
    run_identity_script(FOCUS_EDITOR_CONTROL_SCRIPT, process, Some(marker))
}

fn run_identity_script(
    script: &str,
    process: &WindowProcessIdentity,
    marker: Option<&str>,
) -> io::Result<bool> {
    let mut command = hidden_powershell(script);
    command
        .env(WINDOW_PROCESS_ID_ENV, process.process_id.to_string())
        .env(
            WINDOW_PROCESS_START_TICKS_ENV,
            process.start_time_ticks.to_string(),
        )
        .env(WINDOW_PROCESS_NAME_ENV, &process.process_name)
        .stdout(Stdio::null());
    if let Some(marker) = marker {
        command.env(WINDOW_MARKER_ENV, marker);
    }
    command.status().map(|status| status.success())
}

fn hidden_powershell(script: &str) -> Command {
    use std::os::windows::process::CommandExt;
    let mut command = Command::new("powershell.exe");
    command
        .args([
            "-NoLogo",
            "-NoProfile",
            "-NonInteractive",
            "-WindowStyle",
            "Hidden",
            "-Command",
            script,
        ])
        .creation_flags(CREATE_NO_WINDOW)
        .stdin(Stdio::null())
        .stderr(Stdio::null());
    command
}

fn find_vscode() -> Option<PathBuf> {
    let mut command_launchers = Vec::new();
    let mut executable_fallbacks = Vec::new();
    if let Some(local_app_data) = env::var_os("LOCALAPPDATA") {
        let install_root = PathBuf::from(local_app_data)
            .join("Programs")
            .join("Microsoft VS Code");
        command_launchers.push(install_root.join("bin").join("code.cmd"));
        executable_fallbacks.push(install_root.join("Code.exe"));
    }
    for variable in ["ProgramFiles", "ProgramFiles(x86)"] {
        if let Some(program_files) = env::var_os(variable) {
            let install_root = PathBuf::from(program_files).join("Microsoft VS Code");
            command_launchers.push(install_root.join("bin").join("code.cmd"));
            executable_fallbacks.push(install_root.join("Code.exe"));
        }
    }
    if let Some(path) = env::var_os("PATH") {
        for entry in env::split_paths(&path) {
            command_launchers.push(entry.join("code.cmd"));
            executable_fallbacks.push(entry.join("Code.exe"));
            if entry
                .file_name()
                .is_some_and(|name| name.eq_ignore_ascii_case("bin"))
            {
                if let Some(install_root) = entry.parent() {
                    command_launchers.push(entry.join("code.cmd"));
                    executable_fallbacks.push(install_root.join("Code.exe"));
                }
            }
        }
    }

    command_launchers
        .into_iter()
        .chain(executable_fallbacks)
        .find(|candidate| candidate.is_file())
}
