use rayon::prelude::*;
mod agent;
mod context;
mod roles;

use uuid::Uuid;

use crate::roles::QualityJudge;
use crate::roles::{role_enum_to_action_call::role_enum_to_action_call, Roles};

fn main() {
    // Création des agents (exemple, à adapter selon le contexte réel)
    let agents = vec![
        agent::Agent {
            id: Uuid::now_v7(),
            name: "QualityJudgeBot".to_string(),
            roles: vec![Roles::QualityJudge(QualityJudge::EmptyCheck)],
        },
        agent::Agent {
            id: Uuid::now_v7(),
            name: "SynthBot".to_string(),
            roles: vec![Roles::Synthesizer],
        },
        agent::Agent {
            id: Uuid::now_v7(),
            name: "SpokesBot".to_string(),
            roles: vec![Roles::Spokesperson],
        },
    ];

    let input = "Donnée métier à traiter";
    let mut context = input.to_string();

    // On détermine le nombre d'étapes à parcourir (max des indices présents)
    let max_step = agents
        .iter()
        .flat_map(|agent| agent.roles.iter().flat_map(|r| r.exec_orders()))
        .max()
        .unwrap_or(0);

    for step in 0..=max_step {
        // Collecte tous les rôles à exécuter à cette étape
        let roles_to_run: Vec<_> = agents
            .iter()
            .flat_map(|agent| agent.roles.iter())
            .filter(|role| role.exec_orders().contains(&step))
            .collect();

        if roles_to_run.is_empty() {
            continue;
        }

        // Exécution parallèle des rôles de cette étape
        let results: Vec<_> = roles_to_run
            .par_iter()
            .map(|role| (role, role_enum_to_action_call(role, &context)))
            .collect();

        // Affichage (à adapter selon le contexte)
        println!("\n[Étape {step}] Résultats :");
        for (role, s) in &results {
            println!("- {:?} : {}", role, s);
        }

        // Mise à jour du contexte pour l'étape suivante (exemple naïf)
        // Ici, on concatène tous les résultats String pour l'étape suivante
        let next_context = results
            .iter()
            .map(|(_, s)| s)
            .cloned()
            .collect::<Vec<_>>()
            .join(" | ");
        if !next_context.is_empty() {
            context = next_context;
        }
    }
}
