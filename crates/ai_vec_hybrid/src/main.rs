use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
#[derive(Deserialize)]
struct Fact {
    subj: String,
    rel: String,
    obj: String,
}

// --------- Embeddings (toy) ---------

const D: usize = 16; // dimension du vecteur

fn l2_normalize(v: &mut [f32]) {
    let mut n2 = 0.0;
    for x in v.iter() {
        n2 += x * x;
    }
    let n = n2.sqrt().max(1e-9);
    for x in v.iter_mut() {
        *x /= n;
    }
}

fn embed_text(s: &str) -> [f32; D] {
    // hash-binning super simple + n-grams (1..3)
    let mut v = [0f32; D];
    let bytes = s.as_bytes();
    for n in 1..=3 {
        for w in bytes.windows(n) {
            // cheap hash
            let mut h: u32 = 2166136261;
            for &b in w {
                h = h ^ (b as u32);
                h = h.wrapping_mul(16777619);
            }
            let idx = (h as usize) % D;
            v[idx] += 1.0;
        }
    }
    l2_normalize(&mut v);
    v
}

fn cosine(a: &[f32], b: &[f32]) -> f32 {
    let mut dot = 0.0;
    let mut na = 0.0;
    let mut nb = 0.0;
    for i in 0..a.len() {
        dot += a[i] * b[i];
        na += a[i] * a[i];
        nb += b[i] * b[i];
    }
    dot / (na.sqrt() * nb.sqrt() + 1e-9)
}

// --------- Graphe + index ---------

#[derive(Clone)]
struct Node {
    id: usize,
    name: String,
    emb: [f32; D],
}

#[derive(Clone)]
struct Edge {
    from: usize,
    rel: String,
    to: usize,
}

struct Graph {
    nodes: Vec<Node>,
    name2id: HashMap<String, usize>,
    edges: Vec<Edge>,
}

impl Graph {
    fn new() -> Self {
        Self {
            nodes: vec![],
            name2id: HashMap::new(),
            edges: vec![],
        }
    }

    fn add_node(&mut self, name: &str) -> usize {
        if let Some(&id) = self.name2id.get(name) {
            return id;
        }
        let id = self.nodes.len();
        let emb = embed_text(name);
        self.nodes.push(Node {
            id,
            name: name.to_string(),
            emb,
        });
        self.name2id.insert(name.to_string(), id);
        id
    }

    fn set_node_embedding_ema(&mut self, name: &str, new_text: &str, alpha: f32) {
        if let Some(&id) = self.name2id.get(name) {
            let new_emb = embed_text(new_text);
            let node = &mut self.nodes[id];
            for i in 0..D {
                node.emb[i] = (1.0 - alpha) * node.emb[i] + alpha * new_emb[i];
            }
            l2_normalize(&mut node.emb);
        }
    }

    fn add_edge(&mut self, from_name: &str, rel: &str, to_name: &str) {
        let from = self.add_node(from_name);
        let to = self.add_node(to_name);
        self.edges.push(Edge {
            from,
            rel: rel.to_string(),
            to,
        });
    }

    fn remove_edge_exact(&mut self, from_name: &str, rel: &str, to_name: &str) {
        let from = match self.name2id.get(from_name) {
            Some(&i) => i,
            None => return,
        };
        let to = match self.name2id.get(to_name) {
            Some(&i) => i,
            None => return,
        };
        self.edges
            .retain(|e| !(e.from == from && e.to == to && e.rel == rel));
    }

    fn update_fact(
        &mut self,
        subj: &str,
        old_rel: &str,
        old_obj: &str,
        new_rel: &str,
        new_obj: &str,
    ) {
        // supprime l’ancien fait puis ajoute le nouveau
        self.remove_edge_exact(subj, old_rel, old_obj);
        self.add_edge(subj, new_rel, new_obj);
    }

    fn outgoing(&self, name: &str) -> Vec<&Edge> {
        let Some(&id) = self.name2id.get(name) else {
            return vec![];
        };
        self.edges.iter().filter(|e| e.from == id).collect()
    }

    fn k_nn(&self, q: &[f32; D], k: usize) -> Vec<(usize, f32)> {
        let mut sims: Vec<(usize, f32)> = self
            .nodes
            .iter()
            .map(|n| (n.id, cosine(&n.emb, q)))
            .collect();
        sims.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        sims.truncate(k);
        sims
    }

    fn search_text(&self, query: &str, k: usize) -> Vec<(String, f32)> {
        let q = embed_text(query);
        self.k_nn(&q, k)
            .into_iter()
            .map(|(id, s)| (self.nodes[id].name.clone(), s))
            .collect()
    }
}

// --------- Démo ---------

fn print_facts(g: &Graph, name: &str) {
    let outs = g.outgoing(name);
    if outs.is_empty() {
        println!("  (aucun fait sortant pour {name})");
    } else {
        for e in outs {
            let to = &g.nodes[e.to].name;
            println!("  {name} --{}--> {}", e.rel, to);
        }
    }
}

fn main() {
    // Charger les faits depuis facts.json
    let data = fs::read_to_string("crates/ai_vec_hybrid/facts.json")
        .expect("Impossible de lire facts.json");
    let facts: Vec<Fact> = serde_json::from_str(&data).expect("JSON mal formé");

    let mut g = Graph::new();
    for fact in &facts {
        g.add_edge(&fact.subj, &fact.rel, &fact.obj);
    }

    println!("== Recherche vectorielle (avant correction) ==");
    for (name, score) in g.search_text("pluton", 5) {
        let node = g.nodes.iter().find(|n| n.name == name).unwrap();
        println!("  match: {name:>12}  sim={score:.3}  emb={:?}", node.emb);
    }

    println!("\n== Faits sur Pluton (avant) ==");
    print_facts(&g, "Pluton");

    // Correction d’un fait : Pluton est_une planète -> Pluton est_une planète_naine
    println!("\n>> Correction : 'Pluton est_une planète'  ->  'Pluton est_une planète_naine'");
    g.update_fact("Pluton", "est_une", "planète", "est_une", "planète_naine");

    println!("\n== Faits sur Pluton (après correction) ==");
    print_facts(&g, "Pluton");

    // Mise à jour embedding locale
    println!("\n>> Mise à jour embedding locale de 'Pluton' (alpha=0.3) avec texte : 'dwarf planet kuiper belt object'");
    g.set_node_embedding_ema("Pluton", "dwarf planet kuiper belt object", 0.3);

    println!("\n== Recherche vectorielle (après correction + EMA locale) ==");
    for (name, score) in g.search_text("pluton dwarf planet", 5) {
        let node = g.nodes.iter().find(|n| n.name == name).unwrap();
        println!("  match: {name:>12}  sim={score:.3}  emb={:?}", node.emb);
    }

    // Statut courant via le graphe
    println!("\n== Statut de Pluton via le graphe ==");
    let outs = g.outgoing("Pluton");
    let mut statut: Option<String> = None;
    for e in outs {
        if e.rel == "est_une" {
            statut = Some(g.nodes[e.to].name.clone());
        }
    }
    println!(
        "  Pluton est_une -> {}",
        statut.unwrap_or_else(|| "(inconnu)".into())
    );

    println!("\nOK.");
}
