use crate::base_models::neural::transformer::Transformer;
use crate::experts::Expert;
use crate::shared::{ExpertAux, ExpertOut};
use crate::shared::{contexts::Context,inputs::InputData, values::Value};

pub struct NlpFrenchTagger {
    pub model: Transformer,
    // autres params spécifiques
}

#[async_trait::async_trait]
impl Expert for NlpFrenchTagger {
    fn name(&self) -> &'static str {
        "nlp_french_tagger"
    }

    fn can_handle(&self, task: &str) -> bool {
        task.contains("fr:tag")
    }

    async fn infer(&self, x: &InputData, _ctx: &Context) -> ExpertOut {
        let now = std::time::Instant::now();
        let prompt = x.as_text().unwrap_or("");
        let out = self.model.forward_text(prompt);

        ExpertOut {
            value: Value::Text {
                schema: 1,
                data: out,
            },
            aux: ExpertAux {
                latency_ms: now.elapsed().as_millis() as u64,
                cost_units: (self.model.layers as f32) * 0.5,
                confidence: 0.6,
                trace_id: None,
            },
        }
    }
}
