use serde::{Deserialize, Serialize};

use crate::storage::Database;

use super::NoteServiceError;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SearchFilters {
    pub tag: Option<String>,
    pub has_todo: Option<bool>,
    pub is_pinned: Option<bool>,
    pub is_archived: Option<bool>,
    pub path: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    pub note_id: String,
    pub title: String,
    pub snippet: String,
    pub rank: f64,
    pub updated_at: String,
}

pub struct SearchService<'a> {
    db: &'a Database,
}

impl<'a> SearchService<'a> {
    pub fn new(db: &'a Database) -> Self {
        Self { db }
    }

    pub fn parse_query(input: &str) -> (String, SearchFilters) {
        let mut filters = SearchFilters::default();
        let mut fts_parts: Vec<String> = vec![];
        let mut remaining = input.to_string();

        if let Some(caps) = regex::Regex::new(r"\btag:(\S+)")
            .unwrap()
            .captures(&remaining)
        {
            filters.tag = Some(caps[1].to_string());
            remaining = remaining
                .replace(caps.get(0).unwrap().as_str(), "")
                .trim()
                .to_string();
        }

        if remaining.contains("has:todo") {
            filters.has_todo = Some(true);
            remaining = remaining.replace("has:todo", "").trim().to_string();
        }

        if let Some(caps) = regex::Regex::new(r"\bpath:(\S+)")
            .unwrap()
            .captures(&remaining)
        {
            filters.path = Some(caps[1].to_string());
            remaining = remaining
                .replace(caps.get(0).unwrap().as_str(), "")
                .trim()
                .to_string();
        }

        if !remaining.is_empty() {
            fts_parts.push(remaining);
        }

        (fts_parts.join(" "), filters)
    }

    pub fn search(&self, query: &str) -> Result<Vec<SearchResult>, NoteServiceError> {
        let (fts_query, filters) = Self::parse_query(query);
        let conn = self.db.conn();

        let mut sql = String::from(
            "SELECT n.id, n.title, n.content, n.updated_at
             FROM notes n WHERE n.is_trashed = 0",
        );
        let mut params: Vec<Box<dyn rusqlite::types::ToSql>> = vec![];

        if let Some(tag) = &filters.tag {
            sql.push_str(&format!(
                " AND n.id IN (SELECT note_id FROM note_tags nt JOIN tags t ON t.id = nt.tag_id WHERE t.name = ?{})",
                params.len() + 1,
            ));
            params.push(Box::new(tag.clone()));
        }

        if filters.has_todo == Some(true) {
            sql.push_str(" AND n.content LIKE '%[ ]%'");
        }

        if !fts_query.is_empty() {
            sql.push_str(&format!(
                " AND n.rowid IN (SELECT rowid FROM notes_fts WHERE notes_fts MATCH ?{})",
                params.len() + 1,
            ));
            params.push(Box::new(fts_query));
        }

        if let Some(pinned) = filters.is_pinned {
            sql.push_str(&format!(" AND n.is_pinned = ?{}", params.len() + 1));
            params.push(Box::new(pinned as i32));
        }

        sql.push_str(" ORDER BY n.is_pinned DESC, n.updated_at DESC LIMIT 50");

        let mut stmt = conn.prepare(&sql)?;
        let param_refs: Vec<&dyn rusqlite::types::ToSql> =
            params.iter().map(|p| p.as_ref()).collect();
        let results = stmt.query_map(param_refs.as_slice(), |row| {
            Ok(SearchResult {
                note_id: row.get(0)?,
                title: row.get(1)?,
                snippet: row.get::<_, String>(2)?.chars().take(200).collect(),
                rank: 0.0,
                updated_at: row.get(3)?,
            })
        })?;

        results.filter_map(|r| r.ok()).collect::<Vec<_>>().pipe(Ok)
    }

    // --- Smart Views ---

    pub fn save_smart_view(&self, name: &str, query: &str) -> Result<(), NoteServiceError> {
        let conn = self.db.conn();
        conn.execute(
            "INSERT OR REPLACE INTO meta (key, value) VALUES (?1, ?2)",
            rusqlite::params![format!("smart_view:{}", name), query],
        )?;
        Ok(())
    }

    pub fn list_smart_views(&self) -> Result<Vec<(String, String)>, NoteServiceError> {
        let conn = self.db.conn();
        let mut stmt =
            conn.prepare("SELECT key, value FROM meta WHERE key LIKE 'smart_view:%' ORDER BY key")?;
        let views = stmt
            .query_map([], |row| {
                let key: String = row.get(0)?;
                let value: String = row.get(1)?;
                let name = key.strip_prefix("smart_view:").unwrap_or(&key).to_string();
                Ok((name, value))
            })?
            .filter_map(|r| r.ok())
            .collect();
        Ok(views)
    }

    pub fn delete_smart_view(&self, name: &str) -> Result<(), NoteServiceError> {
        let conn = self.db.conn();
        conn.execute(
            "DELETE FROM meta WHERE key = ?1",
            rusqlite::params![format!("smart_view:{}", name)],
        )?;
        Ok(())
    }

    pub fn execute_smart_view(&self, name: &str) -> Result<Vec<SearchResult>, NoteServiceError> {
        let query: String = {
            let conn = self.db.conn();
            conn.query_row(
                "SELECT value FROM meta WHERE key = ?1",
                rusqlite::params![format!("smart_view:{}", name)],
                |row| row.get(0),
            )?
        };
        self.search(&query)
    }
}

trait Pipe: Sized {
    fn pipe<F, R>(self, f: F) -> R
    where
        F: FnOnce(Self) -> R;
}

impl<T> Pipe for T {
    fn pipe<F, R>(self, f: F) -> R
    where
        F: FnOnce(Self) -> R,
    {
        f(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::note::NoteService;
    use crate::storage::Database;

    fn with_setup<F>(f: F)
    where
        F: for<'a> FnOnce(NoteService<'a>, SearchService<'a>),
    {
        let db = Database::open_in_memory().unwrap();
        let note_svc = NoteService::new(&db);
        let search_svc = SearchService::new(&db);
        f(note_svc, search_svc);
    }

    #[test]
    fn test_parse_query_plain() {
        let (fts, filters) = SearchService::parse_query("hello world");
        assert_eq!(fts, "hello world");
        assert!(filters.tag.is_none());
    }

    #[test]
    fn test_parse_query_with_tag() {
        let (fts, filters) = SearchService::parse_query("tag:work database");
        assert_eq!(filters.tag.unwrap(), "work");
        assert_eq!(fts, "database");
    }

    #[test]
    fn test_parse_query_has_todo() {
        let (fts, filters) = SearchService::parse_query("meeting has:todo");
        assert_eq!(filters.has_todo, Some(true));
        assert_eq!(fts, "meeting");
    }

    #[test]
    fn test_parse_query_path() {
        let (fts, filters) = SearchService::parse_query("path:docs/ architecture");
        assert_eq!(filters.path.unwrap(), "docs/");
        assert_eq!(fts, "architecture");
    }

    #[test]
    fn test_parse_query_all_filters() {
        let (fts, filters) = SearchService::parse_query("tag:backend path:src/ has:todo rust");
        assert_eq!(filters.tag.unwrap(), "backend");
        assert_eq!(filters.path.unwrap(), "src/");
        assert_eq!(filters.has_todo, Some(true));
        assert_eq!(fts, "rust");
    }

    #[test]
    fn test_search_basic() {
        with_setup(|note_svc, search_svc| {
            note_svc
                .create("Rust Notes", "Learn about ownership")
                .unwrap();
            note_svc
                .create("Python Notes", "Learn about decorators")
                .unwrap();
            let results = search_svc.search("Rust").unwrap();
            assert_eq!(results.len(), 1);
            assert!(results[0].title.contains("Rust"));
        });
    }

    #[test]
    fn test_search_no_results() {
        with_setup(|note_svc, search_svc| {
            note_svc.create("Test Note", "Content").unwrap();
            let results = search_svc.search("nonexistent").unwrap();
            assert!(results.is_empty());
        });
    }

    #[test]
    fn test_smart_view_crud() {
        with_setup(|_note_svc, search_svc| {
            search_svc.save_smart_view("TODO", "has:todo").unwrap();
            search_svc
                .save_smart_view("Architecture", "tag:architecture")
                .unwrap();
            let views = search_svc.list_smart_views().unwrap();
            assert_eq!(views.len(), 2);
            search_svc.delete_smart_view("TODO").unwrap();
            let views = search_svc.list_smart_views().unwrap();
            assert_eq!(views.len(), 1);
        });
    }

    #[test]
    fn test_smart_view_execute() {
        with_setup(|note_svc, search_svc| {
            note_svc
                .create("Architecture Overview", "System design document")
                .unwrap();
            search_svc.save_smart_view("Arch", "architecture").unwrap();
            let results = search_svc.execute_smart_view("Arch").unwrap();
            assert!(!results.is_empty());
        });
    }
}
