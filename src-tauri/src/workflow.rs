use crate::{
    correction::{apply_corrections, Correction},
    error::{CoreError, ErrorKind},
    inference::{CheckRequest, InferenceProvider},
    settings::SettingsStore,
    snapshot::{RevisionId, SnapshotStore, TextSnapshot},
    text::{SourceApplication, TextSurfaceAdapter, TextSurfaceError},
};
use serde::Serialize;
use std::sync::{Arc, Mutex, MutexGuard};

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkflowError {
    kind: ErrorKind,
    message: String,
}

impl From<&CoreError> for WorkflowError {
    fn from(error: &CoreError) -> Self {
        Self {
            kind: error.kind(),
            message: error.user_message(),
        }
    }
}

/// The complete, runtime-validated state exposed to the React window.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize)]
#[serde(tag = "status", rename_all = "camelCase")]
pub enum WorkflowState {
    #[default]
    Idle,
    Checking {
        revision_id: u64,
        source_text: String,
        source_application: SourceApplication,
    },
    Suggestions {
        revision_id: u64,
        source_text: String,
        working_text: String,
        source_application: SourceApplication,
        corrections: Vec<Correction>,
        accepted_count: usize,
    },
    Clean {
        revision_id: u64,
        working_text: String,
        source_application: SourceApplication,
        applied: bool,
    },
    Error {
        error: WorkflowError,
    },
}

#[derive(Debug, Clone)]
struct ActiveSession {
    snapshot: TextSnapshot,
    remaining: Vec<Correction>,
    accepted: Vec<Correction>,
    working_text: String,
}

/// Coordinates the one vertical slice while keeping UI/window concerns out of
/// the deterministic correction state machine.
pub struct WorkflowController {
    adapter: Arc<dyn TextSurfaceAdapter>,
    provider: Arc<dyn InferenceProvider>,
    settings: Arc<SettingsStore>,
    snapshots: Mutex<SnapshotStore>,
    active: Mutex<Option<ActiveSession>>,
    state: Mutex<WorkflowState>,
    desktop_operation: tokio::sync::Mutex<()>,
}

impl WorkflowController {
    pub fn new(
        adapter: Arc<dyn TextSurfaceAdapter>,
        provider: Arc<dyn InferenceProvider>,
        settings: Arc<SettingsStore>,
    ) -> Self {
        Self {
            adapter,
            provider,
            settings,
            snapshots: Mutex::new(SnapshotStore::new()),
            active: Mutex::new(None),
            state: Mutex::new(WorkflowState::Idle),
            desktop_operation: tokio::sync::Mutex::new(()),
        }
    }

    pub fn current_state(&self) -> WorkflowState {
        lock(&self.state).clone()
    }

    /// Capture a selection, create the authoritative snapshot, and validate a
    /// structured OpenRouter result. Every state transition is published so a
    /// hotkey invocation can update the window before and after the await.
    pub async fn check_current_selection<F>(&self, publish: F) -> WorkflowState
    where
        F: Fn(WorkflowState) + Send + Sync,
    {
        let snapshot = {
            let _desktop_guard = self.desktop_operation.lock().await;
            match self.adapter.capture_selection() {
                Ok(captured) if !captured.text.trim().is_empty() => {
                    match lock(&self.snapshots).create(captured.text, captured.source) {
                        Ok(snapshot) => snapshot,
                        Err(error) => return self.publish_error(error, &publish),
                    }
                }
                Ok(_) => {
                    return self.publish_error(
                        CoreError::TextCaptureError(
                            "Select some text before invoking Emenda".to_owned(),
                        ),
                        &publish,
                    );
                }
                Err(error) => return self.publish_error(capture_error(error), &publish),
            }
        };

        let checking = WorkflowState::Checking {
            revision_id: snapshot.revision_id().get(),
            source_text: snapshot.text().to_owned(),
            source_application: snapshot.source().clone(),
        };
        self.publish_state(checking, &publish);

        let settings = match self.settings.load() {
            Ok(settings) => settings,
            Err(error) => {
                return self.publish_error_if_current(snapshot.revision_id(), error, &publish)
            }
        };
        let request = match CheckRequest::new(
            snapshot.clone(),
            settings.model_id(),
            settings.language_mode(),
        ) {
            Ok(request) => request,
            Err(error) => {
                return self.publish_error_if_current(snapshot.revision_id(), error, &publish)
            }
        };
        let result = match self.provider.check_text(request).await {
            Ok(result) => result,
            Err(error) => {
                return self.publish_error_if_current(snapshot.revision_id(), error, &publish)
            }
        };

        if let Err(error) = lock(&self.snapshots).ensure_current(result.revision_id()) {
            // A newer invocation owns the visible state. The stale result is
            // deliberately ignored instead of overwriting that newer state.
            let _typed_stale_result = error;
            return self.current_state();
        }

        let (_, _, corrections, non_applicable) = result.into_parts();
        if corrections.is_empty() && !non_applicable.is_empty() {
            return self.publish_error_if_current(
                snapshot.revision_id(),
                CoreError::ValidationError(
                    "OpenRouter returned corrections that could not be matched to the selected text"
                        .to_owned(),
                ),
                &publish,
            );
        }

        if corrections.is_empty() {
            *lock(&self.active) = None;
            let clean = WorkflowState::Clean {
                revision_id: snapshot.revision_id().get(),
                working_text: snapshot.text().to_owned(),
                source_application: snapshot.source().clone(),
                applied: false,
            };
            return self.publish_state(clean, &publish);
        }

        let session = ActiveSession {
            working_text: snapshot.text().to_owned(),
            snapshot: snapshot.clone(),
            remaining: corrections,
            accepted: Vec::new(),
        };
        let suggestions = session.view();
        *lock(&self.active) = Some(session);
        self.publish_state(suggestions, &publish)
    }

    /// Accept one visible correction. Accepted edits are rebuilt from the
    /// immutable snapshot; the source application receives exactly one paste
    /// after the final correction is accepted.
    pub async fn apply_correction<F>(&self, correction_index: usize, publish: F) -> WorkflowState
    where
        F: Fn(WorkflowState) + Send + Sync,
    {
        let _desktop_guard = self.desktop_operation.lock().await;
        let (revision_id, source, candidate_text, final_correction) = {
            let active = lock(&self.active);
            let Some(session) = active.as_ref() else {
                return self.publish_error(
                    CoreError::ValidationError("There is no active correction to apply".to_owned()),
                    &publish,
                );
            };

            if correction_index >= session.remaining.len() {
                return self.publish_error(
                    CoreError::ValidationError(
                        "The selected correction no longer exists".to_owned(),
                    ),
                    &publish,
                );
            }
            if let Err(error) = lock(&self.snapshots).ensure_current(session.snapshot.revision_id())
            {
                return self.publish_error(error, &publish);
            }

            let mut accepted = session.accepted.clone();
            accepted.push(session.remaining[correction_index].clone());
            let candidate_text = match apply_corrections(session.snapshot.text(), &accepted) {
                Ok(text) => text,
                Err(error) => return self.publish_error(error, &publish),
            };
            (
                session.snapshot.revision_id(),
                session.snapshot.source().clone(),
                candidate_text,
                session.remaining.len() == 1,
            )
        };

        if final_correction {
            if let Err(error) = lock(&self.snapshots).ensure_current(revision_id) {
                return self.publish_error(error, &publish);
            }
            let snapshot = {
                let active = lock(&self.active);
                match active.as_ref() {
                    Some(session) => session.snapshot.clone(),
                    None => {
                        return self.publish_error(
                            CoreError::ValidationError(
                                "The correction session is no longer active".to_owned(),
                            ),
                            &publish,
                        )
                    }
                }
            };
            if let Err(error) = self.verify_and_replace(&snapshot, &candidate_text) {
                return self.publish_error(error, &publish);
            }

            *lock(&self.active) = None;
            let clean = WorkflowState::Clean {
                revision_id: revision_id.get(),
                working_text: candidate_text,
                source_application: source,
                applied: true,
            };
            return self.publish_state(clean, &publish);
        }

        let suggestions = {
            let mut active = lock(&self.active);
            let Some(session) = active.as_mut() else {
                return self.publish_error(
                    CoreError::StaleRevisionError {
                        result_revision: revision_id.get(),
                        authoritative_revision: lock(&self.snapshots).latest_revision().get(),
                    },
                    &publish,
                );
            };
            let correction = session.remaining.remove(correction_index);
            session.accepted.push(correction);
            session.working_text = candidate_text;
            session.view()
        };
        self.publish_state(suggestions, &publish)
    }

    /// Dismiss an untouched result, or finish an explicitly accepted subset.
    /// This lets the user reject remaining suggestions without silently losing
    /// corrections already accepted during review.
    pub async fn finish_or_dismiss<F>(&self, publish: F) -> WorkflowState
    where
        F: Fn(WorkflowState) + Send + Sync,
    {
        let _desktop_guard = self.desktop_operation.lock().await;
        let session = lock(&self.active).clone();
        let Some(session) = session else {
            return self.publish_state(WorkflowState::Idle, &publish);
        };

        if session.accepted.is_empty() {
            *lock(&self.active) = None;
            return self.publish_state(WorkflowState::Idle, &publish);
        }
        if let Err(error) = lock(&self.snapshots).ensure_current(session.snapshot.revision_id()) {
            return self.publish_error(error, &publish);
        }
        if let Err(error) = self.verify_and_replace(&session.snapshot, &session.working_text) {
            return self.publish_error(error, &publish);
        }

        *lock(&self.active) = None;
        self.publish_state(
            WorkflowState::Clean {
                revision_id: session.snapshot.revision_id().get(),
                working_text: session.working_text,
                source_application: session.snapshot.source().clone(),
                applied: true,
            },
            &publish,
        )
    }

    fn publish_state<F>(&self, state: WorkflowState, publish: &F) -> WorkflowState
    where
        F: Fn(WorkflowState),
    {
        *lock(&self.state) = state.clone();
        publish(state.clone());
        state
    }

    fn publish_error<F>(&self, error: CoreError, publish: &F) -> WorkflowState
    where
        F: Fn(WorkflowState),
    {
        self.publish_state(
            WorkflowState::Error {
                error: WorkflowError::from(&error),
            },
            publish,
        )
    }

    fn publish_error_if_current<F>(
        &self,
        revision_id: RevisionId,
        error: CoreError,
        publish: &F,
    ) -> WorkflowState
    where
        F: Fn(WorkflowState),
    {
        if lock(&self.snapshots).is_current(revision_id) {
            self.publish_error(error, publish)
        } else {
            self.current_state()
        }
    }

    /// Re-copy the still-selected source text before replacement. Window
    /// identity alone is insufficient because a user can move the caret or
    /// edit the document while reviewing suggestions.
    fn verify_and_replace(
        &self,
        snapshot: &TextSnapshot,
        replacement: &str,
    ) -> Result<(), CoreError> {
        self.adapter
            .focus_source(snapshot.source())
            .map_err(replacement_error)?;
        let current = self
            .adapter
            .capture_selection()
            .map_err(replacement_error)?;
        let same_window = current.source.process_id == snapshot.source().process_id
            && current.source.window_id == snapshot.source().window_id;
        if !same_window {
            return Err(CoreError::TextReplacementError(
                "The original source window is no longer active".to_owned(),
            ));
        }
        if current.text != snapshot.text() {
            return Err(CoreError::TextReplacementError(
                "The original selection changed during review; invoke Emenda again".to_owned(),
            ));
        }
        self.adapter
            .replace_selection(snapshot.source(), replacement)
            .map_err(replacement_error)
    }
}

impl ActiveSession {
    fn view(&self) -> WorkflowState {
        WorkflowState::Suggestions {
            revision_id: self.snapshot.revision_id().get(),
            source_text: self.snapshot.text().to_owned(),
            working_text: self.working_text.clone(),
            source_application: self.snapshot.source().clone(),
            corrections: self.remaining.clone(),
            accepted_count: self.accepted.len(),
        }
    }
}

fn lock<T>(mutex: &Mutex<T>) -> MutexGuard<'_, T> {
    mutex
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
}

fn capture_error(error: TextSurfaceError) -> CoreError {
    match error {
        TextSurfaceError::ProtectedSurface(message) => CoreError::ProtectedSurfaceError(message),
        TextSurfaceError::Capture(message) | TextSurfaceError::Replacement(message) => {
            CoreError::TextCaptureError(message)
        }
        TextSurfaceError::UnsupportedPlatform => CoreError::TextCaptureError(error.to_string()),
    }
}

fn replacement_error(error: TextSurfaceError) -> CoreError {
    match error {
        TextSurfaceError::ProtectedSurface(message) => CoreError::ProtectedSurfaceError(message),
        TextSurfaceError::Capture(message) | TextSurfaceError::Replacement(message) => {
            CoreError::TextReplacementError(message)
        }
        TextSurfaceError::UnsupportedPlatform => CoreError::TextReplacementError(error.to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        correction::{validate_candidates, Confidence, CorrectionCandidate, CorrectionCategory},
        inference::{CheckResult, InferenceError, Model},
        language::LanguageProfile,
        text::CapturedSelection,
    };
    use async_trait::async_trait;
    use std::{
        collections::{HashMap, VecDeque},
        sync::Arc,
    };
    use tempfile::TempDir;
    use tokio::sync::Notify;

    #[derive(Default)]
    struct MockTextSurface {
        captures: Mutex<VecDeque<Result<CapturedSelection, TextSurfaceError>>>,
        focused_sources: Mutex<Vec<SourceApplication>>,
        replacements: Mutex<Vec<(SourceApplication, String)>>,
    }

    impl MockTextSurface {
        fn with_captures(captures: impl IntoIterator<Item = CapturedSelection>) -> Self {
            Self {
                captures: Mutex::new(captures.into_iter().map(Ok).collect()),
                ..Self::default()
            }
        }

        fn replacements(&self) -> Vec<(SourceApplication, String)> {
            lock(&self.replacements).clone()
        }
    }

    impl TextSurfaceAdapter for MockTextSurface {
        fn capture_selection(&self) -> Result<CapturedSelection, TextSurfaceError> {
            lock(&self.captures).pop_front().unwrap_or_else(|| {
                Err(TextSurfaceError::Capture(
                    "Mock capture queue is empty".to_owned(),
                ))
            })
        }

        fn focus_source(&self, source: &SourceApplication) -> Result<(), TextSurfaceError> {
            lock(&self.focused_sources).push(source.clone());
            Ok(())
        }

        fn replace_selection(
            &self,
            source: &SourceApplication,
            replacement: &str,
        ) -> Result<(), TextSurfaceError> {
            lock(&self.replacements).push((source.clone(), replacement.to_owned()));
            Ok(())
        }
    }

    struct StaticProvider {
        corrections_by_text: HashMap<String, Vec<Correction>>,
    }

    impl StaticProvider {
        fn new(corrections_by_text: HashMap<String, Vec<Correction>>) -> Self {
            Self {
                corrections_by_text,
            }
        }
    }

    #[async_trait]
    impl InferenceProvider for StaticProvider {
        async fn list_models(&self) -> Result<Vec<Model>, InferenceError> {
            Ok(Vec::new())
        }

        async fn check_text(&self, request: CheckRequest) -> Result<CheckResult, InferenceError> {
            Ok(CheckResult {
                revision_id: request.snapshot().revision_id(),
                detected_language: LanguageProfile::EnGb,
                corrections: self
                    .corrections_by_text
                    .get(request.snapshot().text())
                    .cloned()
                    .unwrap_or_default(),
                non_applicable: Vec::new(),
            })
        }

        async fn health_check(&self) -> Result<(), InferenceError> {
            Ok(())
        }
    }

    struct DelayedFirstProvider {
        first_started: Arc<Notify>,
        release_first: Arc<Notify>,
    }

    #[async_trait]
    impl InferenceProvider for DelayedFirstProvider {
        async fn list_models(&self) -> Result<Vec<Model>, InferenceError> {
            Ok(Vec::new())
        }

        async fn check_text(&self, request: CheckRequest) -> Result<CheckResult, InferenceError> {
            if request.snapshot().revision_id().get() == 1 {
                self.first_started.notify_one();
                self.release_first.notified().await;
            }

            Ok(CheckResult {
                revision_id: request.snapshot().revision_id(),
                detected_language: LanguageProfile::EnGb,
                corrections: Vec::new(),
                non_applicable: Vec::new(),
            })
        }

        async fn health_check(&self) -> Result<(), InferenceError> {
            Ok(())
        }
    }

    fn source() -> SourceApplication {
        SourceApplication {
            process_id: 42,
            application_name: "Test Editor".to_owned(),
            executable: Some("editor.exe".to_owned()),
            window_title: "Draft".to_owned(),
            window_id: "window-42".to_owned(),
        }
    }

    fn captured(text: &str) -> CapturedSelection {
        CapturedSelection {
            text: text.to_owned(),
            source: source(),
        }
    }

    fn trusted_corrections(
        text: &str,
        candidates: impl IntoIterator<Item = (usize, usize, &'static str, &'static str)>,
    ) -> Vec<Correction> {
        let candidates = candidates
            .into_iter()
            .map(|(start, end, original, replacement)| CorrectionCandidate {
                start,
                end,
                original: original.to_owned(),
                replacement: replacement.to_owned(),
                category: CorrectionCategory::Spelling,
                confidence: Confidence::High,
                explanation: Some("Typo".to_owned()),
            })
            .collect();
        let report = validate_candidates(text, candidates);
        assert!(
            report.non_applicable().is_empty(),
            "test fixture corrections must be applicable"
        );
        report.into_corrections()
    }

    fn controller(
        adapter: Arc<MockTextSurface>,
        provider: Arc<dyn InferenceProvider>,
    ) -> (WorkflowController, TempDir) {
        let settings_directory = tempfile::tempdir().expect("temporary settings directory");
        let settings = Arc::new(SettingsStore::new(
            settings_directory.path().join("settings.json"),
        ));
        (
            WorkflowController::new(adapter, provider, settings),
            settings_directory,
        )
    }

    #[test]
    fn no_corrections_finishes_clean_without_touching_the_source() {
        tauri::async_runtime::block_on(async {
            let adapter = Arc::new(MockTextSurface::with_captures([captured(
                "Already correct.",
            )]));
            let provider = Arc::new(StaticProvider::new(HashMap::new()));
            let (controller, _settings_directory) = controller(adapter.clone(), provider);

            let state = controller.check_current_selection(|_| {}).await;

            assert_eq!(
                state,
                WorkflowState::Clean {
                    revision_id: 1,
                    working_text: "Already correct.".to_owned(),
                    source_application: source(),
                    applied: false,
                }
            );
            assert!(adapter.replacements().is_empty());
        });
    }

    #[test]
    fn accepted_subset_is_staged_then_finalized_with_exactly_one_replace() {
        tauri::async_runtime::block_on(async {
            let text = "A quik brownn fox.";
            let corrections =
                trusted_corrections(text, [(2, 6, "quik", "quick"), (7, 13, "brownn", "brown")]);
            let adapter = Arc::new(MockTextSurface::with_captures([
                captured(text),
                captured(text),
            ]));
            let provider = Arc::new(StaticProvider::new(HashMap::from([(
                text.to_owned(),
                corrections,
            )])));
            let (controller, _settings_directory) = controller(adapter.clone(), provider);

            let initial = controller.check_current_selection(|_| {}).await;
            assert!(matches!(
                initial,
                WorkflowState::Suggestions {
                    accepted_count: 0,
                    ..
                }
            ));

            let staged = controller.apply_correction(0, |_| {}).await;
            assert!(matches!(
                staged,
                WorkflowState::Suggestions {
                    ref working_text,
                    accepted_count: 1,
                    ..
                } if working_text == "A quick brownn fox."
            ));
            assert!(adapter.replacements().is_empty());

            let finalized = controller.finish_or_dismiss(|_| {}).await;
            assert!(matches!(
                finalized,
                WorkflowState::Clean {
                    ref working_text,
                    applied: true,
                    ..
                } if working_text == "A quick brownn fox."
            ));
            assert_eq!(
                adapter.replacements(),
                vec![(source(), "A quick brownn fox.".to_owned())]
            );
        });
    }

    #[test]
    fn changed_source_selection_returns_typed_replacement_error_without_pasting() {
        tauri::async_runtime::block_on(async {
            let text = "I liek this.";
            let corrections = trusted_corrections(text, [(2, 6, "liek", "like")]);
            let adapter = Arc::new(MockTextSurface::with_captures([
                captured(text),
                captured("The user changed this."),
            ]));
            let provider = Arc::new(StaticProvider::new(HashMap::from([(
                text.to_owned(),
                corrections,
            )])));
            let (controller, _settings_directory) = controller(adapter.clone(), provider);
            assert!(matches!(
                controller.check_current_selection(|_| {}).await,
                WorkflowState::Suggestions { .. }
            ));

            let state = controller.apply_correction(0, |_| {}).await;

            assert!(matches!(
                state,
                WorkflowState::Error {
                    error: WorkflowError {
                        kind: ErrorKind::TextReplacementError,
                        ..
                    }
                }
            ));
            assert!(adapter.replacements().is_empty());
        });
    }

    #[test]
    fn delayed_stale_response_cannot_overwrite_newer_authoritative_state() {
        tauri::async_runtime::block_on(async {
            let first_started = Arc::new(Notify::new());
            let release_first = Arc::new(Notify::new());
            let adapter = Arc::new(MockTextSurface::with_captures([
                captured("older text"),
                captured("newer text"),
            ]));
            let provider = Arc::new(DelayedFirstProvider {
                first_started: first_started.clone(),
                release_first: release_first.clone(),
            });
            let (controller, _settings_directory) = controller(adapter, provider);
            let controller = Arc::new(controller);

            let older_controller = controller.clone();
            let older = tauri::async_runtime::spawn(async move {
                older_controller.check_current_selection(|_| {}).await
            });
            first_started.notified().await;

            let newer = controller.check_current_selection(|_| {}).await;
            assert_eq!(
                newer,
                WorkflowState::Clean {
                    revision_id: 2,
                    working_text: "newer text".to_owned(),
                    source_application: source(),
                    applied: false,
                }
            );

            release_first.notify_one();
            let older_returned = older.await.expect("older request task should finish");

            assert_eq!(older_returned, newer);
            assert_eq!(controller.current_state(), newer);
        });
    }
}
