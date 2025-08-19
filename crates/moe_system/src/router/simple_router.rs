use crate::router::router_feedback::RouterFeedback;
use crate::router::router_input::RouterInput;
use crate::router::router_trait::Router;
use crate::shared::contexts::context::Context;
use crate::shared::errors::moe_result::MoeResult;
use crate::shared::experts::{ExpertId, ExpertOut, ExpertRef};
use crate::shared::gatings::gate_scores::GateScores;
use crate::shared::{ExpertAux, Value};
use async_trait::async_trait;

pub struct SimpleRouter;

#[async_trait]
impl Router for SimpleRouter {
    fn gate(&self, _input: &RouterInput, _ctx: &Context) -> MoeResult<GateScores> {
        // Retourne un score fixe pour un expert fictif
        let mut logits = std::collections::HashMap::new();
        logits.insert(ExpertId(1), 1.0);
        Ok(GateScores { logits })
    }

    fn pick_topk(&self, scores: &GateScores, k: usize) -> MoeResult<Vec<ExpertRef>> {
        // Retourne des ExpertRef neutres (handle = Arc<()>), la résolution réelle se fait ailleurs
        let refs = scores
            .logits
            .keys()
            .take(k)
            .map(|id| ExpertRef {
                id: *id,
                handle: std::sync::Arc::new(()),
            })
            .collect();
        Ok(refs)
    }

    async fn call_experts(
        &self,
        _input: &RouterInput,
        picks: &[ExpertRef],
        _ctx: &Context,
    ) -> MoeResult<Vec<(ExpertRef, ExpertOut)>> {
        // Retourne une sortie factice pour chaque expert
        let outs = picks
            .iter()
            .map(|r| {
                (
                    r.clone(),
                    ExpertOut {
                        value: Value::Text {
                            schema: 1,
                            data: "réponse factice".to_string(),
                        },
                        aux: ExpertAux {
                            latency_ms: 1,
                            cost_units: 0.1,
                            confidence: 1.0,
                            trace_id: None,
                        },
                    },
                )
            })
            .collect();
        Ok(outs)
    }

    fn train_signal(&mut self, _fb: RouterFeedback) {
        // Pas de logique d'apprentissage ici
    }
}
