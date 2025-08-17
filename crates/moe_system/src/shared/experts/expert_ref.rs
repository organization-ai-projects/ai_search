use crate::shared::experts::expert_id::ExpertId;
use crate::shared::experts::expert_trait::Expert;
use std::sync::Arc;

#[derive(Clone)]
pub struct ExpertRef {
    pub id: ExpertId,
    pub handle: Arc<dyn Expert>,
}
