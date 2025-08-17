use super::{block::TransformerBlock, config::TransformerConfig};
use crate::device::{best_device, Device};

pub struct Transformer {
    pub config: TransformerConfig,
    pub blocks: Vec<TransformerBlock>,
    pub device: Box<dyn Device>,
    // Ajoute ici les poids globaux, embeddings, etc.
}

impl Transformer {
    pub fn new(config: TransformerConfig, device: Option<Box<dyn Device>>) -> Self {
        let blocks = (0..config.layers)
            .map(|i| TransformerBlock {
                dim: config.dim,
                layer_idx: i,
            })
            .collect();
        let device = device.unwrap_or_else(|| best_device());
        Self {
            config,
            blocks,
            device,
        }
    }

    pub fn load_weights(&mut self, expert_id: &str, weights: &[u8]) -> Result<(), String> {
        self.device.load_expert(expert_id, weights)
    }

    pub fn forward(&self, expert_id: &str, input: &[f32]) -> Result<Vec<f32>, String> {
        self.device.forward_expert(expert_id, input)
    }
}
