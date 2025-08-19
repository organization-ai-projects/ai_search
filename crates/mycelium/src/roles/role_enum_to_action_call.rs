use super::{
    quality_judge::{quality_judge_to_action_call, QualityJudge},
    reviewer::Reviewer,
    validator::Validator,
    RoleBehavior, RoleSelector, Roles, Spokesperson, Synthesizer,
};

/// Associe un rôle enum à l'appel de l'action correspondante
pub fn role_enum_to_action_call(role: &Roles, input: &str) -> Box<dyn std::any::Any> {
    match role {
        Roles::Synthesizer => Synthesizer.process(input),
        Roles::QualityJudge(judge) => quality_judge_to_action_call(judge.clone(), input),
        Roles::RoleSelector => RoleSelector.process(input),
        Roles::Spokesperson => Spokesperson.process(input),
        Roles::Reviewer(reviewer) => Box::new(reviewer.review(input)),
        Roles::Validator(validator) => Box::new(validator.validate(input)),
    }
}
