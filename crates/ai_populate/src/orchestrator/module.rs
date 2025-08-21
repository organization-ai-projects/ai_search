//! Définition du trait commun à tous les modules de traitement IA
use crate::orchestrator::data_packet::DataPacket;

pub trait Module: Send {
    /// Nom unique du module
    fn name(&self) -> &str;
    /// Modalité prise en charge (ex: "text", "image", ...)
    fn modality(&self) -> &str;
    /// Traitement principal
    fn process(&self, input: &DataPacket) -> DataPacket;
    /// Fallback optionnel en cas d'échec ou d'incompréhension
    fn fallback(&self, input: &DataPacket) -> Option<DataPacket> {
        None
    }
}
