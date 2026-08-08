use crate::{error::CoreError, text::SourceApplication};
use serde::Serialize;
use std::time::SystemTime;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
#[serde(transparent)]
pub struct RevisionId(u64);

impl RevisionId {
    pub const fn get(self) -> u64 {
        self.0
    }
}

/// The immutable input identity for one inference request.
///
/// Fields are deliberately private and there are no mutation methods. The
/// workflow clones this value before awaiting inference, while `SnapshotStore`
/// independently retains the newest authoritative snapshot.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TextSnapshot {
    revision_id: RevisionId,
    text: String,
    created_at: SystemTime,
    source: SourceApplication,
}

impl TextSnapshot {
    pub const fn revision_id(&self) -> RevisionId {
        self.revision_id
    }

    pub fn text(&self) -> &str {
        &self.text
    }

    pub const fn created_at(&self) -> SystemTime {
        self.created_at
    }

    pub const fn source(&self) -> &SourceApplication {
        &self.source
    }
}

#[derive(Debug, Default)]
pub struct SnapshotStore {
    latest_revision: RevisionId,
    current: Option<TextSnapshot>,
}

impl SnapshotStore {
    pub fn new() -> Self {
        Self::default()
    }

    /// Start a new request. Its revision immediately supersedes every older
    /// in-flight request, even before this request returns.
    pub fn create(
        &mut self,
        text: String,
        source: SourceApplication,
    ) -> Result<TextSnapshot, CoreError> {
        let next_revision =
            self.latest_revision.0.checked_add(1).ok_or_else(|| {
                CoreError::ConfigurationError("Revision counter exhausted".into())
            })?;
        let snapshot = TextSnapshot {
            revision_id: RevisionId(next_revision),
            text,
            created_at: SystemTime::now(),
            source,
        };

        self.latest_revision = snapshot.revision_id;
        self.current = Some(snapshot.clone());
        Ok(snapshot)
    }

    pub fn current(&self) -> Option<&TextSnapshot> {
        self.current.as_ref()
    }

    pub const fn latest_revision(&self) -> RevisionId {
        self.latest_revision
    }

    pub fn is_current(&self, revision_id: RevisionId) -> bool {
        self.current
            .as_ref()
            .is_some_and(|snapshot| snapshot.revision_id == revision_id)
    }

    pub fn ensure_current(&self, revision_id: RevisionId) -> Result<(), CoreError> {
        if self.is_current(revision_id) {
            Ok(())
        } else {
            Err(CoreError::StaleRevisionError {
                result_revision: revision_id.get(),
                authoritative_revision: self.latest_revision.get(),
            })
        }
    }

    /// Admit a revision-bound value only if it belongs to the latest request.
    pub fn accept<T>(&self, revision_id: RevisionId, value: T) -> Result<T, CoreError> {
        self.ensure_current(revision_id)?;
        Ok(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn source() -> SourceApplication {
        SourceApplication {
            process_id: 7,
            application_name: "Editor".to_owned(),
            executable: Some("editor.exe".to_owned()),
            window_title: "Document".to_owned(),
            window_id: "window-7".to_owned(),
        }
    }

    #[test]
    fn revisions_increase_monotonically() {
        let mut store = SnapshotStore::new();
        let first = store.create("first".to_owned(), source()).unwrap();
        let second = store.create("second".to_owned(), source()).unwrap();

        assert_eq!(first.revision_id().get(), 1);
        assert_eq!(second.revision_id().get(), 2);
        assert_eq!(store.current().unwrap().text(), "second");
    }

    #[test]
    fn revision_41_is_stale_after_revision_42_starts() {
        let mut store = SnapshotStore::new();
        let mut revision_41 = None;
        for number in 1..=42 {
            let snapshot = store
                .create(format!("revision {number}"), source())
                .unwrap();
            if number == 41 {
                revision_41 = Some(snapshot.revision_id());
            }
        }

        let error = store.ensure_current(revision_41.unwrap()).unwrap_err();
        assert_eq!(
            error,
            CoreError::StaleRevisionError {
                result_revision: 41,
                authoritative_revision: 42,
            }
        );
        assert_eq!(store.current().unwrap().text(), "revision 42");
    }

    #[test]
    fn current_revision_is_accepted() {
        let mut store = SnapshotStore::new();
        let snapshot = store.create("current".to_owned(), source()).unwrap();

        assert_eq!(store.accept(snapshot.revision_id(), "result"), Ok("result"));
    }
}
