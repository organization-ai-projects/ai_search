//! Abstraction pour le support GPU
use cust::{memory::DeviceBuffer, prelude::*};
use std::collections::HashMap;

pub struct GpuDevice {
    context: Context,
    experts: HashMap<String, Module>,
}

impl GpuDevice {
    pub fn new() -> Self {
        // Initialise CUDA et crée un contexte
        let _cuda = cust::quick_init().expect("CUDA init failed");
        let device = Device::get_device(0).expect("No CUDA device");
        let context =
            Context::create_and_push(ContextFlags::MAP_HOST | ContextFlags::SCHED_AUTO, device)
                .expect("Failed to create CUDA context");
        GpuDevice {
            context,
            experts: HashMap::new(),
        }
    }
    // Ajoutez ici les méthodes spécifiques GPU (allocation, transfert, etc.)
}

use super::device_trait::Device;

impl Device for GpuDevice {
    fn name(&self) -> &'static str {
        "gpu"
    }
    fn load_expert(&mut self, expert_id: &str, weights: &[u8]) -> Result<(), String> {
        // Charge un module CUDA (PTX ou cubin) en mémoire
        let module =
            Module::load_from_bytes(weights).map_err(|e| format!("CUDA load error: {}", e))?;
        self.experts.insert(expert_id.to_string(), module);
        Ok(())
    }
    fn forward_expert(&self, expert_id: &str, input: &[f32]) -> Result<Vec<f32>, String> {
        let module = self.experts.get(expert_id).ok_or("Expert non chargé")?;
        // Allocation du buffer d'entrée sur le device
        let mut d_input = DeviceBuffer::from_slice(input).map_err(|e| e.to_string())?;
        // Prépare un buffer de sortie (1 f32 pour un dot product, à adapter)
        let mut d_output = DeviceBuffer::from_slice(&[0f32]).map_err(|e| e.to_string())?;
        // Récupère la fonction kernel (doit exister dans le module)
        let func = module
            .get_function("forward")
            .map_err(|e| format!("Kernel 'forward' introuvable: {}", e))?;
        // Lance le kernel (exemple : 1 bloc, 1 thread)
        unsafe {
            launch!(func<<<1, 1, 0, Stream::null()>>>(
                d_input.as_device_ptr(),
                d_output.as_device_ptr(),
                input.len() as u32
            ))
            .map_err(|e| format!("Erreur lancement kernel: {}", e))?;
        }
        // Récupère le résultat
        let mut output = [0f32];
        d_output.copy_to(&mut output).map_err(|e| e.to_string())?;
        Ok(output.to_vec())
    }
    fn unload_expert(&mut self, expert_id: &str) -> Result<(), String> {
        self.experts.remove(expert_id);
        Ok(())
    }
}
