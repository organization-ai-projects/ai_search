use crate::shared::experts::expert_aux::ExpertAux;
use crate::shared::experts::expert_id::ExpertId;
use crate::shared::values::Value;

#[derive(Clone, Debug)]
pub struct RoutedOutput {
    pub expert_id: ExpertId,
    pub value: Value,
    pub aux: ExpertAux,
}
