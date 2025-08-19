use super::super::role_behavior_trait::RoleBehavior;
use super::quality_judge_empty_check::QualityJudgeEmptyCheck;
use super::QualityJudge;

/// Exécute le contrôle correspondant à la variante QualityJudge passée.
pub fn quality_judge_to_action_call(judge: QualityJudge, input: &str) -> Box<dyn std::any::Any> {
    match judge {
        QualityJudge::EmptyCheck => QualityJudgeEmptyCheck.process(input),
        // QualityJudge::Autre => AutreJudge.process(input),
    }
}
