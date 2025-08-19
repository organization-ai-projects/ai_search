use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Entrée d'index locale pour orchestrateurs (id, name, tags, path)
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct OrchestratorIndexEntry {
    pub id: Uuid,
    pub name: String,
    pub tags: Vec<String>,
    pub path: String,
}

impl OrchestratorIndexEntry {
    pub fn new(name: String, tags: Vec<String>, path: String) -> Self {
        Self {
            id: Uuid::now_v7(),
            name,
            tags,
            path,
        }
    }
}
