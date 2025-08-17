// Trait Orchestrator
use crate::shared::contexts::context::Context;
use crate::shared::errors::moe_result::MoeResult;
use crate::shared::inputs::input_data::InputData;
use crate::shared::routers::RouterRef;
use crate::shared::synthesizers::synthesizer_trait::SynthesizedOut;
use crate::shared::values::value::Value;

pub trait Orchestrator: Send + Sync {
    fn choose_router(&self, x: &InputData, ctx: &Context) -> MoeResult<RouterRef>;

    fn synthesize(
        &self,
        primary: (&RouterRef, SynthesizedOut),
        shadow: Option<Vec<(&RouterRef, SynthesizedOut)>>,
    ) -> MoeResult<Value>;

    fn feedback(&mut self, fb: OrchestrationFeedback);
}
