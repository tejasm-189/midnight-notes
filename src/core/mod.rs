pub mod backlinks;
pub mod export;
pub mod history;
pub mod markdown;
pub mod note;
pub mod plugin;
pub mod search;
pub mod tag;
pub mod watcher;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum NoteServiceError {
    #[error("sqlite error: {0}")]
    Sqlite(#[from] rusqlite::Error),
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
    #[error("zip error: {0}")]
    Zip(#[from] zip::result::ZipError),
    #[error("json error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("not found")]
    NotFound,
    #[error("invalid input: {0}")]
    InvalidInput(String),
}
