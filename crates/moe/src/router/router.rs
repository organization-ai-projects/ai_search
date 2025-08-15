// modules vectorizer et scoring sont déjà des fichiers siblings, pas besoin de les redéclarer ici

use crate::router::scoring::Score;
use crate::router::vectorizer::{TfVectorizer, Vectorizer};
use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::Write;

#[derive(Clone)]
struct Cluster {
    centroid: Vec<f32>,
    experts: HashMap<String, Score>,
}

use crate::experts::config::Config;
use crate::experts::registry_experts::get_dependencies;
use crate::experts::registry_experts::get_factories;

pub struct Router {
    vectorizer: TfVectorizer,
    clusters: Vec<Cluster>,
    weights: HashMap<&'static str, f32>,
    // plus de factory globale, tout est paresseux
}

impl Router {
    pub fn new(_config: Option<Config>) -> Self {
        // Poids par défaut pour le scoring multi-critères
        let mut weights = HashMap::new();
        weights.insert("success", 1.0);
        weights.insert("speed", 0.2);
        weights.insert("diversity", 0.1);
        // On construit le vocabulaire à partir de l’historique pour la vectorisation sémantique
        let mut corpus = Vec::new();
        let feedback_content = std::fs::read_to_string("routing_feedback.json").unwrap_or_default();
        for line in feedback_content.lines() {
            if let Ok(entry) = serde_json::from_str::<serde_json::Value>(line) {
                if let Some(input_hist) = entry.get("input").and_then(|v| v.as_str()) {
                    corpus.push(input_hist.to_string());
                }
            }
        }
        // Ajoute un mot bidon pour éviter vocab vide
        if corpus.is_empty() {
            corpus.push("vide".to_string());
        }
        let vectorizer = TfVectorizer::fit(&corpus);

        // plus de factory globale, tout est paresseux
        Self {
            vectorizer,
            clusters: Vec::new(),
            weights,
        }
    }

    /// Distance euclidienne entre deux vecteurs
    fn euclidean(a: &[f32], b: &[f32]) -> f32 {
        a.iter()
            .zip(b.iter())
            .map(|(x, y)| (x - y).powi(2))
            .sum::<f32>()
            .sqrt()
    }

    /// Charger les clusters depuis l’historique (création/suppression dynamique)
    fn load_clusters(&mut self) {
        self.clusters.clear();
        let feedback_content = std::fs::read_to_string("routing_feedback.json").unwrap_or_default();
        for line in feedback_content.lines() {
            if let Ok(entry) = serde_json::from_str::<serde_json::Value>(line) {
                if let Some(input_hist) = entry.get("input").and_then(|v| v.as_str()) {
                    let vec = self.vectorizer.vectorize(input_hist);
                    let mut experts = HashMap::new();
                    if let Some(selected) = entry.get("selected_experts").and_then(|v| v.as_array())
                    {
                        for e in selected {
                            if let Some(name) = e.as_str() {
                                let mut score = Score::default();
                                // Score multi-critères (à enrichir)
                                score.success = if entry.get("feedback").and_then(|v| v.as_str())
                                    == Some("oui")
                                {
                                    1.0
                                } else {
                                    0.0
                                };
                                score.count = 1;
                                experts.insert(name.to_string(), score);
                            }
                        }
                    }
                    self.clusters.push(Cluster {
                        centroid: vec,
                        experts,
                    });
                }
            }
        }
        // Fusionner les clusters proches (auto-organisation, à améliorer)
        // TODO: implémenter fusion/suppression dynamique si trop proches ou trop petits
    }

    /// Sélection dynamique d’experts par similarité sémantique et scoring multi-critères
    pub fn select_experts(&mut self, input: &str) -> Vec<String> {
        self.load_clusters();
        let input_vec = self.vectorizer.vectorize(input);

        // Recherche du cluster le plus proche (euclidienne)
        let (best_idx, best_dist) = self
            .clusters
            .iter()
            .enumerate()
            .map(|(i, c)| (i, Self::euclidean(&input_vec, &c.centroid)))
            .min_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
            .unwrap_or((usize::MAX, f32::MAX));

        let mut selected_names = Vec::new();
        let mut rule_applied = None;
        if best_dist < 50.0 && best_idx != usize::MAX {
            // Sélection multi-critères
            let scores = &self.clusters[best_idx].experts;
            let mut sorted: Vec<_> = scores.iter().collect();
            sorted.sort_by(|a, b| {
                b.1.total(&self.weights)
                    .partial_cmp(&a.1.total(&self.weights))
                    .unwrap()
            });
            for (name, _) in sorted.iter().take(3) {
                // On ne retient que les experts pour lesquels un factory existe (via get_factories)
                let factories = get_factories(&[name]);
                if factories.contains_key(*name) {
                    selected_names.push(name.to_string());
                }
            }
            rule_applied = Some("semantic_cluster_scoring".to_string());
            println!(
                "[Router] Routage sémantique par cluster (dist={:.2}) : {:?}",
                best_dist, selected_names
            );
        } else {
            // Cas jamais vu : recherche du plus proche expert par similarité directe
            // fallback désactivé : pas de sélection directe d'expert inconnu
            // On ne connaît pas la liste des experts, donc on peut demander à la config ou à un listing statique, ou ignorer ce fallback
            // Pour l'instant, on ne sélectionne rien
            // TODO: Optionnellement, fournir une méthode pour lister tous les experts connus si besoin
            println!("[Router] Aucun expert sélectionné (aucune règle intelligente trouvée).");
        }

        // Log du routage pour apprentissage futur (on peut garder ce log, mais pas de println!)
        let log_entry = serde_json::json!({
            "input": input,
            "rule": rule_applied,
            "selected_experts": selected_names
        });
        if let Ok(mut file) = OpenOptions::new()
            .append(true)
            .create(true)
            .open("routing_history.json")
        {
            let _ = writeln!(file, "{}", log_entry);
        }
        selected_names
    }

    /// Route l'entrée : sélectionne les experts, les instancie et exécute leur process
    pub fn route(&mut self, input: &str) -> Vec<(String, Result<String, String>, u128)> {
        println!("[Router] Début du routage pour l'entrée : '{input}'");
        let selected_names = self.select_experts(input);
        println!("[Router] Experts sélectionnés : {:?}", selected_names);

        // Résolution récursive des dépendances via le registry (plus aucun match ni référence statique)
        let mut all_needed = std::collections::HashSet::new();
        fn resolve_deps(name: &str, acc: &mut std::collections::HashSet<String>) {
            if !acc.insert(name.to_string()) {
                return; // déjà traité
            }
            for dep in get_dependencies(name) {
                resolve_deps(dep, acc);
            }
        }
        for name in &selected_names {
            resolve_deps(name, &mut all_needed);
        }

        // Instanciation paresseuse de tous les experts nécessaires d'un coup
        let needed_vec: Vec<&str> = all_needed.iter().map(|s| s.as_str()).collect();
        let factories = get_factories(&needed_vec);

        // Exécution des experts strictement nécessaires (sélectionnés + dépendances, sans doublons)
        let mut results = Vec::new();
        for name in all_needed {
            let start = std::time::Instant::now();
            let res = match factories.get(&name) {
                Some(f) => {
                    let expert = f();
                    expert.process(input).map_err(|err| err.to_string())
                }
                None => Err("Expert non trouvé".to_string()),
            };
            let duration = start.elapsed().as_millis();
            results.push((name, res, duration));
        }
        println!("[Router] Fin du routage pour l'entrée : '{input}'");
        results
    }
}
