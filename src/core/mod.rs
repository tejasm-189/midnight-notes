pub mod backlinks;
pub mod export;
pub mod history;
pub mod note;
pub mod plugin;
pub mod search;
pub mod tag;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum NoteServiceError {
    #[error("sqlite error: {0}")]
    Sqlite(#[from] rusqlite::Error),
    #[error("not found")]
    NotFound,
    #[error("invalid input: {0}")]
    InvalidInput(String),
}
