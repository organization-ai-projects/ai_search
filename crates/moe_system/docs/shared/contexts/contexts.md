### 2.4 Contexte partagé (budget, trace, etc.)

```rust
//src/shared/contexts/context.rs
/// Contexte partagé pour la gestion du budget, du tracing, etc.
pub struct Context {
    pub budget_ms: u64,
    pub deadline_at: std::time::Instant,
    pub trace_id: String,
    pub cancel: CancelToken, // défini en 2.7
    // mémoire, kv, etc.
}

pub fn remaining_ms(ctx: &Context) -> Option<u64> {
    ctx.deadline_at.checked_duration_since(std::time::Instant::now())
        .map(|d| d.as_millis() as u64)
}
```

### 2.7 Token d'annulation thread-safe pour propagation du cancel (coopératif, sans lock)
```rust
//src/shared/contexts/cancel_token.rs
/// Token d'annulation thread-safe pour propagation du cancel (coopératif, sans lock)
#[derive(Clone)]
pub struct CancelToken(pub std::sync::Arc<std::sync::atomic::AtomicBool>);
impl CancelToken {
    pub fn cancel(&self) { self.0.store(true, std::sync::atomic::Ordering::SeqCst) }
    pub fn is_cancelled(&self) -> bool { self.0.load(std::sync::atomic::Ordering::SeqCst) }
}
```