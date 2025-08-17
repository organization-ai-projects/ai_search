use crate::{experts::Expert, shared::ExpertId};

#[derive(Clone)]
pub struct ExpertRef {
    pub id: ExpertId,
    pub handle: std::sync::Arc<dyn Expert>,
}
