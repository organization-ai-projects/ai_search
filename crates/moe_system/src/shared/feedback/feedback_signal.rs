use crate::shared::contexts::context::Context;
use crate::shared::gatings::gate_scores::GateScores;
use crate::shared::inputs::input_data::InputData;
use crate::shared::outputs::output_data::OutputData;
use crate::shared::routers::RoutedOutput;

#[derive(Clone, Debug)]
pub struct FeedbackSignal {
    pub input: InputData,
    pub output: OutputData,
    pub routed: Vec<RoutedOutput>,
    pub gate_scores: GateScores,
    pub ctx: Context,
}
