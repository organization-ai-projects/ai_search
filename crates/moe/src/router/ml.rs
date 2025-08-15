use crate::experts::expert_trait::Expert;
use std::collections::HashSet;

/// Prototype ML : routage par similarité k-NN sur l'historique de feedbacks négatifs
pub fn ml_route<'a>(
    input: &str,
    experts: &'a [Box<dyn Expert + Sync>],
) -> Option<Vec<&'a Box<dyn Expert + Sync>>> {
    let feedback_content = std::fs::read_to_string("routing_feedback.json").unwrap_or_default();
    let mut scored: Vec<(usize, Vec<String>)> = Vec::new();
    for line in feedback_content.lines() {
        if let Ok(entry) = serde_json::from_str::<serde_json::Value>(line) {
            if let Some(input_hist) = entry.get("input").and_then(|v| v.as_str()) {
                let score = jaccard_similarity(&input.to_lowercase(), &input_hist.to_lowercase());
                if let Some(experts_names) =
                    entry.get("selected_experts").and_then(|v| v.as_array())
                {
                    let names: Vec<String> = experts_names
                        .iter()
                        .filter_map(|e| e.as_str().map(|s| s.to_string()))
                        .collect();
                    scored.push((score, names));
                }
            }
        }
    }
    scored.sort_by_key(|(score, _)| std::cmp::Reverse(*score));
    if let Some((best_score, best_names)) = scored.into_iter().find(|(s, _)| *s > 0) {
        let selected: Vec<_> = best_names
            .iter()
            .filter_map(|name| experts.iter().find(|e| e.name().eq_ignore_ascii_case(name)))
            .collect();
        if !selected.is_empty() {
            return Some(selected);
        }
    }
    None
}

fn jaccard_similarity(a: &str, b: &str) -> usize {
    let set_a: HashSet<_> = a.split_whitespace().collect();
    let set_b: HashSet<_> = b.split_whitespace().collect();
    set_a.intersection(&set_b).count()
}
