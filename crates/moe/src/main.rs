mod analyze_feedback;
mod analyze_routing;
// mod config; // déplacé dans experts
// mod expert_trait; // déplacé dans experts
mod experts;
mod feedback;
mod orchestrator;
mod router;
mod suggest_routing_patch;

// plus d'import d'experts ici
use orchestrator::orchestrator::Orchestrator;
// use router::router::Router; // Le routeur ne doit jamais être utilisé directement

// Exemple d'expert concret

// use config::Config; // déplacé dans experts
// plus d'import std::fs ici

fn main() {
    let config: Option<experts::config::Config> = std::fs::read_to_string("experts.json")
        .ok()
        .and_then(|txt| serde_json::from_str(&txt).ok());
    let mut orchestrator = Orchestrator::new(config);
    let inputs = ["Hello, MoE!", "reverse this!"];
    for input in &inputs {
        let results = orchestrator.run(input);
        for (name, res, duration) in &results {
            match res {
                Ok(val) => println!("[Expert:{}] Résultat : {} ({} ms)", name, val, duration),
                Err(e) => println!("[Expert:{}] ERREUR : {} ({} ms)", name, e, duration),
            }
        }
        let synth = orchestrator.synthetize(&results);
        println!("[Orchestrator] Synthèse : {}", synth);
    }

    // Analyse automatique de l'historique de routage
    analyze_routing::analyze_routing_history();
    // Analyse automatique des feedbacks utilisateurs
    analyze_feedback::analyze_feedback();
    // Suggestion automatique de patch de routage
    suggest_routing_patch::suggest_routing_patch();
}
