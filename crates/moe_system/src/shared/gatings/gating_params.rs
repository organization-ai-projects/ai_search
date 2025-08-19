/// Paramètres de gating pour le router avancé
#[derive(Clone, Debug)]
pub struct GatingParams {
    pub top_k: usize,
    pub temperature: f32,
}
