//! Sélection automatique du meilleur device disponible (GPU si possible, sinon CPU)

use super::{CpuDevice, Device, GpuDevice};

/// Essaie d'instancier un GPU, fallback CPU si erreur
pub fn best_device() -> Box<dyn Device> {
    match GpuDevice::new() {
        Ok(gpu) => Box::new(gpu),
        Err(_) => Box::new(CpuDevice::new()),
    }
}
