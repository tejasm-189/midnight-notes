use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// A note entity.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Note {
    pub id: String,
    pub title: String,
    pub content: String,
    pub is_pinned: bool,
    pub is_archived: bool,
    pub is_trashed: bool,
    pub encrypted: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// A tag entity with support for nested hierarchies.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tag {
    pub id: String,
    pub name: String,
    pub parent_id: Option<String>,
    pub color: Option<String>,
    pub created_at: DateTime<Utc>,
}

/// A backlink representing a [[wiki-link]] from one note to another.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Backlink {
    pub source_note_id: String,
    pub target_note_id: String,
    pub created_at: DateTime<Utc>,
}

/// A snapshot of a note at a point in time (for version history).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NoteHistory {
    pub id: String,
    pub note_id: String,
    pub content_snapshot: String,
    pub title_snapshot: String,
    pub created_at: DateTime<Utc>,
}

/// A search result from FTS5.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    pub note: Note,
    pub rank: f64,
    pub snippet: Option<String>,
}

/// Application metadata key-value pair.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Meta {
    pub key: String,
    pub value: String,
}
