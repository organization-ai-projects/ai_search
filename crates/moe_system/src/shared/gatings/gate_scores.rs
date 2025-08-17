use crate::shared::ExpertId;

#[derive(Clone, Debug)]
pub struct GateScores {
    // scores/softmax par expert_id
    pub logits: Vec<(ExpertId, f32)>,
}
