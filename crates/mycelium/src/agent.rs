use crate::roles::Roles;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Agent {
    pub id: Uuid,          // UUIDv7 pour identifier l'agent
    pub name: String,      // Nom de l'agent
    pub roles: Vec<Roles>, // Rôles de l'agent (ordre déterminé par Roles::exec_order)
}

// Pour créer un nouvel agent :
// let roles = vec![Roles::Synthesizer, Roles::QualityJudge(QualityJudge::EmptyCheck)];
// let agent = Agent {
//     id: Uuid::now_v7(),
//     name: "SynthBot".to_string(),
//     roles,
// };
