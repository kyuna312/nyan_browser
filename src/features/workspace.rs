use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

pub struct Workspace {
    pub name: String,
    pub projects: Vec<Project>,
    pub bookmarks: Vec<Bookmark>,
    pub notes: Vec<Note>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Bookmark {
    pub title: String,
    pub url: String,
    pub tags: Vec<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Note {
    pub title: String,
    pub content: String,
    pub tags: Vec<String>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Project {
    pub name: String,
    pub urls: Vec<String>,
    pub environment: HashMap<String, String>,
    pub auto_refresh: bool,
    pub watch_paths: Vec<PathBuf>,
}
