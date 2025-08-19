use crate::shared::experts::expert_id::ExpertId;
use std::any::Any;
use std::sync::Arc;

#[derive(Clone)]
pub struct ExpertRef {
    pub id: ExpertId,
    pub handle: Arc<dyn Any + Send + Sync>,
}
