use serde::Deserialize;
use std::fs;

#[derive(Deserialize, Debug)]
struct RoutingLogEntry {
    input: String,
    rule: Option<String>,
    selected_experts: Vec<String>,
}

pub fn analyze_routing_history() {
    let content = fs::read_to_string("routing_history.json").unwrap_or_default();
    let mut rule_counts = std::collections::HashMap::new();
    let mut fallback_count = 0;
    for line in content.lines() {
        if let Ok(entry) = serde_json::from_str::<RoutingLogEntry>(line) {
            let rule = entry.rule.unwrap_or_else(|| "unknown".to_string());
            *rule_counts.entry(rule.clone()).or_insert(0) += 1;
            if rule == "fallback" || rule == "naive_fallback" {
                fallback_count += 1;
            }
        }
    }
    println!("\n[Analyse Routage] Statistiques sur l'historique :");
    for (rule, count) in &rule_counts {
        println!("- Règle '{rule}' utilisée {count} fois");
    }
    println!("- Fallback utilisé {fallback_count} fois");
    if fallback_count > 5 {
        println!("\n[Conseil] Le fallback est souvent utilisé : pensez à enrichir router_config.json pour mieux couvrir les cas d'usage.");
    }
}
