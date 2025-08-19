use crate::shared::ExpertId;

#[derive(Clone, Debug)]
pub struct SynthesisMetadata {
    pub entropy: f32,
    pub topk: usize,
    pub lat_total_ms: u64,
    pub drop_count: usize,
    pub util_by_expert: Vec<(ExpertId, f32)>, // pour load-balance
}
