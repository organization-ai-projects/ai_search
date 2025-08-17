use crate::experts::{expert_id::ExpertId, expert_ref::ExpertRef};

pub trait ExpertRegistry: Send + Sync {
    fn list(&self) -> Vec<(ExpertId, &'static str)>;
    fn get_by_id(&self, id: ExpertId) -> Option<ExpertRef>;
    fn get_by_name(&self, name: &str) -> Option<ExpertRef>;
}
