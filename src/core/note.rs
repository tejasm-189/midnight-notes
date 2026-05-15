use chrono::Utc;
use uuid::Uuid;

use crate::crypto::cipher;
use crate::storage::models::Note;
use crate::storage::Database;

use super::NoteServiceError;

fn encrypt(db: &Database, text: &str) -> String {
    if db.is_encryption_enabled() {
        let key = db.encryption_key();
        if let Ok(enc) = cipher::xchacha20_encrypt(&key, text.as_bytes()) {
            return hex::encode(enc);
        }
    }
    text.to_string()
}

fn decrypt(db: &Database, text: &str) -> String {
    if db.is_encryption_enabled() {
        let key = db.encryption_key();
        if let Ok(bytes) = hex::decode(text) {
            if let Ok(dec) = cipher::xchacha20_decrypt(&key, &bytes) {
                return String::from_utf8_lossy(&dec).to_string();
            }
        }
    }
    text.to_string()
}

pub struct NoteService<'a> {
    db: &'a Database,
}

impl<'a> NoteService<'a> {
    pub fn new(db: &'a Database) -> Self {
        Self { db }
    }

    /// Create a new note.
    pub fn create(&self, title: &str, content: &str) -> Result<Note, NoteServiceError> {
        let conn = self.db.conn();
        let id = Uuid::new_v4().to_string();
        let now = Utc::now().to_rfc3339();
        let enc_content = encrypt(self.db, content);
        let enc_title = encrypt(self.db, title);

        conn.execute(
            "INSERT INTO notes (id, title, content, created_at, updated_at) VALUES (?1, ?2, ?3, ?4, ?5)",
            params![id, enc_title, enc_content, now, now],
        )?;

        Ok(Note {
            id,
            title: title.to_string(),
            content: content.to_string(),
            is_pinned: false,
            is_archived: false,
            is_trashed: false,
            encrypted: self.db.is_encryption_enabled(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        })
    }

    /// Get a note by ID.
    pub fn get(&self, id: &str) -> Result<Option<Note>, NoteServiceError> {
        let conn = self.db.conn();
        let mut stmt = conn.prepare(
            "SELECT id, title, content, is_pinned, is_archived, is_trashed, encrypted, created_at, updated_at
             FROM notes WHERE id = ?1",
        )?;

        let mut rows = stmt.query_map(params![id], |row| self.note_from_row(row))?;

        match rows.next() {
            Some(Ok(note)) => Ok(Some(note)),
            Some(Err(e)) => Err(e.into()),
            None => Ok(None),
        }
    }

    /// Update a note's title and content.
    pub fn update(&self, id: &str, title: &str, content: &str) -> Result<(), NoteServiceError> {
        let conn = self.db.conn();
        let now = Utc::now().to_rfc3339();

        // Save history snapshot before updating (query directly on the same conn)
        let old_content: Result<String, _> = conn.query_row(
            "SELECT content FROM notes WHERE id = ?1",
            params![id],
            |row| row.get(0),
        );
        if let Ok(old) = old_content {
            let history_id = Uuid::new_v4().to_string();
            conn.execute(
                "INSERT INTO note_history (id, note_id, content_snapshot, title_snapshot, created_at) VALUES (?1, ?2, ?3, ?4, ?5)",
                params![history_id, id, old, title, now],
            )?;
        }

        conn.execute(
            "UPDATE notes SET title = ?1, content = ?2, updated_at = ?3 WHERE id = ?4",
            params![title, content, now, id],
        )?;
        Ok(())
    }

    /// Soft-delete a note (move to trash).
    pub fn trash(&self, id: &str) -> Result<(), NoteServiceError> {
        let conn = self.db.conn();
        let now = Utc::now().to_rfc3339();
        conn.execute(
            "UPDATE notes SET is_trashed = 1, updated_at = ?1 WHERE id = ?2",
            params![now, id],
        )?;
        Ok(())
    }

    /// Permanently delete a note.
    pub fn delete(&self, id: &str) -> Result<(), NoteServiceError> {
        let conn = self.db.conn();
        conn.execute("DELETE FROM notes WHERE id = ?1", params![id])?;
        Ok(())
    }

    /// Toggle pin status.
    pub fn toggle_pin(&self, id: &str) -> Result<bool, NoteServiceError> {
        let conn = self.db.conn();
        let current: bool = conn.query_row(
            "SELECT is_pinned FROM notes WHERE id = ?1",
            params![id],
            |row| row.get::<_, i32>(0).map(|v| v != 0),
        )?;
        let new = !current;
        conn.execute(
            "UPDATE notes SET is_pinned = ?1 WHERE id = ?2",
            params![new as i32, id],
        )?;
        Ok(new)
    }

    /// Toggle archive status.
    pub fn toggle_archive(&self, id: &str) -> Result<bool, NoteServiceError> {
        let conn = self.db.conn();
        let current: bool = conn.query_row(
            "SELECT is_archived FROM notes WHERE id = ?1",
            params![id],
            |row| row.get::<_, i32>(0).map(|v| v != 0),
        )?;
        let new = !current;
        conn.execute(
            "UPDATE notes SET is_archived = ?1 WHERE id = ?2",
            params![new as i32, id],
        )?;
        Ok(new)
    }

    /// Helper to construct a Note from a database row, with decryption.
    fn note_from_row(&self, row: &rusqlite::Row) -> rusqlite::Result<Note> {
        let raw_title: String = row.get(1)?;
        let raw_content: String = row.get(2)?;
        Ok(Note {
            id: row.get(0)?,
            title: decrypt(self.db, &raw_title),
            content: decrypt(self.db, &raw_content),
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
    }

    /// List all active (non-trashed, non-archived) notes, newest first.
    pub fn list_active(&self) -> Result<Vec<Note>, NoteServiceError> {
        let conn = self.db.conn();
        let mut stmt = conn.prepare(
            "SELECT id, title, content, is_pinned, is_archived, is_trashed, encrypted, created_at, updated_at
             FROM notes WHERE is_trashed = 0 AND is_archived = 0
             ORDER BY is_pinned DESC, updated_at DESC",
        )?;

        let notes = stmt
            .query_map([], |row| self.note_from_row(row))?
            .filter_map(|r| r.ok())
            .collect();

        Ok(notes)
    }

    /// List archived notes.
    pub fn list_archived(&self) -> Result<Vec<Note>, NoteServiceError> {
        let conn = self.db.conn();
        let mut stmt = conn.prepare(
            "SELECT id, title, content, is_pinned, is_archived, is_trashed, encrypted, created_at, updated_at
             FROM notes WHERE is_archived = 1 AND is_trashed = 0
             ORDER BY updated_at DESC",
        )?;
        let notes = stmt
            .query_map([], |row| self.note_from_row(row))?
            .filter_map(|r| r.ok())
            .collect();
        Ok(notes)
    }

    /// Restore a trashed or archived note (clear both flags).
    pub fn restore(&self, id: &str) -> Result<(), NoteServiceError> {
        let conn = self.db.conn();
        let now = Utc::now().to_rfc3339();
        conn.execute(
            "UPDATE notes SET is_trashed = 0, is_archived = 0, updated_at = ?1 WHERE id = ?2",
            params![now, id],
        )?;
        Ok(())
    }

    /// Permanently delete a trashed note.
    pub fn delete_permanently(&self, id: &str) -> Result<(), NoteServiceError> {
        let conn = self.db.conn();
        conn.execute("DELETE FROM note_tags WHERE note_id = ?1", params![id])?;
        conn.execute(
            "DELETE FROM backlinks WHERE source_note_id = ?1 OR target_note_id = ?1",
            params![id],
        )?;
        conn.execute("DELETE FROM note_history WHERE note_id = ?1", params![id])?;
        conn.execute("DELETE FROM notes WHERE id = ?1", params![id])?;
        Ok(())
    }

    /// List trashed notes.
    pub fn list_trashed(&self) -> Result<Vec<Note>, NoteServiceError> {
        let conn = self.db.conn();
        let mut stmt = conn.prepare(
            "SELECT id, title, content, is_pinned, is_archived, is_trashed, encrypted, created_at, updated_at
             FROM notes WHERE is_trashed = 1
             ORDER BY updated_at DESC",
        )?;

        let notes = stmt
            .query_map([], |row| self.note_from_row(row))?
            .filter_map(|r| r.ok())
            .collect();

        Ok(notes)
    }

    /// Search notes using FTS5.
    pub fn search(&self, query: &str) -> Result<Vec<(Note, f64)>, NoteServiceError> {
        let conn = self.db.conn();
        let mut stmt = conn.prepare(
            "SELECT n.id, n.title, n.content, n.is_pinned, n.is_archived, n.is_trashed,
                    n.encrypted, n.created_at, n.updated_at, f.rank
             FROM notes_fts f
             JOIN notes n ON n.rowid = f.rowid
             WHERE notes_fts MATCH ?1
             ORDER BY f.rank
             LIMIT 50",
        )?;

        let results = stmt
            .query_map(params![query], |row| {
                let note = Note {
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
                };
                let rank: f64 = row.get(9)?;
                Ok((note, rank))
            })?
            .filter_map(|r| r.ok())
            .collect();

        Ok(results)
    }
}

use rusqlite::params;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::storage::Database;

    fn with_service<F>(f: F)
    where
        F: for<'a> FnOnce(NoteService<'a>),
    {
        let db = Database::open_in_memory().unwrap();
        let service = NoteService::new(&db);
        f(service);
    }

    #[test]
    fn test_create_note() {
        with_service(|service| {
            let note = service.create("Test Title", "Test Content").unwrap();
            assert_eq!(note.title, "Test Title");
            assert_eq!(note.content, "Test Content");
        });
    }

    #[test]
    fn test_get_note() {
        with_service(|service| {
            let created = service.create("Get Test", "Content").unwrap();
            let fetched = service.get(&created.id).unwrap().unwrap();
            assert_eq!(fetched.id, created.id);
        });
    }

    #[test]
    fn test_get_nonexistent() {
        with_service(|service| {
            let result = service.get("nonexistent-id").unwrap();
            assert!(result.is_none());
        });
    }

    #[test]
    fn test_update_note() {
        with_service(|service| {
            let note = service.create("Original", "Original content").unwrap();
            service
                .update(&note.id, "Updated", "Updated content")
                .unwrap();
            let updated = service.get(&note.id).unwrap().unwrap();
            assert_eq!(updated.title, "Updated");
            assert_eq!(updated.content, "Updated content");
        });
    }

    #[test]
    fn test_trash_and_list() {
        with_service(|service| {
            let note = service.create("To Trash", "Content").unwrap();
            service.trash(&note.id).unwrap();
            let active = service.list_active().unwrap();
            assert!(!active.iter().any(|n| n.id == note.id));
            let trashed = service.list_trashed().unwrap();
            assert!(trashed.iter().any(|n| n.id == note.id));
        });
    }

    #[test]
    fn test_delete_note() {
        with_service(|service| {
            let note = service.create("To Delete", "Content").unwrap();
            service.delete(&note.id).unwrap();
            let result = service.get(&note.id).unwrap();
            assert!(result.is_none());
        });
    }

    #[test]
    fn test_toggle_pin() {
        with_service(|service| {
            let note = service.create("Pin Test", "Content").unwrap();
            assert!(!note.is_pinned);
            let new_state = service.toggle_pin(&note.id).unwrap();
            assert!(new_state);
            let fetched = service.get(&note.id).unwrap().unwrap();
            assert!(fetched.is_pinned);
        });
    }

    #[test]
    fn test_list_active_ordering() {
        with_service(|service| {
            let n1 = service.create("Alpha", "Content").unwrap();
            let _ = service.create("Beta", "Content").unwrap();
            service.toggle_pin(&n1.id).unwrap();
            let active = service.list_active().unwrap();
            assert_eq!(active[0].id, n1.id);
        });
    }

    #[test]
    fn test_search() {
        with_service(|service| {
            service
                .create("Rust Programming", "Learn about ownership")
                .unwrap();
            service.create("Python Scripts", "Automation").unwrap();
            let results = service.search("Rust").unwrap();
            assert!(!results.is_empty());
            assert!(results.iter().any(|(n, _)| n.title.contains("Rust")));
        });
    }
}
