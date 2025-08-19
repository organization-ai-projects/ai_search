use super::analysis_quality::{analysis_quality_to_action_call, AnalysisQuality};
use super::quality_judge::quality_judge_to_action_call;
use super::Roles;

/// Dispatch unique, typé, compatible rayon : retourne toujours un String
pub fn role_enum_to_action_call(role: &Roles, input: &str) -> String {
    match role {
        Roles::Synthétizer => format!("Synthétisé: {}", input),
        Roles::QualityJudge(judge) => quality_judge_to_action_call(judge.clone(), input)
            .downcast::<String>()
            .ok()
            .map(|s| *s)
            .unwrap_or_default(),
        Roles::AnalysisQuality(aq) => analysis_quality_to_action_call(aq.clone(), input)
            .downcast::<String>()
            .ok()
            .map(|s| *s)
            .unwrap_or_default(),
        Roles::RoleSelector => "[RoleSelector: pas d'action]".to_string(),
        Roles::Spokesperson => format!("Réponse portée: {}", input),
    }
}
