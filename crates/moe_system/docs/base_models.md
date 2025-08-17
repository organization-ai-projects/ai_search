## Modèle interne (neuronal, etc.) et expert métier exposé

```rust
// base_models/neural/transformer.rs (modèle générique, non exposé au routeur)
pub struct Transformer {
    pub dim: usize,
    pub layers: usize,
    // poids/params réels dans ton implémentation
}

impl Transformer {
    pub fn forward_text(&self, prompt: &str) -> String {
        // Stub: dans le vrai code, passe par ton backend (CPU/GPU)
        format!("[gen:{}layers:{}] {}", self.dim, self.layers, prompt)
    }
}
```