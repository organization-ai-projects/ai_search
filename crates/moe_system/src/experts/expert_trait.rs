use crate::experts::expert_out::ExpertOut;
use crate::shared::{contexts::Context, errors::MoeResult, inputs::InputData};

#[async_trait::async_trait]
pub trait Expert: Send + Sync {
    fn name(&self) -> &'static str;
    fn can_handle(&self, task: &str) -> bool;
    async fn infer(&self, x: &InputData, ctx: &Context) -> MoeResult<ExpertOut>;
}
