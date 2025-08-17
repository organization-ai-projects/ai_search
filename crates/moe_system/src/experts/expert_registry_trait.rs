use crate::{experts::ExpertRef, shared::ExpertId};

pub trait ExpertRegistry: Send + Sync {
    fn list(&self) -> Vec<(ExpertId, &'static str)>;
    fn get_by_id(&self, id: ExpertId) -> Option<ExpertRef>;
    fn get_by_name(&self, name: &str) -> Option<ExpertRef>;

    /// Retourne tous les experts associés à un tag donné (catégorie, fonctionnalité, etc.)
    fn get_by_tag(&self, tag: &str) -> Vec<ExpertRef>;

    /// Filtrage générique sur les experts (future-proof)
    fn filter_by<F>(&self, predicate: F) -> Vec<ExpertRef>
    where
        F: Fn(&ExpertRef) -> bool;
}
