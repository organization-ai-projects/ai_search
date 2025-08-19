use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Validator {
    BasicValidation,
    // Ajoute d'autres variantes ici
}

impl Validator {
    pub fn validate(&self, input: &str) -> String {
        match self {
            Validator::BasicValidation => format!("Validation basique: {}", input),
        }
    }
}
