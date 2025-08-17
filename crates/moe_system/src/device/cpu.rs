//! Abstraction pour le support CPU

use std::collections::HashMap;

/// Modèle simple : un vecteur de poids (exemple)
struct Model {
    weights: Vec<f32>,
}

pub struct CpuDevice {
    experts: HashMap<String, Model>,
}

impl CpuDevice {
    pub fn new() -> Self {
        CpuDevice {
            experts: HashMap::new(),
        }
    }
    // Ajoutez ici les méthodes spécifiques CPU
}

use super::device_trait::Device;

impl Device for CpuDevice {
    fn name(&self) -> &'static str {
        "cpu"
    }
    fn load_expert(&mut self, expert_id: &str, weights: &[u8]) -> Result<(), String> {
        // On suppose que les poids sont sérialisés en f32 little endian
        if weights.len() % 4 != 0 {
            return Err("Poids mal alignés (doivent être des f32)".to_string());
        }
        let mut w = Vec::with_capacity(weights.len() / 4);
        for chunk in weights.chunks_exact(4) {
            w.push(f32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]));
        }
        self.experts
            .insert(expert_id.to_string(), Model { weights: w });
        Ok(())
    }
    fn forward_expert(&self, expert_id: &str, input: &[f32]) -> Result<Vec<f32>, String> {
        let model = self.experts.get(expert_id).ok_or("Expert non chargé")?;
        if input.len() != model.weights.len() {
            return Err("Taille d'entrée incompatible avec le modèle".to_string());
        }
        // Produit scalaire simple (exemple)
        let dot = input.iter().zip(&model.weights).map(|(a, b)| a * b).sum();
        Ok(vec![dot])
    }
    fn unload_expert(&mut self, expert_id: &str) -> Result<(), String> {
        self.experts.remove(expert_id);
        Ok(())
    }
}
