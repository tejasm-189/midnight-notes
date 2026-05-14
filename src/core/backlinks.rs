use chrono::Utc;
use regex::Regex;

use crate::storage::models::Note;
use crate::storage::Database;

use super::NoteServiceError;

pub struct BacklinkService<'a> {
    db: &'a Database,
}

impl<'a> BacklinkService<'a> {
    pub fn new(db: &'a Database) -> Self {
        Self { db }
    }

    /// Extract all [[Note Title]] references from content.
    pub fn extract_links(content: &str) -> Vec<String> {
        let re = Regex::new(r"\[\[([^\]]+)\]\]").unwrap();
        re.captures_iter(content)
            .map(|c| c[1].trim().to_string())
            .filter(|s| !s.is_empty())
            .collect()
    }

    /// Resolve a note title to a note ID via FTS5 or exact title match (public API).
    pub fn resolve_title(&self, title: &str) -> Result<Option<String>, NoteServiceError> {
        let conn = self.db.conn();
        Self::resolve_title_inner(&conn, title)
    }

    /// Resolve internally — takes a pre-acquired connection (no deadlock).
    fn resolve_title_inner(
        conn: &rusqlite::Connection,
        title: &str,
    ) -> Result<Option<String>, NoteServiceError> {
        let exact: Result<String, _> = conn.query_row(
            "SELECT id FROM notes WHERE title = ?1 LIMIT 1",
            rusqlite::params![title],
            |row| row.get(0),
        );
        if let Ok(id) = exact {
            return Ok(Some(id));
        }

        let query = format!("\"{}\"*", title.replace('"', "\"\""));
        let fts: Result<String, _> = conn.query_row(
            "SELECT n.id FROM notes_fts f JOIN notes n ON n.rowid = f.rowid
             WHERE notes_fts MATCH ?1 ORDER BY rank LIMIT 1",
            rusqlite::params![query],
            |row| row.get(0),
        );
        fts.map(Some).or(Ok(None))
    }

    /// Refresh all backlinks for a note by scanning its content.
    pub fn refresh(&self, note_id: &str, content: &str) -> Result<(), NoteServiceError> {
        let conn = self.db.conn();

        conn.execute(
            "DELETE FROM backlinks WHERE source_note_id = ?1",
            rusqlite::params![note_id],
        )?;

        let links = Self::extract_links(content);
        for title in links {
            if let Some(target_id) = Self::resolve_title_inner(&conn, &title)? {
                let now = Utc::now().to_rfc3339();
                conn.execute(
                    "INSERT OR IGNORE INTO backlinks (source_note_id, target_note_id, created_at) VALUES (?1, ?2, ?3)",
                    rusqlite::params![note_id, target_id, now],
                )?;
            }
        }

        Ok(())
    }

    pub fn get_linked_mentions(&self, note_id: &str) -> Result<Vec<Note>, NoteServiceError> {
        let conn = self.db.conn();
        let mut stmt = conn.prepare(
            "SELECT n.id, n.title, n.content, n.is_pinned, n.is_archived, n.is_trashed,
                    n.encrypted, n.created_at, n.updated_at
             FROM notes n JOIN backlinks b ON n.id = b.source_note_id
             WHERE b.target_note_id = ?1
             ORDER BY b.created_at DESC",
        )?;

        let notes = stmt
            .query_map(rusqlite::params![note_id], row_to_note)?
            .filter_map(|r| r.ok())
            .collect();

        Ok(notes)
    }

    pub fn get_outgoing_links(&self, note_id: &str) -> Result<Vec<Note>, NoteServiceError> {
        let conn = self.db.conn();
        let mut stmt = conn.prepare(
            "SELECT n.id, n.title, n.content, n.is_pinned, n.is_archived, n.is_trashed,
                    n.encrypted, n.created_at, n.updated_at
             FROM notes n JOIN backlinks b ON n.id = b.target_note_id
             WHERE b.source_note_id = ?1
             ORDER BY b.created_at DESC",
        )?;

        let notes = stmt
            .query_map(rusqlite::params![note_id], row_to_note)?
            .filter_map(|r| r.ok())
            .collect();

        Ok(notes)
    }
}

fn row_to_note(row: &rusqlite::Row) -> rusqlite::Result<Note> {
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::note::NoteService;

    fn with_services<F>(f: F)
    where
        F: for<'a> FnOnce(NoteService<'a>, BacklinkService<'a>),
    {
        let db = Database::open_in_memory().unwrap();
        let note_svc = NoteService::new(&db);
        let bl_svc = BacklinkService::new(&db);
        f(note_svc, bl_svc);
    }

    #[test]
    fn test_extract_links() {
        let links = BacklinkService::extract_links("See [[Data Models]] and [[API Design]].");
        assert_eq!(links, vec!["Data Models", "API Design"]);
    }

    #[test]
    fn test_extract_links_no_matches() {
        let links = BacklinkService::extract_links("No links here.");
        assert!(links.is_empty());
    }

    #[test]
    fn test_extract_links_empty_content() {
        let links = BacklinkService::extract_links("");
        assert!(links.is_empty());
    }

    #[test]
    fn test_extract_links_trims_whitespace() {
        let links = BacklinkService::extract_links("[[  Spaced Out  ]]");
        assert_eq!(links, vec!["Spaced Out"]);
    }

    #[test]
    fn test_refresh_and_get_mentions() {
        with_services(|note_svc, bl_svc| {
            let target = note_svc.create("Target Note", "Content").unwrap();
            let source = note_svc
                .create("Source Note", &format!("See [[Target Note]] for details."))
                .unwrap();
            bl_svc.refresh(&source.id, &source.content).unwrap();
            let mentions = bl_svc.get_linked_mentions(&target.id).unwrap();
            assert_eq!(mentions.len(), 1);
            assert_eq!(mentions[0].id, source.id);
        });
    }

    #[test]
    fn test_outgoing_links() {
        with_services(|note_svc, bl_svc| {
            let target = note_svc.create("Target", "Content").unwrap();
            let source = note_svc
                .create("Source", &format!("See [[Target]]."))
                .unwrap();
            bl_svc.refresh(&source.id, &source.content).unwrap();
            let outgoing = bl_svc.get_outgoing_links(&source.id).unwrap();
            assert_eq!(outgoing.len(), 1);
            assert_eq!(outgoing[0].id, target.id);
        });
    }

    #[test]
    fn test_refresh_replaces_old_links() {
        with_services(|note_svc, bl_svc| {
            let t1 = note_svc.create("Target1", "C").unwrap();
            let t2 = note_svc.create("Target2", "C").unwrap();
            let src = note_svc.create("Source", "[[Target1]]").unwrap();
            bl_svc.refresh(&src.id, &src.content).unwrap();
            assert_eq!(bl_svc.get_linked_mentions(&t1.id).unwrap().len(), 1);
            bl_svc.refresh(&src.id, "[[Target2]]").unwrap();
            assert_eq!(bl_svc.get_linked_mentions(&t1.id).unwrap().len(), 0);
            assert_eq!(bl_svc.get_linked_mentions(&t2.id).unwrap().len(), 1);
        });
    }

    #[test]
    fn test_resolve_title_exact() {
        with_services(|note_svc, bl_svc| {
            let note = note_svc.create("Exact Title", "Content").unwrap();
            let found = bl_svc.resolve_title("Exact Title").unwrap().unwrap();
            assert_eq!(found, note.id);
        });
    }

    #[test]
    fn test_resolve_title_nonexistent() {
        with_services(|_note_svc, bl_svc| {
            let result = bl_svc.resolve_title("Nonexistent Note").unwrap();
            assert!(result.is_none());
        });
    }
}
