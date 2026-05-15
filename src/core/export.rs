use std::io::{Read, Write};
use std::path::Path;
use uuid::Uuid;

use crate::crypto::cipher;
use crate::crypto::keychain;
use crate::storage::models::Note;
use crate::storage::Database;

use super::NoteServiceError;

pub struct ExportService<'a> {
    db: &'a Database,
}

impl<'a> ExportService<'a> {
    pub fn new(db: &'a Database) -> Self {
        Self { db }
    }

    pub fn export_notes(
        &self,
        note_ids: &[&str],
        output_path: &Path,
        password: &str,
    ) -> Result<(), NoteServiceError> {
        let file = std::fs::File::create(output_path)?;
        let mut zip = zip::ZipWriter::new(file);
        let options = zip::write::FileOptions::<()>::default()
            .compression_method(zip::CompressionMethod::Deflated);

        let salt = keychain::generate_salt();
        let encryption_key = keychain::derive_key(password, &salt)
            .map_err(|e| NoteServiceError::InvalidInput(e.to_string()))?;

        // Write metadata first so import can read key before decrypting entries
        zip.start_file("_metadata.json", options)?;
        let meta = serde_json::json!({
            "version": 1,
            "algorithm": "XChaCha20-Poly1305",
            "key_derivation": "Argon2id",
            "salt": hex::encode(salt),
            "note_count": note_ids.len(),
            "exported_at": chrono::Utc::now().to_rfc3339(),
        });
        zip.write_all(meta.to_string().as_bytes())?;

        // Write encrypted notes
        for note_id in note_ids {
            let note = self.get_note(note_id)?;
            let json = serde_json::to_string(&note)?;
            let encrypted = cipher::xchacha20_encrypt(&encryption_key, json.as_bytes())
                .map_err(|e| NoteServiceError::InvalidInput(e.to_string()))?;

            zip.start_file(format!("{}.enc", note_id), options)?;
            zip.write_all(&encrypted)?;
        }

        zip.finish()?;
        Ok(())
    }

    pub fn import_notes(
        &self,
        input_path: &Path,
        password: &str,
    ) -> Result<Vec<Note>, NoteServiceError> {
        let file = std::fs::File::open(input_path)?;
        let mut archive = zip::ZipArchive::new(file)?;
        let mut encryption_key: Option<[u8; 32]> = None;
        let mut imported = Vec::new();

        for i in 0..archive.len() {
            let mut entry = archive.by_index(i)?;
            let name = entry.name().to_string();

            if name == "_metadata.json" {
                let mut meta_str = String::new();
                entry.read_to_string(&mut meta_str)?;
                let meta: serde_json::Value = serde_json::from_str(&meta_str)?;
                if let Some(salt_hex) = meta["salt"].as_str() {
                    let salt = hex::decode(salt_hex).map_err(|_| {
                        NoteServiceError::InvalidInput("invalid salt in metadata".into())
                    })?;
                    let mut salt_arr = [0u8; 16];
                    if salt.len() == 16 {
                        salt_arr.copy_from_slice(&salt);
                    }
                    encryption_key = Some(
                        keychain::derive_key(password, &salt_arr)
                            .map_err(|e| NoteServiceError::InvalidInput(e.to_string()))?,
                    );
                }
                continue;
            }

            if !name.ends_with(".enc") {
                continue;
            }

            let key = encryption_key.ok_or_else(|| {
                NoteServiceError::InvalidInput("no metadata found in archive".into())
            })?;

            let mut encrypted = Vec::new();
            entry.read_to_end(&mut encrypted)?;

            let decrypted = cipher::xchacha20_decrypt(&key, &encrypted)
                .map_err(|e| NoteServiceError::InvalidInput(e.to_string()))?;

            let note_data: serde_json::Value = serde_json::from_slice(&decrypted)?;
            let note = self.create_note_from_import(&note_data)?;
            imported.push(note);
        }

        Ok(imported)
    }

    fn get_note(&self, note_id: &str) -> Result<Note, NoteServiceError> {
        let conn = self.db.conn();
        conn.query_row(
            "SELECT id, title, content, is_pinned, is_archived, is_trashed,
                    encrypted, created_at, updated_at
             FROM notes WHERE id = ?1",
            rusqlite::params![note_id],
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
                        .unwrap_or_else(|_| chrono::Utc::now()),
                    updated_at: row
                        .get::<_, String>(8)?
                        .parse()
                        .unwrap_or_else(|_| chrono::Utc::now()),
                })
            },
        )
        .map_err(Into::into)
    }

    fn create_note_from_import(&self, data: &serde_json::Value) -> Result<Note, NoteServiceError> {
        let conn = self.db.conn();
        let id = data["id"]
            .as_str()
            .map(|s| s.to_string())
            .unwrap_or_else(|| Uuid::new_v4().to_string());
        let title = data["title"].as_str().unwrap_or("Imported Note");
        let content = data["content"].as_str().unwrap_or("");
        let now = chrono::Utc::now().to_rfc3339();

        conn.execute(
            "INSERT OR IGNORE INTO notes (id, title, content, created_at, updated_at) VALUES (?1, ?2, ?3, ?4, ?5)",
            rusqlite::params![id, title, content, now, now],
        )?;

        Ok(Note {
            id,
            title: title.to_string(),
            content: content.to_string(),
            is_pinned: false,
            is_archived: false,
            is_trashed: false,
            encrypted: true,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::note::NoteService;
    use tempfile::tempdir;

    fn with_setup<F>(f: F)
    where
        F: for<'a> FnOnce(NoteService<'a>, ExportService<'a>),
    {
        let db = Database::open_in_memory().unwrap();
        let note_svc = NoteService::new(&db);
        let export_svc = ExportService::new(&db);
        f(note_svc, export_svc);
    }

    #[test]
    fn exporting_then_importing_preserves_content() {
        with_setup(|note_svc, export_svc| {
            let note = note_svc.create("Export Test", "Secret content").unwrap();
            let dir = tempdir().unwrap();
            let zip_path = dir.path().join("export.zip");

            export_svc
                .export_notes(&[&note.id], &zip_path, "exportpass")
                .unwrap();

            let imported = export_svc.import_notes(&zip_path, "exportpass").unwrap();

            assert_eq!(imported.len(), 1);
            assert_eq!(imported[0].title, "Export Test");
            assert_eq!(imported[0].content, "Secret content");
        });
    }

    #[test]
    fn importing_with_wrong_password_fails() {
        with_setup(|note_svc, export_svc| {
            let note = note_svc.create("Test", "Content").unwrap();
            let dir = tempdir().unwrap();
            let zip_path = dir.path().join("export.zip");

            export_svc
                .export_notes(&[&note.id], &zip_path, "correctpass")
                .unwrap();

            let result = export_svc.import_notes(&zip_path, "wrongpass");
            assert!(result.is_err());
        });
    }

    #[test]
    fn exporting_with_no_notes_still_works() {
        with_setup(|_note_svc, export_svc| {
            let dir = tempdir().unwrap();
            let zip_path = dir.path().join("empty.zip");
            export_svc.export_notes(&[], &zip_path, "pass").unwrap();
            let imported = export_svc.import_notes(&zip_path, "pass").unwrap();
            assert!(imported.is_empty());
        });
    }
}
