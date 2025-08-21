//! Définition du format d'échange générique entre modules

use std::collections::HashMap;

use crate::orchestrator::any_clone::AnyClone;

pub struct DataPacket {
    pub modality: String,              // ex: "text", "image", "audio"
    pub payload: Box<dyn AnyClone>,    // contenu typé dynamiquement et clonable
    pub meta: HashMap<String, String>, // métadonnées (optionnel)
}

impl Clone for DataPacket {
    fn clone(&self) -> Self {
        DataPacket {
            modality: self.modality.clone(),
            payload: self.payload.clone(),
            meta: self.meta.clone(),
        }
    }
}
