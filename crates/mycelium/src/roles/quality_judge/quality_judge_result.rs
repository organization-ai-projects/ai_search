use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum QualityJudgeResult {
    Vide,
    Valide,
    NonValide,
    Inconnu,
}
