use super::super::role_behavior_trait::RoleBehavior;

pub struct QualityJudgeEmptyCheck;

impl RoleBehavior for QualityJudgeEmptyCheck {
    fn name(&self) -> &'static str {
        "QualityJudgeEmptyCheck"
    }
    fn process(&self, input: &str) -> Box<dyn std::any::Any> {
        let status = if input.trim().is_empty() {
            "Vide"
        } else {
            "Non vide"
        };
        Box::new(status.to_string())
    }
}
