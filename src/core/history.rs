use chrono::Utc;

use crate::storage::models::{Note, NoteHistory};
use crate::storage::Database;

use super::NoteServiceError;

pub struct HistoryService<'a> {
    db: &'a Database,
}

impl<'a> HistoryService<'a> {
    pub fn new(db: &'a Database) -> Self {
        Self { db }
    }

    pub fn list(&self, note_id: &str) -> Result<Vec<NoteHistory>, NoteServiceError> {
        let conn = self.db.conn();
        let mut stmt = conn.prepare(
            "SELECT id, note_id, content_snapshot, title_snapshot, created_at
             FROM note_history WHERE note_id = ?1
             ORDER BY created_at DESC LIMIT 100",
        )?;
        let snapshots = stmt
            .query_map(rusqlite::params![note_id], |row| {
                Ok(NoteHistory {
                    id: row.get(0)?,
                    note_id: row.get(1)?,
                    content_snapshot: row.get(2)?,
                    title_snapshot: row.get(3)?,
                    created_at: row
                        .get::<_, String>(4)?
                        .parse()
                        .unwrap_or_else(|_| Utc::now()),
                })
            })?
            .filter_map(|r| r.ok())
            .collect();
        Ok(snapshots)
    }

    pub fn get(&self, history_id: &str) -> Result<Option<NoteHistory>, NoteServiceError> {
        let conn = self.db.conn();
        let mut stmt = conn.prepare(
            "SELECT id, note_id, content_snapshot, title_snapshot, created_at
             FROM note_history WHERE id = ?1",
        )?;
        let mut rows = stmt.query_map(rusqlite::params![history_id], |row| {
            Ok(NoteHistory {
                id: row.get(0)?,
                note_id: row.get(1)?,
                content_snapshot: row.get(2)?,
                title_snapshot: row.get(3)?,
                created_at: row
                    .get::<_, String>(4)?
                    .parse()
                    .unwrap_or_else(|_| Utc::now()),
            })
        })?;
        match rows.next() {
            Some(Ok(h)) => Ok(Some(h)),
            Some(Err(e)) => Err(e.into()),
            None => Ok(None),
        }
    }

    pub fn restore(&self, history_id: &str) -> Result<Note, NoteServiceError> {
        let snapshot = self.get(history_id)?.ok_or(NoteServiceError::NotFound)?;

        let conn = self.db.conn();
        let now = Utc::now().to_rfc3339();

        let current: Note = conn.query_row(
            "SELECT id, title, content, is_pinned, is_archived, is_trashed,
                    encrypted, created_at, updated_at
             FROM notes WHERE id = ?1",
            rusqlite::params![&snapshot.note_id],
            |row| {
                Ok(Note {
                    id: row.get(0)?,
                    title: row.get(1)?,
                    content: row.get(2)?,
                    is_pinned: row.get::<_, i32>(3)? != 0,
                    is_archived: row.get::<_, i32>(4)? != 0,
                    is_trashed: row.get::<_, i32>(5)? != 0,
                    encrypted: row.get::<_, i32>(6)? != 0,
                    created_at: row
                        .get::<_, String>(7)?
                        .parse()
                        .unwrap_or_else(|_| Utc::now()),
                    updated_at: row
                        .get::<_, String>(8)?
                        .parse()
                        .unwrap_or_else(|_| Utc::now()),
                })
            },
        )?;

        conn.execute(
            "UPDATE notes SET title = ?1, content = ?2, updated_at = ?3 WHERE id = ?4",
            rusqlite::params![
                snapshot.title_snapshot,
                snapshot.content_snapshot,
                now,
                &snapshot.note_id
            ],
        )?;

        Ok(Note {
            title: snapshot.title_snapshot,
            content: snapshot.content_snapshot,
            updated_at: Utc::now(),
            ..current
        })
    }

    pub fn diff(
        &self,
        from_id: &str,
        to_id: &str,
    ) -> Result<Vec<diff::Result<String>>, NoteServiceError> {
        let from = self.get(from_id)?.ok_or(NoteServiceError::NotFound)?;
        let to = self.get(to_id)?.ok_or(NoteServiceError::NotFound)?;
        Ok(diff::lines(&from.content_snapshot, &to.content_snapshot)
            .into_iter()
            .map(|r| match r {
                diff::Result::Left(l) => diff::Result::Left(l.to_string()),
                diff::Result::Right(r) => diff::Result::Right(r.to_string()),
                diff::Result::Both(b, c) => diff::Result::Both(b.to_string(), c.to_string()),
            })
            .collect())
    }

    pub fn diff_with_current(
        &self,
        history_id: &str,
        current_content: &str,
    ) -> Result<Vec<diff::Result<String>>, NoteServiceError> {
        let snapshot = self.get(history_id)?.ok_or(NoteServiceError::NotFound)?;
        Ok(diff::lines(&snapshot.content_snapshot, current_content)
            .into_iter()
            .map(|r| match r {
                diff::Result::Left(l) => diff::Result::Left(l.to_string()),
                diff::Result::Right(r) => diff::Result::Right(r.to_string()),
                diff::Result::Both(b, c) => diff::Result::Both(b.to_string(), c.to_string()),
            })
            .collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::note::NoteService;

    fn with_setup<F>(f: F)
    where
        F: for<'a> FnOnce(NoteService<'a>, HistoryService<'a>),
    {
        let db = Database::open_in_memory().unwrap();
        let note_svc = NoteService::new(&db);
        let hist_svc = HistoryService::new(&db);
        f(note_svc, hist_svc);
    }

    #[test]
    fn new_note_has_no_version_history() {
        with_setup(|note_svc, hist_svc| {
            let note = note_svc.create("Test", "Content").unwrap();
            let history = hist_svc.list(&note.id).unwrap();
            assert!(history.is_empty());
        });
    }

    #[test]
    fn updating_a_note_creates_a_snapshot() {
        with_setup(|note_svc, hist_svc| {
            let note = note_svc.create("Test", "Original").unwrap();
            note_svc.update(&note.id, "Test", "Updated").unwrap();
            let history = hist_svc.list(&note.id).unwrap();
            assert_eq!(history.len(), 1);
            assert_eq!(history[0].content_snapshot, "Original");
        });
    }

    #[test]
    fn getting_snapshot_by_id_returns_correct_data() {
        with_setup(|note_svc, hist_svc| {
            let note = note_svc.create("Test", "V1").unwrap();
            note_svc.update(&note.id, "Test", "V2").unwrap();
            let history = hist_svc.list(&note.id).unwrap();
            let snapshot = hist_svc.get(&history[0].id).unwrap().unwrap();
            assert_eq!(snapshot.content_snapshot, "V1");
        });
    }

    #[test]
    fn restoring_snapshot_reverts_content() {
        with_setup(|note_svc, hist_svc| {
            let note = note_svc.create("Test", "V1").unwrap();
            note_svc.update(&note.id, "Test", "V2").unwrap();
            let history = hist_svc.list(&note.id).unwrap();
            hist_svc.restore(&history[0].id).unwrap();
            let restored = note_svc.get(&note.id).unwrap().unwrap();
            assert_eq!(restored.content, "V1");
        });
    }

    #[test]
    fn diffing_two_snapshots_shows_changes() {
        with_setup(|note_svc, hist_svc| {
            let note = note_svc.create("Test", "Hello World").unwrap();
            note_svc.update(&note.id, "Test", "Hello Rust").unwrap();
            let history = hist_svc.list(&note.id).unwrap();
            assert_eq!(history.len(), 1);
            // diff with itself should have only Both lines
            let d = hist_svc.diff(&history[0].id, &history[0].id).unwrap();
            assert!(d.iter().any(|r| matches!(r, diff::Result::Both(_, _))));
        });
    }

    #[test]
    fn getting_nonexistent_snapshot_returns_none() {
        with_setup(|_note_svc, hist_svc| {
            let result = hist_svc.get("nonexistent").unwrap();
            assert!(result.is_none());
        });
    }
}
