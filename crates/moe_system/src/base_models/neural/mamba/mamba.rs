use super::{block::MambaBlock, config::MambaConfig};
use crate::device::{best_device, Device};

pub struct Mamba {
    pub config: MambaConfig,
    pub blocks: Vec<MambaBlock>,
    pub device: Box<dyn Device>,
    // Ajoute ici les poids globaux, embeddings, etc.
}

impl Mamba {
    pub fn new(config: MambaConfig, device: Option<Box<dyn Device>>) -> Self {
        let blocks = (0..config.layers)
            .map(|i| MambaBlock {
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
