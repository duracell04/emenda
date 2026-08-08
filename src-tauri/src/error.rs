use serde::ser::{Serialize, SerializeStruct, Serializer};
use std::{error::Error, fmt};

/// Stable error categories crossing the Tauri command boundary.
///
/// Error details are intentionally plain strings: provider responses and
/// credential values must never be included in them.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[allow(clippy::enum_variant_names)] // Names are the stable product error taxonomy from SPEC.md.
pub enum ErrorKind {
    ConfigurationError,
    AuthenticationError,
    NetworkError,
    InferenceError,
    StructuredOutputError,
    ValidationError,
    StaleRevisionError,
    TextCaptureError,
    TextReplacementError,
    ProtectedSurfaceError,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(clippy::enum_variant_names)] // Names are the stable product error taxonomy from SPEC.md.
pub enum CoreError {
    ConfigurationError(String),
    AuthenticationError(String),
    NetworkError(String),
    InferenceError(String),
    StructuredOutputError(String),
    ValidationError(String),
    StaleRevisionError {
        result_revision: u64,
        authoritative_revision: u64,
    },
    TextCaptureError(String),
    TextReplacementError(String),
    ProtectedSurfaceError(String),
}

impl CoreError {
    pub const fn kind(&self) -> ErrorKind {
        match self {
            Self::ConfigurationError(_) => ErrorKind::ConfigurationError,
            Self::AuthenticationError(_) => ErrorKind::AuthenticationError,
            Self::NetworkError(_) => ErrorKind::NetworkError,
            Self::InferenceError(_) => ErrorKind::InferenceError,
            Self::StructuredOutputError(_) => ErrorKind::StructuredOutputError,
            Self::ValidationError(_) => ErrorKind::ValidationError,
            Self::StaleRevisionError { .. } => ErrorKind::StaleRevisionError,
            Self::TextCaptureError(_) => ErrorKind::TextCaptureError,
            Self::TextReplacementError(_) => ErrorKind::TextReplacementError,
            Self::ProtectedSurfaceError(_) => ErrorKind::ProtectedSurfaceError,
        }
    }

    /// A concise message safe to expose to the product UI.
    pub fn user_message(&self) -> String {
        match self {
            Self::ConfigurationError(message)
            | Self::AuthenticationError(message)
            | Self::NetworkError(message)
            | Self::InferenceError(message)
            | Self::StructuredOutputError(message)
            | Self::ValidationError(message)
            | Self::TextCaptureError(message)
            | Self::TextReplacementError(message)
            | Self::ProtectedSurfaceError(message) => message.clone(),
            Self::StaleRevisionError {
                result_revision,
                authoritative_revision,
            } => format!(
                "Revision {result_revision} is stale; revision {authoritative_revision} is current"
            ),
        }
    }
}

impl fmt::Display for CoreError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.user_message())
    }
}

impl Error for CoreError {}

impl Serialize for CoreError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("CoreError", 2)?;
        state.serialize_field("kind", &self.kind())?;
        state.serialize_field("message", &self.user_message())?;
        state.end()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serializes_a_stable_frontend_payload() {
        let error = CoreError::AuthenticationError("The API key was rejected".into());
        let json = serde_json::to_value(error).expect("error should serialize");

        assert_eq!(json["kind"], "authenticationError");
        assert_eq!(json["message"], "The API key was rejected");
    }

    #[test]
    fn stale_errors_include_both_revisions() {
        let error = CoreError::StaleRevisionError {
            result_revision: 41,
            authoritative_revision: 42,
        };

        assert_eq!(error.kind(), ErrorKind::StaleRevisionError);
        assert!(error.to_string().contains("41"));
        assert!(error.to_string().contains("42"));
    }
}
