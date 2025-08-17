use crate::shared::CancelToken;

/// Contexte partagé pour la gestion du budget, du tracing, etc.
pub struct Context {
    pub budget_ms: u64,
    pub deadline_at: std::time::Instant,
    pub trace_id: String,
    pub cancel: CancelToken, // défini en 2.7
                             // mémoire, kv, etc.
}

pub fn remaining_ms(ctx: &Context) -> Option<u64> {
    ctx.deadline_at
        .checked_duration_since(std::time::Instant::now())
        .map(|d| d.as_millis() as u64)
}
