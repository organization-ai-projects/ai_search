use crate::shared::experts::expert_id::ExpertId;

#[derive(Clone, Debug)]
pub struct RouterFeedback {
    pub trace_id: String,
    pub task_loss: f32, // qualité finale (retour orch)
    pub load_balance_grad: f32,
    pub entropy_grad: f32,
    pub util_by_expert: Vec<(ExpertId, f32)>, // stats d'usage pour régul.
}
