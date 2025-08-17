use crate::experts::expert_id::ExpertId;

#[derive(Clone)]
pub struct ExpertRef {
    pub id: ExpertId,
    pub handle: std::sync::Arc<dyn Expert>,
}
