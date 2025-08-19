use super::super::role_behavior_trait::RoleBehavior;
use super::analysis_quality::AnalysisQuality;
use super::analysis_quality_basic::AnalysisQualityBasic;

pub fn analysis_quality_to_action_call(
    role: AnalysisQuality,
    input: &str,
) -> Box<dyn std::any::Any> {
    match role {
        AnalysisQuality::Basic => AnalysisQualityBasic.process(input),
    }
}
