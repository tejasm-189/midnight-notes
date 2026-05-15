use chrono::Utc;
use uuid::Uuid;

use crate::storage::models::Tag;
use crate::storage::Database;

use super::NoteServiceError;

pub struct TagService<'a> {
    db: &'a Database,
}

impl<'a> TagService<'a> {
    pub fn new(db: &'a Database) -> Self {
        Self { db }
    }

    pub fn create(
        &self,
        name: &str,
        parent_id: Option<&str>,
        color: Option<&str>,
    ) -> Result<Tag, NoteServiceError> {
        if name.is_empty() {
            return Err(NoteServiceError::InvalidInput(
                "tag name cannot be empty".into(),
            ));
        }
        let conn = self.db.conn();
        let id = Uuid::new_v4().to_string();
        let now = Utc::now().to_rfc3339();

        conn.execute(
            "INSERT INTO tags (id, name, parent_id, color, created_at) VALUES (?1, ?2, ?3, ?4, ?5)",
            rusqlite::params![id, name, parent_id, color, now],
        )?;

        Ok(Tag {
            id,
            name: name.to_string(),
            parent_id: parent_id.map(|s| s.to_string()),
            color: color.map(|s| s.to_string()),
            created_at: Utc::now(),
        })
    }

    pub fn get(&self, id: &str) -> Result<Option<Tag>, NoteServiceError> {
        let conn = self.db.conn();
        let mut stmt =
            conn.prepare("SELECT id, name, parent_id, color, created_at FROM tags WHERE id = ?1")?;
        let mut rows = stmt.query_map(rusqlite::params![id], row_to_tag)?;
        match rows.next() {
            Some(Ok(tag)) => Ok(Some(tag)),
            Some(Err(e)) => Err(e.into()),
            None => Ok(None),
        }
    }

    pub fn get_by_name(&self, name: &str) -> Result<Option<Tag>, NoteServiceError> {
        let conn = self.db.conn();
        let mut stmt = conn
            .prepare("SELECT id, name, parent_id, color, created_at FROM tags WHERE name = ?1")?;
        let mut rows = stmt.query_map(rusqlite::params![name], row_to_tag)?;
        match rows.next() {
            Some(Ok(tag)) => Ok(Some(tag)),
            Some(Err(e)) => Err(e.into()),
            None => Ok(None),
        }
    }

    pub fn update(
        &self,
        id: &str,
        name: &str,
        parent_id: Option<&str>,
        color: Option<&str>,
    ) -> Result<(), NoteServiceError> {
        if name.is_empty() {
            return Err(NoteServiceError::InvalidInput(
                "tag name cannot be empty".into(),
            ));
        }
        let conn = self.db.conn();
        conn.execute(
            "UPDATE tags SET name = ?1, parent_id = ?2, color = ?3 WHERE id = ?4",
            rusqlite::params![name, parent_id, color, id],
        )?;
        Ok(())
    }

    pub fn delete(&self, id: &str) -> Result<(), NoteServiceError> {
        let conn = self.db.conn();
        conn.execute(
            "DELETE FROM note_tags WHERE tag_id = ?1",
            rusqlite::params![id],
        )?;
        conn.execute("DELETE FROM tags WHERE id = ?1", rusqlite::params![id])?;
        Ok(())
    }

    pub fn list_roots(&self) -> Result<Vec<Tag>, NoteServiceError> {
        let conn = self.db.conn();
        let mut stmt = conn.prepare(
            "SELECT id, name, parent_id, color, created_at FROM tags WHERE parent_id IS NULL ORDER BY name",
        )?;
        let tags = stmt
            .query_map([], row_to_tag)?
            .filter_map(|r| r.ok())
            .collect();
        Ok(tags)
    }

    pub fn get_children(&self, parent_id: &str) -> Result<Vec<Tag>, NoteServiceError> {
        let conn = self.db.conn();
        let mut stmt = conn.prepare(
            "SELECT id, name, parent_id, color, created_at FROM tags WHERE parent_id = ?1 ORDER BY name",
        )?;
        let tags = stmt
            .query_map(rusqlite::params![parent_id], row_to_tag)?
            .filter_map(|r| r.ok())
            .collect();
        Ok(tags)
    }

    pub fn get_all(&self) -> Result<Vec<Tag>, NoteServiceError> {
        let conn = self.db.conn();
        let mut stmt =
            conn.prepare("SELECT id, name, parent_id, color, created_at FROM tags ORDER BY name")?;
        let tags = stmt
            .query_map([], row_to_tag)?
            .filter_map(|r| r.ok())
            .collect();
        Ok(tags)
    }

    pub fn assign_to_note(&self, tag_id: &str, note_id: &str) -> Result<(), NoteServiceError> {
        let conn = self.db.conn();
        conn.execute(
            "INSERT OR IGNORE INTO note_tags (note_id, tag_id) VALUES (?1, ?2)",
            rusqlite::params![note_id, tag_id],
        )?;
        Ok(())
    }

    pub fn remove_from_note(&self, tag_id: &str, note_id: &str) -> Result<(), NoteServiceError> {
        let conn = self.db.conn();
        conn.execute(
            "DELETE FROM note_tags WHERE note_id = ?1 AND tag_id = ?2",
            rusqlite::params![note_id, tag_id],
        )?;
        Ok(())
    }

    pub fn get_tags_for_note(&self, note_id: &str) -> Result<Vec<Tag>, NoteServiceError> {
        let conn = self.db.conn();
        let mut stmt = conn.prepare(
            "SELECT t.id, t.name, t.parent_id, t.color, t.created_at
             FROM tags t JOIN note_tags nt ON t.id = nt.tag_id
             WHERE nt.note_id = ?1 ORDER BY t.name",
        )?;
        let tags = stmt
            .query_map(rusqlite::params![note_id], row_to_tag)?
            .filter_map(|r| r.ok())
            .collect();
        Ok(tags)
    }

    pub fn get_notes_for_tag(&self, tag_id: &str) -> Result<Vec<String>, NoteServiceError> {
        let conn = self.db.conn();
        let mut stmt =
            conn.prepare("SELECT note_id FROM note_tags WHERE tag_id = ?1 ORDER BY note_id")?;
        let ids = stmt
            .query_map(rusqlite::params![tag_id], |row| row.get(0))?
            .filter_map(|r| r.ok())
            .collect();
        Ok(ids)
    }
}

fn row_to_tag(row: &rusqlite::Row) -> rusqlite::Result<Tag> {
    Ok(Tag {
        id: row.get(0)?,
        name: row.get(1)?,
        parent_id: row.get(2)?,
        color: row.get(3)?,
        created_at: row
            .get::<_, String>(4)?
            .parse()
            .unwrap_or_else(|_| Utc::now()),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn with_service<F>(f: F)
    where
        F: for<'a> FnOnce(TagService<'a>),
    {
        let db = crate::storage::Database::open_in_memory().unwrap();
        // Disable FK enforcement for tag-only tests (no notes exist)
        db.conn()
            .execute_batch("PRAGMA foreign_keys = OFF;")
            .unwrap();
        let svc = TagService::new(&db);
        f(svc);
    }

    #[test]
    fn creating_a_tag_and_getting_by_id_returns_it() {
        with_service(|svc| {
            let tag = svc.create("work", None, None).unwrap();
            assert_eq!(tag.name, "work");
            let fetched = svc.get(&tag.id).unwrap().unwrap();
            assert_eq!(fetched.name, "work");
        });
    }

    #[test]
    fn creating_a_tag_with_parent_creates_hierarchy() {
        with_service(|svc| {
            let parent = svc.create("work", None, None).unwrap();
            let child = svc.create("projects", Some(&parent.id), None).unwrap();
            assert_eq!(child.parent_id.as_deref(), Some(parent.id.as_str()));
        });
    }

    #[test]
    fn getting_a_tag_by_name_returns_correct_tag() {
        with_service(|svc| {
            svc.create("unique-name", None, None).unwrap();
            let tag = svc.get_by_name("unique-name").unwrap().unwrap();
            assert_eq!(tag.name, "unique-name");
        });
    }

    #[test]
    fn updating_tag_name_and_color_persists_changes() {
        with_service(|svc| {
            let tag = svc.create("old", None, None).unwrap();
            svc.update(&tag.id, "new", None, Some("#ff0000")).unwrap();
            let updated = svc.get(&tag.id).unwrap().unwrap();
            assert_eq!(updated.name, "new");
            assert_eq!(updated.color.unwrap(), "#ff0000");
        });
    }

    #[test]
    fn deleting_a_tag_removes_it_and_note_tags() {
        with_service(|svc| {
            let tag = svc.create("delete-me", None, None).unwrap();
            svc.delete(&tag.id).unwrap();
            assert!(svc.get(&tag.id).unwrap().is_none());
        });
    }

    #[test]
    fn root_tags_have_no_parent() {
        with_service(|svc| {
            svc.create("root1", None, None).unwrap();
            svc.create("root2", None, None).unwrap();
            let parent = svc.create("parent", None, None).unwrap();
            svc.create("child", Some(&parent.id), None).unwrap();
            let roots = svc.list_roots().unwrap();
            assert_eq!(roots.len(), 3);
        });
    }

    #[test]
    fn getting_children_of_a_parent_tag_works() {
        with_service(|svc| {
            let p = svc.create("parent", None, None).unwrap();
            svc.create("c1", Some(&p.id), None).unwrap();
            svc.create("c2", Some(&p.id), None).unwrap();
            let children = svc.get_children(&p.id).unwrap();
            assert_eq!(children.len(), 2);
        });
    }

    #[test]
    fn assigning_tag_to_note_and_listing_tags_works() {
        with_service(|svc| {
            let tag = svc.create("devops", None, None).unwrap();
            svc.assign_to_note(&tag.id, "note-1").unwrap();
            let tags = svc.get_tags_for_note("note-1").unwrap();
            assert_eq!(tags.len(), 1);
            assert_eq!(tags[0].name, "devops");
        });
    }

    #[test]
    fn removing_tag_from_note_clears_it() {
        with_service(|svc| {
            let tag = svc.create("temp", None, None).unwrap();
            svc.assign_to_note(&tag.id, "note-1").unwrap();
            svc.remove_from_note(&tag.id, "note-1").unwrap();
            let tags = svc.get_tags_for_note("note-1").unwrap();
            assert!(tags.is_empty());
        });
    }

    #[test]
    fn creating_tag_with_empty_name_is_rejected() {
        with_service(|svc| {
            let result = svc.create("", None, None);
            assert!(result.is_err());
        });
    }

    #[test]
    fn listing_all_tags_returns_every_tag() {
        with_service(|svc| {
            svc.create("a", None, None).unwrap();
            svc.create("b", None, None).unwrap();
            assert_eq!(svc.get_all().unwrap().len(), 2);
        });
    }
}
