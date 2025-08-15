use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::io::{self, Write};

#[derive(Deserialize, Debug)]
struct FeedbackEntry {
    input: String,
    rule: Option<String>,
    selected_experts: Vec<String>,
    results: Vec<String>,
}

/// Génère une suggestion de patch JSON pour router_config.json à partir des feedbacks négatifs
pub fn suggest_routing_patch() {
    let content = fs::read_to_string("routing_feedback.json").unwrap_or_default();
    let mut input_counts: HashMap<String, usize> = HashMap::new();
    let mut input_examples: HashMap<String, FeedbackEntry> = HashMap::new();
    for line in content.lines() {
        if let Ok(entry) = serde_json::from_str::<FeedbackEntry>(line) {
            *input_counts.entry(entry.input.clone()).or_insert(0) += 1;
            input_examples.entry(entry.input.clone()).or_insert(entry);
        }
    }
    // Trouver l'input le plus problématique
    if let Some((worst_input, count)) = input_counts.iter().max_by_key(|(_, c)| *c) {
        if *count > 1 {
            let entry = &input_examples[worst_input];
            // Extraire un mot-clé simple (le mot le plus fréquent de l'input)
            let mut word_freq = HashMap::new();
            for word in worst_input.split_whitespace() {
                *word_freq.entry(word.to_lowercase()).or_insert(0) += 1;
            }
            if let Some((keyword, _)) = word_freq.iter().max_by_key(|(_, c)| *c) {
                // Suggérer d'associer ce mot-clé aux experts sélectionnés
                let patch = serde_json::json!({
                    "add_rule": {
                        "keyword": keyword,
                        "experts": entry.selected_experts
                    }
                });
                println!(
                    "\n[Suggestion] Patch JSON à ajouter à router_config.json :\n{}",
                    serde_json::to_string_pretty(&patch).unwrap()
                );
                println!("[Conseil] Ajoutez cette règle pour mieux router les inputs similaires à : '{}'.", worst_input);

                // Auto-ajustement sans confirmation : appliquer la règle directement
                let mut router_config: serde_json::Value = serde_json::from_str(
                    &fs::read_to_string("router_config.json").unwrap_or("{}".to_string()),
                )
                .unwrap_or(serde_json::json!({"rules":[],"fallback":[]}));
                if let Some(rules) = router_config
                    .get_mut("rules")
                    .and_then(|r| r.as_array_mut())
                {
                    rules.push(serde_json::json!({
                        "keyword": keyword,
                        "experts": entry.selected_experts
                    }));
                    // Sauvegarder
                    if let Ok(mut file) = fs::OpenOptions::new()
                        .write(true)
                        .truncate(true)
                        .open("router_config.json")
                    {
                        let _ = write!(
                            file,
                            "{}",
                            serde_json::to_string_pretty(&router_config).unwrap()
                        );
                        println!("[Auto-ajustement] Règle ajoutée automatiquement à router_config.json !");
                    }
                }
            }
        }
    }
}
