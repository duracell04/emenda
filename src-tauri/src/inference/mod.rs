mod openrouter;

pub use openrouter::OpenRouterProvider;

use crate::{
    correction::{Correction, NonApplicableCorrection},
    error::CoreError,
    language::{LanguageMode, LanguageProfile},
    snapshot::{RevisionId, TextSnapshot},
};
use async_trait::async_trait;
use serde::Serialize;

pub type InferenceError = CoreError;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Model {
    pub id: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[derive(Debug, Clone)]
pub struct CheckRequest {
    snapshot: TextSnapshot,
    model_id: String,
    language_mode: LanguageMode,
}

impl CheckRequest {
    pub fn new(
        snapshot: TextSnapshot,
        model_id: impl Into<String>,
        language_mode: LanguageMode,
    ) -> Result<Self, CoreError> {
        let model_id = model_id.into();
        let model_id = model_id.trim();
        if model_id.is_empty() || model_id.len() > 512 || model_id.chars().any(char::is_control) {
            return Err(CoreError::ConfigurationError(
                "OpenRouter model ID is invalid".to_owned(),
            ));
        }
        Ok(Self {
            snapshot,
            model_id: model_id.to_owned(),
            language_mode,
        })
    }

    pub const fn snapshot(&self) -> &TextSnapshot {
        &self.snapshot
    }

    pub fn model_id(&self) -> &str {
        &self.model_id
    }

    pub const fn language_mode(&self) -> LanguageMode {
        self.language_mode
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CheckResult {
    pub revision_id: RevisionId,
    pub detected_language: LanguageProfile,
    pub corrections: Vec<Correction>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub non_applicable: Vec<NonApplicableCorrection>,
}

impl CheckResult {
    pub const fn revision_id(&self) -> RevisionId {
        self.revision_id
    }

    pub const fn detected_language(&self) -> LanguageProfile {
        self.detected_language
    }

    pub fn corrections(&self) -> &[Correction] {
        &self.corrections
    }

    pub fn non_applicable(&self) -> &[NonApplicableCorrection] {
        &self.non_applicable
    }

    pub fn into_parts(
        self,
    ) -> (
        RevisionId,
        LanguageProfile,
        Vec<Correction>,
        Vec<NonApplicableCorrection>,
    ) {
        (
            self.revision_id,
            self.detected_language,
            self.corrections,
            self.non_applicable,
        )
    }
}

#[async_trait]
pub trait InferenceProvider: Send + Sync {
    async fn list_models(&self) -> Result<Vec<Model>, InferenceError>;
    async fn check_text(&self, request: CheckRequest) -> Result<CheckResult, InferenceError>;
    async fn health_check(&self) -> Result<(), InferenceError>;
}
