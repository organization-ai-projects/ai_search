//! Orchestrateur principal : gestion du pipeline de modules IA
use crate::orchestrator::{data_packet::DataPacket, module::Module};

pub struct Orchestrator {
    modules: Vec<Box<dyn Module>>,
}

impl Orchestrator {
    pub fn new() -> Self {
        Self {
            modules: Vec::new(),
        }
    }

    /// Ajoute un module au pipeline
    pub fn add_module(&mut self, module: Box<dyn Module>) {
        self.modules.push(module);
    }

    /// Exécute le pipeline sur un DataPacket
    pub fn run(&self, mut packet: DataPacket) -> DataPacket {
        for module in &self.modules {
            if module.modality() == packet.modality {
                let result = module.process(&packet);
                packet = result;
            }
        }
        packet
    }

    /// Exécute le pipeline avec gestion de fallback
    pub fn run_with_fallback(&self, mut packet: DataPacket) -> DataPacket {
        for module in &self.modules {
            if module.modality() == packet.modality {
                let result = module.process(&packet);
                // Si le module ne modifie pas le packet, tente fallback
                if std::ptr::eq(&result, &packet) {
                    if let Some(fb) = module.fallback(&packet) {
                        packet = fb;
                        continue;
                    }
                }
                packet = result;
            }
        }
        packet
    }
}
