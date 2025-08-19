mod agent;
mod roles;

use std::collections::HashSet;
use uuid::Uuid;

use crate::roles::QualityJudge;
use crate::roles::{role_enum_to_action_call::role_enum_to_action_call, Roles};
use crate::roles::{Reviewer, Validator};

fn main() {
    let mut roles_set = HashSet::new();
    roles_set.insert(Roles::Synthesizer);
    roles_set.insert(Roles::QualityJudge(QualityJudge::EmptyCheck));
    roles_set.insert(Roles::Reviewer(Reviewer::SimpleReview));
    roles_set.insert(Roles::Validator(Validator::BasicValidation));
    let agent = agent::Agent {
        id: Uuid::now_v7(),
        name: "SynthBot".to_string(),
        roles: roles_set,
    };
    println!("Agent created: {:?}", agent);

    let input = "Donnée métier à traiter";
    let mut results = Vec::new();
    for role in &agent.roles {
        let res = role_enum_to_action_call(role, input);
        results.push((role, res));
    }
    for (role, res) in results {
        if let Some(s) = res.downcast_ref::<String>() {
            println!("Résultat pour {:?} : {}", role, s);
        } else {
            println!("Résultat pour {:?} : [Type inconnu]", role);
        }
    }
}
