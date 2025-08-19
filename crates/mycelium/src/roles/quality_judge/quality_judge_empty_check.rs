use super::super::role_behavior_trait::RoleBehavior;
use super::quality_judge_result::QualityJudgeResult;
use serde_json;

pub struct QualityJudgeEmptyCheck;

impl RoleBehavior for QualityJudgeEmptyCheck {
    fn name(&self) -> &'static str {
        "QualityJudgeEmptyCheck"
    }
    fn process(&self, input: &str) -> Box<dyn std::any::Any> {
        let result = if input.trim().is_empty() {
            QualityJudgeResult::Vide
        } else {
            QualityJudgeResult::Valide
        };
        Box::new(serde_json::to_string(&result).unwrap())
    }
}
