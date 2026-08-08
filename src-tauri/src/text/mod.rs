//! Desktop text-surface transport.
//!
//! The correction workflow deliberately depends on this small boundary rather
//! than on a particular clipboard, input-simulation, or windowing library.
//! V0.1 supplies a Windows clipboard-assisted adapter; later accessibility
//! adapters can implement the same contract without changing snapshot state.

use serde::{Deserialize, Serialize};
use std::{error::Error, fmt};

#[cfg(target_os = "windows")]
mod windows;

#[cfg(target_os = "windows")]
pub use windows::WindowsTextSurfaceAdapter;

/// Identifies the desktop window from which a selection was captured.
///
/// `window_id` is kept as the platform adapter's opaque identifier. Callers
/// should not parse it or use a process id as proof that the original window
/// still exists.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SourceApplication {
    pub process_id: u64,
    pub application_name: String,
    pub executable: Option<String>,
    pub window_title: String,
    pub window_id: String,
}

/// Selected text together with the window that owned the selection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CapturedSelection {
    pub text: String,
    pub source: SourceApplication,
}

/// Typed failures at the privileged desktop boundary.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TextSurfaceError {
    /// No adapter is available for this build target.
    UnsupportedPlatform,
    /// The selected text or its source application could not be captured.
    Capture(String),
    /// The source selection could not be replaced deterministically.
    Replacement(String),
    /// The operating system denied interaction with the source surface.
    ProtectedSurface(String),
}

impl fmt::Display for TextSurfaceError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnsupportedPlatform => formatter
                .write_str("Desktop text capture is not supported on this operating system"),
            Self::Capture(message)
            | Self::Replacement(message)
            | Self::ProtectedSurface(message) => formatter.write_str(message),
        }
    }
}

impl Error for TextSurfaceError {}

/// Small boundary around selected-text capture and replacement.
pub trait TextSurfaceAdapter: Send + Sync {
    fn capture_selection(&self) -> Result<CapturedSelection, TextSurfaceError>;

    fn focus_source(&self, source: &SourceApplication) -> Result<(), TextSurfaceError>;

    /// Refocuses `source` and replaces its still-active selection with one
    /// native paste operation so the source application's undo remains useful.
    fn replace_selection(
        &self,
        source: &SourceApplication,
        replacement: &str,
    ) -> Result<(), TextSurfaceError>;
}

/// Construct the adapter appropriate for the current build target.
#[cfg(target_os = "windows")]
pub fn platform_adapter() -> Result<Box<dyn TextSurfaceAdapter>, TextSurfaceError> {
    Ok(Box::new(WindowsTextSurfaceAdapter::new()?))
}

/// Non-Windows builds remain viable while V0.1's native transport is Windows
/// only. A later platform module can replace this typed unsupported result.
#[cfg(not(target_os = "windows"))]
pub fn platform_adapter() -> Result<Box<dyn TextSurfaceAdapter>, TextSurfaceError> {
    Err(TextSurfaceError::UnsupportedPlatform)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn source() -> SourceApplication {
        SourceApplication {
            process_id: 42,
            application_name: "Editor".into(),
            executable: Some(r"C:\Program Files\Editor\editor.exe".into()),
            window_title: "Draft".into(),
            window_id: "HWND(1234)".into(),
        }
    }

    #[test]
    fn source_application_uses_the_frontend_contract() {
        let json = serde_json::to_value(source()).expect("source should serialize");

        assert_eq!(json["processId"], 42);
        assert_eq!(json["applicationName"], "Editor");
        assert_eq!(json["windowTitle"], "Draft");
        assert!(json.get("process_id").is_none());
    }

    #[test]
    fn captured_selection_round_trips_without_losing_source_identity() {
        let captured = CapturedSelection {
            text: "Gr\u{fc}ezi, Welt".into(),
            source: source(),
        };

        let encoded = serde_json::to_string(&captured).expect("selection should serialize");
        let decoded: CapturedSelection =
            serde_json::from_str(&encoded).expect("selection should deserialize");

        assert_eq!(decoded, captured);
    }

    #[cfg(not(target_os = "windows"))]
    #[test]
    fn unsupported_targets_return_a_typed_error() {
        assert!(matches!(
            platform_adapter(),
            Err(TextSurfaceError::UnsupportedPlatform)
        ));
    }
}
