// Trait Expert utilisé par tous les experts
use std::error::Error;

pub trait Expert: Sync {
    fn process(&self, input: &str) -> Result<String, Box<dyn Error + Send + Sync>>;
    fn name(&self) -> &'static str;
    /// Dépendances nécessaires pour cet expert (par défaut aucune)
    fn dependencies() -> Vec<&'static str>
    where
        Self: Sized,
    {
        vec![]
    }
}
