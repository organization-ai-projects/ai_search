use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Entrée d'index locale pour experts (id, name, tags, path)
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct ExpertIndexEntry {
    pub id: Uuid,
    pub name: String,
    pub tags: Vec<String>,
    pub path: String,
}

impl ExpertIndexEntry {
    pub fn new(name: String, tags: Vec<String>, path: String) -> Self {
        Self {
            id: Uuid::now_v7(),
            name,
            tags,
            path,
        }
    }
}
