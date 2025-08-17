use crate::shared::Value;

/// Trait d’adaptation vers Value
pub trait ToValue {
    fn to_value(self) -> Value;
}
