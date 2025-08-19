use moe_system::experts::expert_trait::Expert;
use moe_system::experts::in_memory_registry::InMemoryExpertRegistry;
use moe_system::experts::nlp::NlpFrenchTagger;
use moe_system::experts::planning::RulePlanner;
use moe_system::orchestrator::default_orchestrator::DefaultOrchestrator;
use moe_system::orchestrator::default_synthesizer::DefaultSynthesizer;
use moe_system::router::simple_router::SimpleRouter;
use moe_system::shared::contexts::cancel_token::CancelToken;
use moe_system::shared::contexts::context::Context;
use moe_system::shared::errors::moe_result::MoeResult;
use moe_system::shared::experts::{ExpertId, ExpertRef};
use moe_system::shared::gatings::gate_scores::GateScores;
use moe_system::shared::global::global_index::GlobalIndex;
use moe_system::shared::inputs::input_data::InputData;
use std::collections::HashMap;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

#[tokio::main]
async fn main() {
    // 1. Charger l'index global
    let global_index = GlobalIndex::load_from_file("global_index.json")
        .expect("Impossible de charger l'index global");

    // 2. Trouver le chemin de l'index des orchestrateurs via le global
    let orchestrator_index_path = global_index
        .get_path("orchestrators", "default")
        .expect("Pas de chemin d'index orchestrator trouvé");

    // 3. Charger l'index des orchestrateurs
    let orchestrator_index = GlobalIndex::load_from_file(orchestrator_index_path)
        .expect("Impossible de charger l'index des orchestrateurs");

    // 4. Trouver le chemin de l'orchestrateur à utiliser (ex: tag "default")
    let orchestrator_path = orchestrator_index
        .get_path("orchestrator", "default")
        .expect("Pas de chemin d'orchestrateur trouvé");

    // 5. Charger dynamiquement l'orchestrateur (ici, on simule avec DefaultOrchestrator)
    // TODO: remplacer par un vrai chargement dynamique (plugin, reflection, etc.)
    let orchestrator = DefaultOrchestrator::from_path(orchestrator_path, &global_index);

    // 6. Construire le contexte et l'input
    let input = InputData::Text("entrée factice".to_string());
    let ctx = Context {
        budget_ms: 1000,
        deadline_at: std::time::Instant::now() + std::time::Duration::from_millis(1000),
        trace_id: "trace-1".to_string(),
        cancel: CancelToken(Arc::new(AtomicBool::new(false))),
    };

    // 7. Exécution du pipeline
    match orchestrator.run_pipeline(&input, &ctx) {
        Ok(val) => println!("Sortie MoE : {:?}", val),
        Err(e) => eprintln!("Erreur MoE : {:?}", e),
    }
}
