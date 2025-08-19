use crate::roles::Roles;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Agent {
    pub id: Uuid,              // UUIDv7 pour identifier l'agent
    pub name: String,          // Nom de l'agent
    pub roles: HashSet<Roles>, // Rôles de l'agent dans le réseau (HashSet)
}

// Pour créer un nouvel agent :
// let mut roles = HashSet::new();
// roles.insert(Roles::Synthesizer);
// roles.insert(Roles::QualityJudge);
// let agent = Agent {
//     id: Uuid::now_v7(),
//     name: "SynthBot".to_string(),
//     roles,
// };
