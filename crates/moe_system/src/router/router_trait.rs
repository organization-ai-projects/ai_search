use crate::router::router_feedback::RouterFeedback;
use crate::router::router_input::RouterInput;
use crate::shared::contexts::context::Context;
use crate::shared::errors::moe_result::MoeResult;
use crate::shared::experts::ExpertId;
use crate::shared::experts::ExpertOut;
use crate::shared::experts::ExpertRef;
use crate::shared::gatings::gate_scores::GateScores;

#[async_trait::async_trait]
pub trait Router: Send + Sync {
    fn gate(&self, input: &RouterInput, ctx: &Context) -> MoeResult<GateScores>;
    fn pick_topk(&self, scores: &GateScores, k: usize) -> MoeResult<Vec<ExpertRef>> {
        let n = scores.logits.len();
        let k = k.min(n).max(1);
        if n == 0 {
            return Err(MoeError::NoExpertSelected);
        }
        Err(MoeError::SynthesisFailed(
            "pick_topk: impl manquante".into(),
        ))
    }
    async fn call_experts(
        &self,
        input: &RouterInput,
        picks: &[ExpertRef],
        ctx: &Context,
    ) -> MoeResult<Vec<(ExpertRef, ExpertOut)>>;
    fn train_signal(&mut self, fb: RouterFeedback);
}
