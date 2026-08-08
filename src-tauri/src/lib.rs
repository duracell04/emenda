#![forbid(unsafe_code)]

pub mod correction;
pub mod error;
pub mod inference;
pub mod language;
pub mod settings;
pub mod snapshot;
pub mod text;
pub mod workflow;

use crate::{
    error::CoreError,
    inference::{InferenceProvider, Model, OpenRouterProvider},
    settings::{
        api_key_available, CredentialStore, CredentialUpdate, KeyringCredentialStore,
        PublicSettings, SaveSettingsInput, SettingsStore, DEFAULT_HOTKEY,
    },
    text::TextSurfaceAdapter,
    workflow::{WorkflowController, WorkflowState},
};
use std::{path::PathBuf, sync::Arc, time::Duration};
use tauri::{AppHandle, Emitter, Manager, State};
use tauri_plugin_global_shortcut::{Builder as ShortcutBuilder, ShortcutState};

const WORKFLOW_EVENT: &str = "emenda://workflow-state";

struct ApplicationState {
    settings: Arc<SettingsStore>,
    credentials: Arc<dyn CredentialStore>,
    provider: Arc<dyn InferenceProvider>,
    workflow: Arc<WorkflowController>,
}

#[tauri::command]
fn get_settings(state: State<'_, ApplicationState>) -> Result<PublicSettings, CoreError> {
    public_settings(&state)
}

#[tauri::command]
fn save_settings(
    state: State<'_, ApplicationState>,
    settings: SaveSettingsInput,
) -> Result<PublicSettings, CoreError> {
    let (settings, credential_update) = settings.validate()?;
    state.settings.save(&settings)?;
    match credential_update {
        CredentialUpdate::Unchanged => {}
        CredentialUpdate::Remove => state.credentials.delete_api_key()?,
        CredentialUpdate::Replace(api_key) => state.credentials.set_api_key(&api_key)?,
    }
    public_settings(&state)
}

#[tauri::command]
async fn test_openrouter(state: State<'_, ApplicationState>) -> Result<(), CoreError> {
    state.provider.health_check().await
}

#[tauri::command]
async fn list_models(state: State<'_, ApplicationState>) -> Result<Vec<Model>, CoreError> {
    state.provider.list_models().await
}

#[tauri::command]
async fn check_current_selection(
    app: AppHandle,
    state: State<'_, ApplicationState>,
) -> Result<WorkflowState, CoreError> {
    // A button invocation originates in Emenda, unlike the global shortcut.
    // Hide first so Windows can reactivate the source window and preserve its
    // selection before clipboard-assisted capture begins.
    if let Some(window) = app.get_webview_window("main") {
        window.hide().map_err(|error| {
            CoreError::TextCaptureError(format!("Could not hide Emenda before capture: {error}"))
        })?;
    }
    tokio::time::sleep(Duration::from_millis(180)).await;

    let publisher = app.clone();
    Ok(state
        .workflow
        .check_current_selection(move |workflow_state| {
            publish_workflow(&publisher, workflow_state, true);
        })
        .await)
}

#[tauri::command]
async fn apply_correction(
    app: AppHandle,
    state: State<'_, ApplicationState>,
    correction_index: usize,
) -> Result<WorkflowState, CoreError> {
    let publisher = app.clone();
    let workflow_state = state
        .workflow
        .apply_correction(correction_index, move |workflow_state| {
            let focus_window = matches!(&workflow_state, WorkflowState::Error { .. });
            publish_workflow(&publisher, workflow_state, focus_window);
        })
        .await;

    if matches!(&workflow_state, WorkflowState::Clean { applied: true, .. }) {
        if let Some(window) = app.get_webview_window("main") {
            let _ = window.hide();
        }
    }
    Ok(workflow_state)
}

#[tauri::command]
async fn dismiss_suggestions(
    app: AppHandle,
    state: State<'_, ApplicationState>,
) -> Result<WorkflowState, CoreError> {
    let publisher = app.clone();
    let workflow_state = state
        .workflow
        .finish_or_dismiss(move |workflow_state| {
            let focus_window = matches!(&workflow_state, WorkflowState::Error { .. });
            publish_workflow(&publisher, workflow_state, focus_window);
        })
        .await;
    if matches!(
        &workflow_state,
        WorkflowState::Idle | WorkflowState::Clean { applied: true, .. }
    ) {
        if let Some(window) = app.get_webview_window("main") {
            let _ = window.hide();
        }
    }
    Ok(workflow_state)
}

fn public_settings(state: &ApplicationState) -> Result<PublicSettings, CoreError> {
    let settings = state.settings.load()?;
    let configured = api_key_available(state.credentials.as_ref())?;
    Ok(PublicSettings::new(&settings, configured))
}

fn publish_workflow(app: &AppHandle, workflow_state: WorkflowState, focus_window: bool) {
    if focus_window {
        if let Some(window) = app.get_webview_window("main") {
            let _ = window.show();
            let _ = window.unminimize();
            let _ = window.set_focus();
        }
    }
    let _ = app.emit(WORKFLOW_EVENT, workflow_state);
}

fn initialise_state(app: &tauri::App) -> Result<ApplicationState, Box<dyn std::error::Error>> {
    let config_dir = app.path().app_config_dir()?;
    let settings = Arc::new(SettingsStore::new(settings_path(config_dir)));
    let credentials: Arc<dyn CredentialStore> = Arc::new(KeyringCredentialStore::new());
    let provider: Arc<dyn InferenceProvider> =
        Arc::new(OpenRouterProvider::new(Arc::clone(&credentials))?);
    let adapter: Arc<dyn TextSurfaceAdapter> = Arc::from(text::platform_adapter()?);
    let workflow = Arc::new(WorkflowController::new(
        adapter,
        Arc::clone(&provider),
        Arc::clone(&settings),
    ));
    Ok(ApplicationState {
        settings,
        credentials,
        provider,
        workflow,
    })
}

fn settings_path(config_dir: PathBuf) -> PathBuf {
    config_dir.join("settings.json")
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            app.manage(initialise_state(app)?);

            app.handle().plugin(
                ShortcutBuilder::new()
                    .with_shortcuts([DEFAULT_HOTKEY])?
                    .with_handler(|app, _shortcut, event| {
                        if event.state != ShortcutState::Pressed {
                            return;
                        }

                        let app = app.clone();
                        let workflow = app.state::<ApplicationState>().workflow.clone();
                        tauri::async_runtime::spawn(async move {
                            let publisher = app.clone();
                            workflow
                                .check_current_selection(move |workflow_state| {
                                    publish_workflow(&publisher, workflow_state, true);
                                })
                                .await;
                        });
                    })
                    .build(),
            )?;
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_settings,
            save_settings,
            test_openrouter,
            list_models,
            check_current_selection,
            apply_correction,
            dismiss_suggestions
        ])
        .run(tauri::generate_context!())
        .expect("failed to run Emenda");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn settings_file_stays_inside_the_tauri_config_directory() {
        let config_dir = PathBuf::from("config-root");
        assert_eq!(
            settings_path(config_dir),
            PathBuf::from("config-root").join("settings.json")
        );
    }
}
