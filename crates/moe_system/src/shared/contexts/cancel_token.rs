/// Token d'annulation thread-safe pour propagation du cancel (coopératif, sans lock)
#[derive(Clone)]
pub struct CancelToken(pub std::sync::Arc<std::sync::atomic::AtomicBool>);
impl CancelToken {
    pub fn cancel(&self) {
        self.0.store(true, std::sync::atomic::Ordering::SeqCst)
    }
    pub fn is_cancelled(&self) -> bool {
        self.0.load(std::sync::atomic::Ordering::SeqCst)
    }
}
