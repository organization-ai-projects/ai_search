use serde::Deserialize;
use std::collections::HashMap;
use std::fs;

#[derive(Deserialize, Debug)]
struct FeedbackEntry {
    input: String,
    rule: Option<String>,
    selected_experts: Vec<String>,
    results: Vec<String>,
}

pub fn analyze_feedback() {
    let content = fs::read_to_string("routing_feedback.json").unwrap_or_default();
    let mut input_counts: HashMap<String, usize> = HashMap::new();
    let mut rule_counts: HashMap<String, usize> = HashMap::new();
    let mut expert_counts: HashMap<String, usize> = HashMap::new();
    for line in content.lines() {
        if let Ok(entry) = serde_json::from_str::<FeedbackEntry>(line) {
            *input_counts.entry(entry.input.clone()).or_insert(0) += 1;
            if let Some(rule) = entry.rule {
                *rule_counts.entry(rule).or_insert(0) += 1;
            }
            for expert in entry.selected_experts {
                *expert_counts.entry(expert).or_insert(0) += 1;
            }
        }
    }
    println!("\n[Analyse Feedback] Inputs problématiques :");
    for (input, count) in &input_counts {
        if *count > 1 {
            println!("- L'input '{input}' a reçu {count} feedbacks négatifs");
        }
    }
    println!("\n[Analyse Feedback] Règles souvent en échec :");
    for (rule, count) in &rule_counts {
        if *count > 1 {
            println!("- La règle '{rule}' a généré {count} feedbacks négatifs");
        }
    }
    println!("\n[Analyse Feedback] Experts souvent sélectionnés lors d'un échec :");
    for (expert, count) in &expert_counts {
        if *count > 1 {
            println!("- L'expert '{expert}' était sélectionné dans {count} cas négatifs");
        }
    }
    if input_counts.values().any(|&c| c > 1) || rule_counts.values().any(|&c| c > 1) {
        println!("\n[Conseil] Pensez à enrichir router_config.json ou à ajouter de nouveaux experts pour mieux couvrir ces cas.");
    }
}
