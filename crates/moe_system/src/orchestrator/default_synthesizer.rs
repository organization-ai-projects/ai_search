// Implémentation DefaultSynthesizer
use crate::shared::errors::moe_error::MoeError;
use crate::shared::errors::moe_result::MoeResult;
use crate::shared::gatings::gate_scores::GateScores;
use crate::shared::routers::RoutedOutput;
use crate::shared::synthesizers::synthesizer_trait::Synthesizer;
use crate::shared::synthesizers::synthesizer_trait::{SynthesisMetadata, SynthesizedOut};

pub struct DefaultSynthesizer;

impl Synthesizer for DefaultSynthesizer {
    fn id(&self) -> &'static str {
        "default"
    }
    fn synthesize(&self, calls: &[RoutedOutput], scores: &GateScores) -> MoeResult<SynthesizedOut> {
        if calls.is_empty() {
            return Err(MoeError::NoExpertSelected);
        }
        // Sélectionne la sortie avec le meilleur score
        let (best_id, _) = scores
            .logits
            .iter()
            .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal))
            .ok_or(MoeError::SynthesisFailed("no scores".into()))?;
        let out = calls
            .iter()
            .find(|o| o.expert_id == *best_id)
            .ok_or(MoeError::SynthesisFailed("no matching output".into()))?;
        Ok(SynthesizedOut {
            value: out.value.clone(),
            synthesis_metadata: SynthesisMetadata {
                entropy: 0.0, // à calculer
                topk: calls.len(),
                lat_total_ms: calls.iter().map(|o| o.aux.latency_ms).sum(),
                drop_count: 0,
                util_by_expert: scores.logits.clone(),
            },
        })
    }
}
