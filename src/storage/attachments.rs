use std::path::{Path, PathBuf};
use thiserror::Error;
use uuid::Uuid;

#[derive(Error, Debug)]
pub enum AttachmentError {
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
    #[error("attachment not found: {0}")]
    NotFound(String),
    #[error("unsupported content type: {0}")]
    UnsupportedType(String),
}

/// Manages file attachments (images, PDFs, etc.) stored in a hidden directory.
pub struct AttachmentManager {
    base_path: PathBuf,
}

impl AttachmentManager {
    /// Create a new attachment manager rooted at the given directory.
    pub fn new(base_path: impl Into<PathBuf>) -> Self {
        Self {
            base_path: base_path.into(),
        }
    }

    /// Store an attachment from raw bytes. Returns the relative path.
    pub fn store(&self, filename: &str, data: &[u8]) -> Result<String, AttachmentError> {
        let ext = Path::new(filename)
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("bin");
        let id = Uuid::new_v4().to_string();
        let relative = format!("{}.{}", id, ext);
        let full_path = self.base_path.join(&relative);

        std::fs::create_dir_all(&self.base_path)?;
        std::fs::write(&full_path, data)?;

        Ok(relative)
    }

    /// Retrieve an attachment's bytes by its relative path.
    pub fn get(&self, relative_path: &str) -> Result<Vec<u8>, AttachmentError> {
        let full_path = self.base_path.join(relative_path);
        if !full_path.exists() {
            return Err(AttachmentError::NotFound(relative_path.to_string()));
        }
        Ok(std::fs::read(&full_path)?)
    }

    /// Delete an attachment.
    pub fn delete(&self, relative_path: &str) -> Result<(), AttachmentError> {
        let full_path = self.base_path.join(relative_path);
        if full_path.exists() {
            std::fs::remove_file(&full_path)?;
        }
        Ok(())
    }

    /// Get the full filesystem path for an attachment.
    pub fn path(&self, relative_path: &str) -> PathBuf {
        self.base_path.join(relative_path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn storing_and_retrieving_attachment_bytes_works() {
        let dir = tempdir().unwrap();
        let mgr = AttachmentManager::new(dir.path().join(".attachments"));
        let path = mgr.store("test.txt", b"hello world").unwrap();
        assert!(path.ends_with(".txt"));
        let data = mgr.get(&path).unwrap();
        assert_eq!(data, b"hello world");
    }

    #[test]
    fn deleting_a_tag_removes_it_and_note_tags() {
        let dir = tempdir().unwrap();
        let mgr = AttachmentManager::new(dir.path().join(".attachments"));
        let path = mgr.store("test.txt", b"data").unwrap();
        mgr.delete(&path).unwrap();
        assert!(mgr.get(&path).is_err());
    }

    #[test]
    fn getting_nonexistent_attachment_returns_error() {
        let dir = tempdir().unwrap();
        let mgr = AttachmentManager::new(dir.path().join(".attachments"));
        let result = mgr.get("nonexistent.txt");
        assert!(result.is_err());
    }
}
