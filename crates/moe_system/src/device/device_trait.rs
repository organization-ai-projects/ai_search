/// Trait commun pour tous les devices (CPU, GPU, etc.)
pub trait Device {
    /// Nom du device (ex: "cpu", "gpu")
    fn name(&self) -> &'static str;

    /// Charge un expert (modèle) sur le device
    fn load_expert(&mut self, expert_id: &str, weights: &[u8]) -> Result<(), String>;

    /// Exécute un forward sur un batch pour un expert donné
    fn forward_expert(&self, expert_id: &str, input: &[f32]) -> Result<Vec<f32>, String>;

    /// Libère la mémoire du modèle
    fn unload_expert(&mut self, expert_id: &str) -> Result<(), String>;
}
