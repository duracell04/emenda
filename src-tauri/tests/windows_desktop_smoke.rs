#![cfg(target_os = "windows")]

#[path = "support/windows_editor.rs"]
mod windows_editor;

use emenda_lib::{
    error::CoreError,
    inference::{InferenceProvider, OpenRouterProvider},
    settings::{ApiKey, CredentialStore, SettingsStore, DEFAULT_MODEL_ID, OPENROUTER_API_KEY_ENV},
    text::{platform_adapter, TextSurfaceAdapter},
    workflow::{WorkflowController, WorkflowState},
};
use std::{env, fs, path::Path, sync::Arc};
use windows_editor::{title_matches, EditorKind, EditorSession, DESKTOP_SMOKE_LOCK};

const ORIGINAL_TEXT: &str = "I liek this sentence.";
const CORRECTED_TEXT: &str = "I like this sentence.";

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
    // tests serialize native interaction if both ignored cases share a process.
    let _desktop_guard = DESKTOP_SMOKE_LOCK
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    assert!(
        env::var_os(OPENROUTER_API_KEY_ENV).is_some_and(|value| !value.is_empty()),
        "set OPENROUTER_API_KEY before running the ignored Windows desktop smoke test"
    );

    let workspace = tempfile::tempdir().expect("create smoke-test workspace");
    let settings_store = SettingsStore::new(workspace.path().join("settings.json"));
    let settings = settings_store
        .load()
        .expect("load isolated smoke-test settings");
    assert_eq!(
        settings.model_id(),
        DEFAULT_MODEL_ID,
        "strict native smoke must use openrouter/free"
    );
    let settings_store = Arc::new(settings_store);
    let credentials: Arc<dyn CredentialStore> = Arc::new(EnvironmentCredentials);
    let provider: Arc<dyn InferenceProvider> = Arc::new(
        OpenRouterProvider::new(credentials).expect("initialise the live OpenRouter provider"),
    );
    let adapter: Arc<dyn TextSurfaceAdapter> =
        Arc::from(platform_adapter().expect("initialise the real Windows text-surface adapter"));
    let workflow = WorkflowController::new(adapter, provider, settings_store);

    let editor_name = kind.name();
    let file = workspace.path().join(format!(
        "emenda-{}-smoke-{}.txt",
        kind.slug(),
        std::process::id()
    ));
    fs::write(&file, ORIGINAL_TEXT)
        .unwrap_or_else(|error| panic!("{editor_name}: seed source file: {error}"));

    let mut editor = EditorSession::launch(kind, &file, workspace.path())
        .unwrap_or_else(|error| panic!("{editor_name}: launch editor: {error}"));
    editor
        .wait_until_active()
        .unwrap_or_else(|error| panic!("{editor_name}: wait for editor window: {error}"));
    tauri::async_runtime::block_on(exercise_surface(&workflow, &editor, &file));
    editor.close();
}

async fn exercise_surface(workflow: &WorkflowController, editor: &EditorSession, file: &Path) {
    let editor_name = editor.kind().name();
    editor
        .select_all()
        .unwrap_or_else(|error| panic!("{editor_name}: prepare the source selection: {error}"));

    // A strict smoke run is one explicit user-equivalent invocation. There is
    // no retry or fallback: any typed provider/protocol/semantic state remains
    // the recorded outcome of this openrouter/free attempt.
    let state = workflow.check_current_selection(|_| {}).await;
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
            assert!(
                title_matches(
                    &source_application.window_title,
                    editor.title_marker()
                ),
                "{editor_name}: capture came from an unexpected source window"
            );
            corrections
                .iter()
                .position(|correction| {
                    correction.original() == "liek" && correction.replacement() == "like"
                })
                .unwrap_or_else(|| {
                    panic!(
                        "{editor_name}: the single openrouter/free invocation did not return the exact 'liek' -> 'like' suggestion"
                    )
                })
        }
        other => panic!(
            "{editor_name}: the single openrouter/free invocation did not produce validated suggestions: {other:?}"
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

    editor
        .save_and_wait(file, CORRECTED_TEXT)
        .unwrap_or_else(|error| {
            panic!("{editor_name}: the editor must persist Emenda's replacement: {error}")
        });
}
