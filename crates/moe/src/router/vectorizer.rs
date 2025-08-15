use std::collections::{HashMap, HashSet};

pub trait Vectorizer {
    fn vectorize(&self, input: &str) -> Vec<f32>;
    fn vocab(&self) -> &Vec<String>;
}

/// Vectoriseur TF (bag-of-words dense, pure Rust, pas un mock)
pub struct TfVectorizer {
    vocab: Vec<String>,
    vocab_index: HashMap<String, usize>,
}

impl TfVectorizer {
    /// Construit le vocabulaire à partir d'un corpus (ensemble de textes)
    pub fn fit(corpus: &[String]) -> Self {
        let mut vocab_set = HashSet::new();
        for doc in corpus {
            for token in doc.split_whitespace() {
                vocab_set.insert(token.to_lowercase());
            }
        }
        let vocab: Vec<String> = vocab_set.into_iter().collect();
        let vocab_index = vocab
            .iter()
            .enumerate()
            .map(|(i, w)| (w.clone(), i))
            .collect();
        Self { vocab, vocab_index }
    }
}

impl Vectorizer for TfVectorizer {
    fn vectorize(&self, input: &str) -> Vec<f32> {
        let mut vec = vec![0.0; self.vocab.len()];
        for token in input.split_whitespace() {
            if let Some(&idx) = self.vocab_index.get(&token.to_lowercase()) {
                vec[idx] += 1.0;
            }
        }
        vec
    }
    fn vocab(&self) -> &Vec<String> {
        &self.vocab
    }
}
