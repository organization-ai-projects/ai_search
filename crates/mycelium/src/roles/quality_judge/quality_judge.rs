use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum QualityJudge {
    EmptyCheck,
    // Ajouter d'autres variantes métier ici
}
