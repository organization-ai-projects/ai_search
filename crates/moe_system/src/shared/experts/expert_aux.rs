#[derive(Clone, Debug)]
pub struct ExpertAux {
    pub latency_ms: u64,
    pub cost_units: f32,
    pub confidence: f32,
    pub trace_id: Option<String>,
}
