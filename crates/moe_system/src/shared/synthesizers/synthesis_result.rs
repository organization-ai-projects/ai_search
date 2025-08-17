use crate::shared::{SynthesisMetadata, Value};

#[derive(Clone, Debug)]
pub struct SynthesisResult {
    /// Sortie unique après synthèse locale du routeur
    pub value: Value,
    /// Métadonnées de synthèse / télémétrie
    pub synthesis_metadata: SynthesisMetadata,
}
