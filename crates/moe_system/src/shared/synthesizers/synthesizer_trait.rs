use crate::shared::{GateScores, MoeResult};

pub trait Synthesizer<C, S>: Send + Sync {
    fn id(&self) -> &'static str;
    fn synthesize(&self, calls: &[C], scores: &GateScores) -> MoeResult<S>;
}
