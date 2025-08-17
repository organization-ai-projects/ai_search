#[derive(Clone, Debug)]
pub struct TransformerConfig {
    pub dim: usize,
    pub layers: usize,
    // Ajoute ici d'autres hyperparamètres (dropout, heads, etc.)
}
