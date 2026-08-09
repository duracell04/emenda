#![forbid(unsafe_code)]

#[cfg(not(target_os = "windows"))]
fn main() {
    eprintln!("emenda-desktop-e2e-fixture is supported only on Windows");
    std::process::exit(2);
}

#[cfg(target_os = "windows")]
#[path = "../../tests/support/windows_editor.rs"]
mod windows_editor;

#[cfg(target_os = "windows")]
mod windows {
    use super::windows_editor::{ensure_interactive_desktop, EditorKind, EditorSession};
    use serde::{Deserialize, Serialize};
    use serde_json::{json, Value};
    use std::{
        fs,
        io::{self, BufRead, BufReader, BufWriter, Write},
        path::PathBuf,
    };
    use tempfile::TempDir;

    const PROTOCOL_VERSION: u8 = 1;
    const MAX_JSONL_BYTES: usize = 64 * 1024;
    const MAX_TEXT_BYTES: usize = 16 * 1024;

    #[derive(Debug, Deserialize)]
    #[serde(tag = "command", rename_all = "camelCase", deny_unknown_fields)]
    enum Request {
        Hello {
            id: u64,
            version: u8,
        },
        Launch {
            id: u64,
            version: u8,
            editor: EditorName,
            #[serde(rename = "originalText")]
            original_text: String,
            #[serde(rename = "expectedText")]
            expected_text: String,
        },
        TriggerHotkey {
            id: u64,
            version: u8,
        },
        VerifyAndSave {
            id: u64,
            version: u8,
        },
        Shutdown {
            id: u64,
            version: u8,
        },
    }

    impl Request {
        const fn id(&self) -> u64 {
            match self {
                Self::Hello { id, .. }
                | Self::Launch { id, .. }
                | Self::TriggerHotkey { id, .. }
                | Self::VerifyAndSave { id, .. }
                | Self::Shutdown { id, .. } => *id,
            }
        }

        const fn version(&self) -> u8 {
            match self {
                Self::Hello { version, .. }
                | Self::Launch { version, .. }
                | Self::TriggerHotkey { version, .. }
                | Self::VerifyAndSave { version, .. }
                | Self::Shutdown { version, .. } => *version,
            }
        }
    }

    #[derive(Clone, Copy, Debug, Deserialize)]
    #[serde(rename_all = "camelCase")]
    enum EditorName {
        Notepad,
        Vscode,
    }

    impl From<EditorName> for EditorKind {
        fn from(value: EditorName) -> Self {
            match value {
                EditorName::Notepad => Self::Notepad,
                EditorName::Vscode => Self::VsCode,
            }
        }
    }

    #[derive(Serialize)]
    #[serde(rename_all = "camelCase")]
    struct Response<'a> {
        id: Option<u64>,
        ok: bool,
        #[serde(skip_serializing_if = "Option::is_none")]
        result: Option<Value>,
        #[serde(skip_serializing_if = "Option::is_none")]
        error: Option<ProtocolError<'a>>,
    }

    #[derive(Serialize)]
    struct ProtocolError<'a> {
        code: &'a str,
        message: String,
    }

    struct FixtureError {
        code: &'static str,
        message: String,
    }

    impl FixtureError {
        fn new(code: &'static str, message: impl Into<String>) -> Self {
            Self {
                code,
                message: message.into(),
            }
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    enum Phase {
        AwaitingHello,
        Ready,
        Launched,
        Triggered,
        Verified,
    }

    struct FixtureSession {
        editor: EditorSession,
        workspace: TempDir,
        file: PathBuf,
        expected_text: String,
    }

    struct Server {
        phase: Phase,
        session: Option<FixtureSession>,
    }

    enum CommandResult {
        Continue(Value),
        Stop(Value),
    }

    impl Server {
        fn new() -> Self {
            Self {
                phase: Phase::AwaitingHello,
                session: None,
            }
        }

        fn handle(&mut self, request: Request) -> Result<CommandResult, FixtureError> {
            if request.version() != PROTOCOL_VERSION {
                return Err(FixtureError::new(
                    "unsupported_protocol",
                    format!(
                        "protocol version {} is unsupported; expected {PROTOCOL_VERSION}",
                        request.version()
                    ),
                ));
            }

            match request {
                Request::Hello { .. } => {
                    self.require_phase(Phase::AwaitingHello, "hello")?;
                    ensure_interactive_desktop()
                        .map_err(|message| FixtureError::new("desktop_unavailable", message))?;
                    self.phase = Phase::Ready;
                    Ok(CommandResult::Continue(json!({
                        "protocolVersion": PROTOCOL_VERSION,
                        "platform": "windows",
                        "phase": format!("{:?}", self.phase).to_lowercase()
                    })))
                }
                Request::Launch {
                    id,
                    editor,
                    original_text,
                    expected_text,
                    ..
                } => {
                    self.require_phase(Phase::Ready, "launch")?;
                    ensure_interactive_desktop()
                        .map_err(|message| FixtureError::new("desktop_unavailable", message))?;
                    validate_text("originalText", &original_text)?;
                    validate_text("expectedText", &expected_text)?;

                    let kind = EditorKind::from(editor);
                    let workspace = tempfile::Builder::new()
                        .prefix("emenda-desktop-e2e-")
                        .tempdir()
                        .map_err(|error| {
                            FixtureError::new("fixture_io", format!("create workspace: {error}"))
                        })?;
                    let file = workspace.path().join(format!(
                        "emenda-e2e-{}-{id}-{}.txt",
                        std::process::id(),
                        kind.slug()
                    ));
                    fs::write(&file, &original_text).map_err(|error| {
                        FixtureError::new("fixture_io", format!("write source fixture: {error}"))
                    })?;
                    let mut owned_editor = EditorSession::launch(kind, &file, workspace.path())
                        .map_err(|error| {
                            FixtureError::new(
                                "editor_launch",
                                format!("launch {}: {error}", kind.name()),
                            )
                        })?;
                    owned_editor
                        .wait_until_active()
                        .map_err(|message| FixtureError::new("editor_activation", message))?;
                    let title_marker = owned_editor.title_marker().to_owned();
                    self.session = Some(FixtureSession {
                        editor: owned_editor,
                        workspace,
                        file,
                        expected_text,
                    });
                    self.phase = Phase::Launched;
                    Ok(CommandResult::Continue(json!({
                        "editor": kind.slug(),
                        "titleMarker": title_marker
                    })))
                }
                Request::TriggerHotkey { .. } => {
                    self.require_phase(Phase::Launched, "triggerHotkey")?;
                    ensure_interactive_desktop()
                        .map_err(|message| FixtureError::new("desktop_unavailable", message))?;
                    self.session
                        .as_ref()
                        .expect("launched phase has a session")
                        .editor
                        .trigger_emenda_hotkey()
                        .map_err(|message| FixtureError::new("hotkey_delivery", message))?;
                    self.phase = Phase::Triggered;
                    Ok(CommandResult::Continue(json!({ "invocations": 1 })))
                }
                Request::VerifyAndSave { .. } => {
                    self.require_phase(Phase::Triggered, "verifyAndSave")?;
                    ensure_interactive_desktop()
                        .map_err(|message| FixtureError::new("desktop_unavailable", message))?;
                    let session = self
                        .session
                        .as_ref()
                        .expect("triggered phase has a session");
                    session
                        .editor
                        .wait_until_refocused()
                        .map_err(|message| FixtureError::new("source_refocus_failed", message))?;
                    session
                        .editor
                        .save_and_wait(&session.file, &session.expected_text)
                        .map_err(|message| FixtureError::new("persistence_mismatch", message))?;
                    let persisted = fs::read_to_string(&session.file).map_err(|error| {
                        FixtureError::new("fixture_io", format!("read source fixture: {error}"))
                    })?;
                    if persisted != session.expected_text {
                        return Err(FixtureError::new(
                            "persistence_mismatch",
                            "the persisted source text did not exactly match expectedText",
                        ));
                    }
                    self.phase = Phase::Verified;
                    Ok(CommandResult::Continue(json!({
                        "verified": true,
                        "byteLength": persisted.len()
                    })))
                }
                Request::Shutdown { .. } => {
                    if let Some(mut session) = self.session.take() {
                        let mut failures = Vec::new();
                        if let Err(error) = session.editor.close_checked() {
                            failures.push(error);
                        }
                        if let Err(error) = session.workspace.close() {
                            failures
                                .push(format!("remove the isolated fixture workspace: {error}"));
                        }
                        if !failures.is_empty() {
                            return Err(FixtureError::new("cleanup_failed", failures.join("; ")));
                        }
                    }
                    Ok(CommandResult::Stop(json!({ "stopped": true })))
                }
            }
        }

        fn require_phase(&self, expected: Phase, command: &str) -> Result<(), FixtureError> {
            if self.phase == expected {
                Ok(())
            } else {
                Err(FixtureError::new(
                    "invalid_state",
                    format!(
                        "{command} is invalid in phase {:?}; expected {:?}",
                        self.phase, expected
                    ),
                ))
            }
        }
    }

    fn validate_text(field: &str, value: &str) -> Result<(), FixtureError> {
        if value.is_empty() || value.len() > MAX_TEXT_BYTES || value.contains('\0') {
            Err(FixtureError::new(
                "invalid_request",
                format!(
                    "{field} must be non-empty UTF-8 without NUL and at most {MAX_TEXT_BYTES} bytes"
                ),
            ))
        } else {
            Ok(())
        }
    }

    struct BoundedLine {
        bytes: Vec<u8>,
        oversized: bool,
    }

    fn read_bounded_line(reader: &mut impl BufRead) -> io::Result<Option<BoundedLine>> {
        let mut bytes = Vec::new();
        let mut oversized = false;
        loop {
            let available = reader.fill_buf()?;
            if available.is_empty() {
                return if bytes.is_empty() && !oversized {
                    Ok(None)
                } else {
                    Ok(Some(BoundedLine { bytes, oversized }))
                };
            }
            let newline = available.iter().position(|byte| *byte == b'\n');
            let consumed = newline.map_or(available.len(), |index| index + 1);
            let content = newline.map_or(available, |index| &available[..index]);
            if !oversized && bytes.len() + content.len() <= MAX_JSONL_BYTES {
                bytes.extend_from_slice(content);
            } else {
                oversized = true;
            }
            reader.consume(consumed);
            if newline.is_some() {
                if bytes.last() == Some(&b'\r') {
                    bytes.pop();
                }
                return Ok(Some(BoundedLine { bytes, oversized }));
            }
        }
    }

    fn emit_response(
        writer: &mut impl Write,
        response: &Response<'_>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        serde_json::to_writer(&mut *writer, response)?;
        writer.write_all(b"\n")?;
        writer.flush()?;
        Ok(())
    }

    pub(super) fn run() -> Result<(), Box<dyn std::error::Error>> {
        let stdin = io::stdin();
        let stdout = io::stdout();
        let mut reader = BufReader::new(stdin.lock());
        let mut writer = BufWriter::new(stdout.lock());
        let mut server = Server::new();

        while let Some(line) = read_bounded_line(&mut reader)? {
            if line.oversized {
                emit_response(
                    &mut writer,
                    &Response {
                        id: None,
                        ok: false,
                        result: None,
                        error: Some(ProtocolError {
                            code: "line_too_long",
                            message: format!(
                                "JSONL requests are limited to {MAX_JSONL_BYTES} bytes"
                            ),
                        }),
                    },
                )?;
                continue;
            }

            let value: Value = match serde_json::from_slice(&line.bytes) {
                Ok(value) => value,
                Err(error) => {
                    emit_response(
                        &mut writer,
                        &Response {
                            id: None,
                            ok: false,
                            result: None,
                            error: Some(ProtocolError {
                                code: "invalid_json",
                                message: error.to_string(),
                            }),
                        },
                    )?;
                    continue;
                }
            };
            let id = value.get("id").and_then(Value::as_u64);
            let request: Request = match serde_json::from_value(value) {
                Ok(request) => request,
                Err(error) => {
                    emit_response(
                        &mut writer,
                        &Response {
                            id,
                            ok: false,
                            result: None,
                            error: Some(ProtocolError {
                                code: "invalid_request",
                                message: error.to_string(),
                            }),
                        },
                    )?;
                    continue;
                }
            };
            let id = request.id();
            match server.handle(request) {
                Ok(CommandResult::Continue(result)) => emit_response(
                    &mut writer,
                    &Response {
                        id: Some(id),
                        ok: true,
                        result: Some(result),
                        error: None,
                    },
                )?,
                Ok(CommandResult::Stop(result)) => {
                    emit_response(
                        &mut writer,
                        &Response {
                            id: Some(id),
                            ok: true,
                            result: Some(result),
                            error: None,
                        },
                    )?;
                    return Ok(());
                }
                Err(error) => emit_response(
                    &mut writer,
                    &Response {
                        id: Some(id),
                        ok: false,
                        result: None,
                        error: Some(ProtocolError {
                            code: error.code,
                            message: error.message,
                        }),
                    },
                )?,
            }
        }
        Ok(())
    }
}

#[cfg(target_os = "windows")]
fn main() {
    if let Err(error) = windows::run() {
        eprintln!("desktop E2E fixture failed: {error}");
        std::process::exit(1);
    }
}
