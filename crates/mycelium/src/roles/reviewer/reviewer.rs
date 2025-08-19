use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Reviewer {
    SimpleReview,
    // Ajoute d'autres variantes ici
}

impl Reviewer {
    pub fn review(&self, input: &str) -> String {
        match self {
            Reviewer::SimpleReview => format!("Revue simple: {}", input),
        }
    }
}
