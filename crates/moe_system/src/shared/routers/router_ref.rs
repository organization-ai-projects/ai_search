use std::sync::Arc;

pub struct RouterRef {
    pub id: String,
    pub handle: Arc<dyn std::any::Any + Send + Sync>, // dyn Router n'est pas accessible ici, on met un trait object neutre
}
