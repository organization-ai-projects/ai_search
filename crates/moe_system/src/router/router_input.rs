use crate::shared::encodings::encoded::Encoded;
use crate::shared::inputs::input_data::InputData;

#[derive(Clone, Debug)]
pub enum RouterInput {
    Encoded(Encoded),
    Raw(InputData),
    // Ajoute d'autres variantes si besoin (ex: features, tokens…)
}
