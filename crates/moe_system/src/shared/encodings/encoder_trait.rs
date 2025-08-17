use crate::shared::{Encoded, InputData, MoeResult};

pub trait Encoder: Send + Sync {
    fn encode(&self, x: &InputData) -> MoeResult<Encoded>;
}
