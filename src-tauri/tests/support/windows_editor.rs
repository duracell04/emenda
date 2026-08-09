#![cfg(target_os = "windows")]

use active_win_pos_rs::get_active_window;
use enigo::{
    Direction::{Click, Press, Release},
    Enigo, Key, Keyboard, Settings as EnigoSettings,
};
use std::{
    collections::HashSet,
    env, fs, io,
    path::{Path, PathBuf},
    process::{Child, Command, Stdio},
    sync::Mutex,
    thread,
    time::{Duration, Instant},
};

const WINDOW_TIMEOUT: Duration = Duration::from_secs(30);
const REFOCUS_TIMEOUT: Duration = Duration::from_secs(30);
const SAVE_TIMEOUT: Duration = Duration::from_secs(10);
const CREATE_NO_WINDOW: u32 = 0x0800_0000;
const WINDOW_MARKER_ENV: &str = "EMENDA_SMOKE_WINDOW_MARKER";
const WINDOW_PROCESS_ID_ENV: &str = "EMENDA_SMOKE_WINDOW_PROCESS_ID";
const WINDOW_PROCESS_START_TICKS_ENV: &str = "EMENDA_SMOKE_WINDOW_PROCESS_START_TICKS";
const WINDOW_PROCESS_NAME_ENV: &str = "EMENDA_SMOKE_WINDOW_PROCESS_NAME";
const NOTEPAD_FILE_ENV: &str = "EMENDA_SMOKE_NOTEPAD_FILE";
const EDITOR_PROCESS_NAME_ENV: &str = "EMENDA_SMOKE_EDITOR_PROCESS_NAME";

#[allow(dead_code)] // Used by integration tests; the JSONL fixture includes this shared module too.
pub(crate) static DESKTOP_SMOKE_LOCK: Mutex<()> = Mutex::new(());

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
$windowMatches = @(Get-Process | Where-Object {
    $_.MainWindowTitle -and
    $_.MainWindowTitle.IndexOf($marker, [System.StringComparison]::OrdinalIgnoreCase) -ge 0
})
if ($windowMatches.Count -eq 0) { exit 20 }
if ($windowMatches.Count -ne 1) { exit 22 }
$match = $windowMatches[0]
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
const LIST_EDITOR_PROCESS_IDS_SCRIPT: &str = r#"
$ErrorActionPreference = 'Stop'
$name = $env:EMENDA_SMOKE_EDITOR_PROCESS_NAME
if (-not $name) { exit 20 }
Get-Process -Name $name -ErrorAction SilentlyContinue |
    ForEach-Object { [Console]::Out.WriteLine($_.Id) }
"#;
const VERIFY_INTERACTIVE_DESKTOP_SCRIPT: &str = r#"
$ErrorActionPreference = 'Stop'
Add-Type @'
using System;
using System.Runtime.InteropServices;
public static class EmendaInputDesktop {
    [DllImport("user32.dll", SetLastError = true)]
    public static extern IntPtr OpenInputDesktop(uint flags, bool inherit, uint access);
    [DllImport("user32.dll", SetLastError = true)]
    public static extern bool SwitchDesktop(IntPtr desktop);
    [DllImport("user32.dll", SetLastError = true)]
    public static extern bool CloseDesktop(IntPtr desktop);
}
'@
$desktop = [EmendaInputDesktop]::OpenInputDesktop(0, $false, 0x0100)
if ($desktop -eq [IntPtr]::Zero) { exit 20 }
try {
    if (-not [EmendaInputDesktop]::SwitchDesktop($desktop)) { exit 21 }
} finally {
    [void][EmendaInputDesktop]::CloseDesktop($desktop)
}
"#;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum EditorKind {
    Notepad,
    VsCode,
}

impl EditorKind {
    pub(crate) const fn name(self) -> &'static str {
        match self {
            Self::Notepad => "Notepad",
            Self::VsCode => "VS Code",
        }
    }

    pub(crate) const fn slug(self) -> &'static str {
        match self {
            Self::Notepad => "notepad",
            Self::VsCode => "vscode",
        }
    }

    const fn process_name(self) -> &'static str {
        match self {
            Self::Notepad => "Notepad",
            Self::VsCode => "Code",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct WindowProcessIdentity {
    process_id: u64,
    start_time_ticks: i64,
    process_name: String,
}

/// Owns one marker-scoped editor window created by a Windows desktop test.
///
/// Cleanup never searches by executable name alone. It revalidates the exact
/// PID, process start time, process name, and (while present) unique title
/// marker before terminating the process created for the test.
pub(crate) struct EditorSession {
    child: Child,
    kind: EditorKind,
    title_marker: String,
    baseline_process_ids: HashSet<u64>,
    window_process: Option<WindowProcessIdentity>,
    closed: bool,
}

impl EditorSession {
    pub(crate) fn launch(kind: EditorKind, file: &Path, workspace: &Path) -> io::Result<Self> {
        let baseline_process_ids = list_editor_process_ids(kind)?;
        if matches!(kind, EditorKind::Notepad) && !baseline_process_ids.is_empty() {
            return Err(io::Error::new(
                io::ErrorKind::AlreadyExists,
                "Notepad was already running; refusing a test that could affect a user-owned window",
            ));
        }
        match kind {
            EditorKind::Notepad => Self::launch_notepad(file, baseline_process_ids),
            EditorKind::VsCode => {
                let executable = find_vscode().ok_or_else(|| {
                    io::Error::new(
                        io::ErrorKind::NotFound,
                        "no installed Visual Studio Code launcher was found",
                    )
                })?;
                Self::launch_vscode(
                    &executable,
                    file,
                    &workspace.join("vscode-profile"),
                    baseline_process_ids,
                )
            }
        }
    }

    fn launch_notepad(file: &Path, baseline_process_ids: HashSet<u64>) -> io::Result<Self> {
        // Start-Process uses the packaged application's ShellExecute path more
        // reliably than direct CreateProcess invocation. The path is passed as
        // data through an environment variable; the script is constant.
        let child = hidden_powershell(LAUNCH_NOTEPAD_SCRIPT)
            .env(NOTEPAD_FILE_ENV, file.as_os_str())
            .stdout(Stdio::null())
            .spawn()?;
        Ok(Self::new(
            child,
            EditorKind::Notepad,
            file,
            baseline_process_ids,
        ))
    }

    fn launch_vscode(
        executable: &Path,
        file: &Path,
        profile: &Path,
        baseline_process_ids: HashSet<u64>,
    ) -> io::Result<Self> {
        let extensions = profile.join("extensions");
        let child = Command::new(executable)
            .arg("--new-window")
            .arg("--wait")
            .arg("--disable-extensions")
            .arg("--disable-workspace-trust")
            .arg("--skip-welcome")
            .arg("--user-data-dir")
            .arg(profile)
            .arg("--extensions-dir")
            .arg(extensions)
            .arg(file)
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()?;
        Ok(Self::new(
            child,
            EditorKind::VsCode,
            file,
            baseline_process_ids,
        ))
    }

    fn new(
        child: Child,
        kind: EditorKind,
        file: &Path,
        baseline_process_ids: HashSet<u64>,
    ) -> Self {
        let title_marker = file
            .file_stem()
            .expect("the desktop-test file must have a stem")
            .to_string_lossy()
            .into_owned();
        Self {
            child,
            kind,
            title_marker,
            baseline_process_ids,
            window_process: None,
            closed: false,
        }
    }

    #[allow(dead_code)] // Used by integration tests, not by the JSONL fixture binary.
    pub(crate) const fn kind(&self) -> EditorKind {
        self.kind
    }

    pub(crate) fn title_marker(&self) -> &str {
        &self.title_marker
    }

    pub(crate) fn wait_until_active(&mut self) -> Result<(), String> {
        let deadline = Instant::now() + WINDOW_TIMEOUT;
        while Instant::now() < deadline {
            match find_and_activate_window(&self.title_marker) {
                Ok(Some(window_process)) => {
                    if !window_process
                        .process_name
                        .eq_ignore_ascii_case(self.kind.process_name())
                    {
                        return Err(format!(
                            "the marker-matched process was '{}', not the requested '{}'",
                            window_process.process_name,
                            self.kind.process_name()
                        ));
                    }
                    if self
                        .baseline_process_ids
                        .contains(&window_process.process_id)
                    {
                        return Err(format!(
                            "the marker-matched {} window belongs to a process that predated this test",
                            self.kind.name()
                        ));
                    }
                    thread::sleep(Duration::from_millis(100));
                    if get_active_window().is_ok_and(|active| {
                        active.process_id == window_process.process_id
                            && title_matches(&active.title, &self.title_marker)
                            && recorded_process_identity_matches(&window_process).unwrap_or(false)
                    }) {
                        // Modern Notepad can return a short-lived launcher PID.
                        // Record the owner of the marker-matched window instead.
                        self.window_process = Some(window_process);
                        thread::sleep(Duration::from_millis(500));
                        return Ok(());
                    }
                }
                Ok(None) => {}
                Err(error) => {
                    return Err(format!(
                        "could not uniquely identify the marker-scoped {} window: {error}",
                        self.kind.name()
                    ));
                }
            }
            thread::sleep(Duration::from_millis(150));
        }
        Err(format!(
            "the editor window containing '{}' did not become active within {:?}",
            self.title_marker, WINDOW_TIMEOUT
        ))
    }

    /// Focuses the owned editor and selects its document exactly once.
    pub(crate) fn select_all(&self) -> Result<(), String> {
        let mut input = Enigo::new(&EnigoSettings::default())
            .map_err(|error| format!("initialise Windows input: {error}"))?;
        self.focus_editor_control()?;

        if matches!(self.kind, EditorKind::VsCode) {
            // This dedicated profile begins with screen-reader mode disabled.
            // Enable it once so Monaco exposes a stable editable UIA surface.
            shift_alt_shortcut(&mut input, Key::F1)
                .map_err(|error| format!("enable VS Code screen-reader mode: {error}"))?;
            thread::sleep(Duration::from_millis(750));
            self.focus_editor_control()?;
        }

        self.assert_still_active()?;
        control_shortcut(&mut input, Key::A)
            .map_err(|error| format!("select source text: {error}"))?;
        thread::sleep(Duration::from_millis(250));
        Ok(())
    }

    /// Selects the owned source text and invokes Emenda's global shortcut once.
    #[allow(dead_code)] // Used by the JSONL fixture, not by the integration-test crate.
    pub(crate) fn trigger_emenda_hotkey(&self) -> Result<(), String> {
        self.select_all()?;
        self.assert_still_active()?;
        let mut input = Enigo::new(&EnigoSettings::default())
            .map_err(|error| format!("initialise Windows input: {error}"))?;
        chord(&mut input, &[Key::Control, Key::Alt], Key::Unicode('e'))
            .map_err(|error| format!("invoke Emenda shortcut: {error}"))?;
        thread::sleep(Duration::from_millis(250));
        Ok(())
    }

    pub(crate) fn save_and_wait(&self, file: &Path, expected: &str) -> Result<(), String> {
        self.assert_still_active()?;
        let mut input = Enigo::new(&EnigoSettings::default())
            .map_err(|error| format!("initialise Windows input: {error}"))?;
        control_shortcut(&mut input, Key::S).map_err(|error| format!("save contents: {error}"))?;
        wait_for_file_contents(file, expected)
    }

    /// Waits for Emenda itself to return focus to the exact owned source.
    /// This only observes foreground state; it never activates a window.
    #[allow(dead_code)] // Used by the JSONL fixture, not by the integration-test crate.
    pub(crate) fn wait_until_refocused(&self) -> Result<(), String> {
        let process = self.window_process.as_ref().ok_or_else(|| {
            format!(
                "the '{}' editor window does not have a recorded process",
                self.title_marker
            )
        })?;
        let deadline = Instant::now() + REFOCUS_TIMEOUT;
        let mut last_title = String::new();
        while Instant::now() < deadline {
            if let Ok(active) = get_active_window() {
                last_title = active.title.clone();
                if active.process_id == process.process_id
                    && title_matches(&active.title, &self.title_marker)
                    && recorded_process_identity_matches(process).unwrap_or(false)
                {
                    return Ok(());
                }
            }
            thread::sleep(Duration::from_millis(100));
        }
        Err(format!(
            "Emenda did not return focus to the exact '{}' source within {:?}; last foreground title was {:?}",
            self.title_marker, REFOCUS_TIMEOUT, last_title
        ))
    }

    fn assert_still_active(&self) -> Result<(), String> {
        let process = self.window_process.as_ref().ok_or_else(|| {
            format!(
                "the '{}' editor window does not have a recorded process",
                self.title_marker
            )
        })?;
        let active = get_active_window()
            .map_err(|()| format!("no active window was available for '{}'", self.title_marker))?;
        if active.process_id == process.process_id
            && title_matches(&active.title, &self.title_marker)
            && recorded_process_identity_matches(process).unwrap_or(false)
        {
            Ok(())
        } else {
            Err(format!(
                "expected the exact '{}' editor process {} to be active, but foreground process {} had title '{}'",
                self.title_marker, process.process_id, active.process_id, active.title
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

    pub(crate) fn close_checked(&mut self) -> Result<(), String> {
        if self.closed {
            return Ok(());
        }

        let mut failures = Vec::new();

        if self.window_process.is_none() {
            match find_and_activate_window(&self.title_marker) {
                Ok(Some(process))
                    if !self.baseline_process_ids.contains(&process.process_id)
                        && process
                            .process_name
                            .eq_ignore_ascii_case(self.kind.process_name()) =>
                {
                    self.window_process = Some(process);
                }
                Ok(Some(process)) => failures.push(format!(
                    "cleanup found marker process {} named '{}', which is not the exact owned {} process",
                    process.process_id,
                    process.process_name,
                    self.kind.name()
                )),
                Ok(None) => {}
                Err(error) => failures.push(format!(
                    "cleanup could not uniquely identify the marker-scoped {} window: {error}",
                    self.kind.name()
                )),
            }
        }
        if let Some(process) = self.window_process.clone() {
            let activated = activate_recorded_window(&process, &self.title_marker).unwrap_or(false);
            if activated
                && wait_for_matching_active_window(
                    &process,
                    &self.title_marker,
                    Duration::from_secs(3),
                )
            {
                if let Ok(mut input) = Enigo::new(&EnigoSettings::default()) {
                    let _ = match self.kind {
                        // Modern Notepad closes the active test tab; classic
                        // Notepad closes its dedicated window.
                        EditorKind::Notepad => control_shortcut(&mut input, Key::W),
                        // The dedicated profile owns this VS Code window.
                        EditorKind::VsCode => control_shift_shortcut(&mut input, Key::W),
                    };
                    thread::sleep(Duration::from_millis(500));
                }
            }

            if !wait_for_recorded_process_to_exit(&process, Duration::from_secs(5)) {
                // After Ctrl+W, modern Notepad may linger without its title.
                // Revalidate immutable process identity before stopping it.
                match terminate_recorded_window_process(&process, &self.title_marker) {
                    Ok(true) => {}
                    Ok(false) => failures.push(format!(
                        "Windows refused to terminate the exact owned {} process",
                        self.kind.name()
                    )),
                    Err(error) => failures.push(format!(
                        "could not terminate the exact owned {} process: {error}",
                        self.kind.name()
                    )),
                }
                if !wait_for_recorded_process_to_exit(&process, Duration::from_secs(5)) {
                    failures.push(format!(
                        "the exact owned {} process remained alive after cleanup",
                        self.kind.name()
                    ));
                }
            }
        }

        let deadline = Instant::now() + Duration::from_secs(2);
        while Instant::now() < deadline {
            match self.child.try_wait() {
                Ok(Some(_)) => {
                    self.closed = true;
                    return if failures.is_empty() {
                        Ok(())
                    } else {
                        Err(failures.join("; "))
                    };
                }
                Ok(None) => thread::sleep(Duration::from_millis(100)),
                Err(error) => {
                    failures.push(format!(
                        "could not inspect the owned editor launcher: {error}"
                    ));
                    break;
                }
            }
        }

        // Reap or stop only the exact launcher handle created above.
        if let Err(error) = self.child.kill() {
            failures.push(format!(
                "could not stop the exact editor launcher handle: {error}"
            ));
        }
        match self.child.wait() {
            Ok(_) => self.closed = failures.is_empty(),
            Err(error) => failures.push(format!("could not reap the editor launcher: {error}")),
        }
        if failures.is_empty() {
            self.closed = true;
            Ok(())
        } else {
            Err(failures.join("; "))
        }
    }

    pub(crate) fn close(&mut self) {
        let _ = self.close_checked();
    }
}

impl Drop for EditorSession {
    fn drop(&mut self) {
        self.close();
    }
}

pub(crate) fn title_matches(title: &str, marker: &str) -> bool {
    title.to_lowercase().contains(&marker.to_lowercase())
}

#[allow(dead_code)] // Used by the JSONL fixture, not by the integration-test crate.
pub(crate) fn ensure_interactive_desktop() -> Result<(), String> {
    let status = hidden_powershell(VERIFY_INTERACTIVE_DESKTOP_SCRIPT)
        .stdout(Stdio::null())
        .status()
        .map_err(|error| format!("inspect the Windows input desktop: {error}"))?;
    if status.success() {
        Ok(())
    } else {
        Err("Windows is locked or no interactive input desktop is available".to_owned())
    }
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

fn wait_for_matching_active_window(
    process: &WindowProcessIdentity,
    marker: &str,
    timeout: Duration,
) -> bool {
    let deadline = Instant::now() + timeout;
    while Instant::now() < deadline {
        if get_active_window().is_ok_and(|active| {
            active.process_id == process.process_id
                && title_matches(&active.title, marker)
                && recorded_process_identity_matches(process).unwrap_or(false)
        }) {
            return true;
        }
        thread::sleep(Duration::from_millis(100));
    }
    false
}

fn find_and_activate_window(marker: &str) -> io::Result<Option<WindowProcessIdentity>> {
    let output = hidden_powershell(FIND_AND_ACTIVATE_WINDOW_SCRIPT)
        .env(WINDOW_MARKER_ENV, marker)
        .stdout(Stdio::piped())
        .output()?;
    if !output.status.success() {
        return match output.status.code() {
            Some(20) => Ok(None),
            Some(22) => Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "more than one window matched the unique fixture marker",
            )),
            Some(code) => Err(io::Error::other(format!(
                "marker window activation failed with exit code {code}"
            ))),
            None => Err(io::Error::other(
                "marker window activation was terminated without an exit code",
            )),
        };
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

fn list_editor_process_ids(kind: EditorKind) -> io::Result<HashSet<u64>> {
    let process_name = match kind {
        EditorKind::Notepad => "notepad",
        EditorKind::VsCode => "Code",
    };
    let output = hidden_powershell(LIST_EDITOR_PROCESS_IDS_SCRIPT)
        .env(EDITOR_PROCESS_NAME_ENV, process_name)
        .stdout(Stdio::piped())
        .output()?;
    if !output.status.success() {
        return Err(io::Error::other(format!(
            "could not enumerate pre-existing {} processes",
            kind.name()
        )));
    }
    let encoded = String::from_utf8(output.stdout)
        .map_err(|error| io::Error::new(io::ErrorKind::InvalidData, error))?;
    encoded
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            line.trim()
                .parse::<u64>()
                .map_err(|error| io::Error::new(io::ErrorKind::InvalidData, error))
        })
        .collect()
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
