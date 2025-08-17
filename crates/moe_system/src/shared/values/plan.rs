use crate::shared::{PlanStep, ToValue, Value};

/// Plan structuré pour la sortie symbolique
#[derive(Clone, Debug)]
pub struct Plan {
    pub goal: String,
    pub steps: Vec<PlanStep>,
}
impl ToValue for Plan {
    fn to_value(self) -> Value {
        Value::Plan {
            schema: 1,
            data: self,
        }
    }
}
