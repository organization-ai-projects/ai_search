use crate::experts::config::Config;
use crate::router::router::Router;

pub struct Orchestrator {
    router: Router,
}

impl Orchestrator {
    pub fn new(config: Option<Config>) -> Self {
        Orchestrator {
            router: Router::new(config),
        }
    }
    pub fn run(&mut self, input: &str) -> Vec<(String, Result<String, String>, u128)> {
        println!("[Orchestrator] Début du traitement pour l'entrée : '{input}'");
        let results = self.router.route(input);
        println!("[Orchestrator] Fin du traitement pour l'entrée : '{input}'");
        results
    }

    /// Synthétise les résultats des experts (vote majoritaire sur les valeurs Ok, sinon premier Ok, sinon premier Err)
    pub fn synthetize(&self, results: &[(String, Result<String, String>, u128)]) -> String {
        use std::collections::HashMap;
        let mut counts = HashMap::new();
        for (_, res, _) in results {
            if let Ok(val) = res {
                *counts.entry(val).or_insert(0) += 1;
            }
        }
        if let Some((val, _)) = counts.into_iter().max_by_key(|(_, c)| *c) {
            val.clone()
        } else {
            // fallback : premier Ok, sinon premier Err
            for (_, res, _) in results {
                if let Ok(val) = res {
                    return val.clone();
                }
            }
            for (_, res, _) in results {
                if let Err(e) = res {
                    return format!("ERREUR: {}", e);
                }
            }
            "Aucun résultat".to_string()
        }
    }
}
