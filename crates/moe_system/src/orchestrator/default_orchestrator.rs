// Implémentation concrète d'un orchestrateur MoE
use crate::orchestrator::orchestration_feedback::OrchestrationFeedback;
use crate::orchestrator::orchestrator_trait::Orchestrator;
use crate::shared::contexts::context::Context;
use crate::shared::errors::moe_result::MoeResult;
use crate::shared::inputs::input_data::InputData;
use crate::shared::routers::{RoutedOutput, RouterRef};
use crate::shared::synthesizers::{SynthesisResult, Synthesizer};
use crate::shared::values::value::Value;
use crate::shared::GateScores;

/// Orchestrateur concret générique pour MoE
/// R = closure ou objet qui encapsule la logique de routing (input, context) -> (RouterRef, Vec<RoutedOutput>)
/// S = synthétiseur (implémente Synthesizer)
pub struct DefaultOrchestrator<R, S> {
    pub router: R,
    pub synthesizer: S,
}

impl<R, S> Orchestrator for DefaultOrchestrator<R, S>
where
    R: Fn(&InputData, &Context) -> MoeResult<(RouterRef, Vec<RoutedOutput>, GateScores)>
        + Send
        + Sync,
    S: Synthesizer<RoutedOutput, SynthesisResult>,
{
    fn choose_router(&self, x: &InputData, ctx: &Context) -> MoeResult<RouterRef> {
        let (router_ref, _, _) = (self.router)(x, ctx)?;
        Ok(router_ref)
    }

    fn synthesize(
        &self,
        primary: (&RouterRef, SynthesisResult),
        _shadow: Option<Vec<(&RouterRef, SynthesisResult)>>,
    ) -> MoeResult<Value> {
        // Par défaut, retourne la valeur principale (shadow routing possible)
        Ok(primary.1.value.clone())
    }

    fn feedback(&mut self, _fb: OrchestrationFeedback) {
        // À compléter selon la logique de feedback transverse
    }
}

impl<R, S> DefaultOrchestrator<R, S>
where
    R: Fn(&InputData, &Context) -> MoeResult<(RouterRef, Vec<RoutedOutput>, GateScores)>
        + Send
        + Sync,
    S: Synthesizer<RoutedOutput, SynthesisResult>,
{
    /// Pipeline complet : input -> router -> synthétiseur -> output
    pub fn run_pipeline(&self, input: &InputData, ctx: &Context) -> MoeResult<Value> {
        let (router_ref, routed_outputs, gate_scores) = (self.router)(input, ctx)?;
        let synth_out = self.synthesizer.synthesize(&routed_outputs, &gate_scores)?;
        Ok(synth_out.value)
    }
}
