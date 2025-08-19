// Struct OrchestrationFeedback

pub struct OrchestrationFeedback {
    pub trace_id: String,
    pub primary_router: String,
    pub shadow_better: Option<String>, // id d’un router shadow gagnant
    pub notes: Option<String>,         // libre (erreurs, drift, etc.)
}
