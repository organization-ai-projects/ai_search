//! Index global du système MoE
// Ce fichier définit la structure d'indexation globale, accessible à toutes les entités du pipeline.
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::io::{self, Read, Write};

/// Entrée d'index globale : un type d'entrée (entry_kind), un ou plusieurs tags, un chemin
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct GlobalIndexEntry {
    pub entry_kind: String,
    pub tags: Vec<String>,
    pub path: String,
}

/// Index global du système MoE : liste plate d'entrées (entity, tag, path)
#[derive(Default, Serialize, Deserialize, Debug, Clone)]
pub struct GlobalIndex {
    pub entries: Vec<GlobalIndexEntry>,
}

impl GlobalIndex {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }

    /// Ajoute ou met à jour une entrée (entry_kind, tags, path)
    pub fn insert(&mut self, entry_kind: &str, tags: Vec<String>, path: &str) {
        if let Some(e) = self
            .entries
            .iter_mut()
            .find(|e| e.entry_kind == entry_kind && e.path == path)
        {
            // Fusionner les tags sans doublons
            for tag in tags {
                if !e.tags.contains(&tag) {
                    e.tags.push(tag);
                }
            }
        } else {
            self.entries.push(GlobalIndexEntry {
                entry_kind: entry_kind.to_string(),
                tags,
                path: path.to_string(),
            });
        }
    }

    /// Récupère le chemin d'une instance par entry_kind et tag (si le tag est présent dans l'entrée)
    pub fn get_path(&self, entry_kind: &str, tag: &str) -> Option<&str> {
        self.entries
            .iter()
            .find(|e| e.entry_kind == entry_kind && e.tags.contains(&tag.to_string()))
            .map(|e| e.path.as_str())
    }

    /// Retourne la liste des tags pour un entry_kind
    pub fn tags_for_kind(&self, entry_kind: &str) -> Vec<&str> {
        self.entries
            .iter()
            .filter(|e| e.entry_kind == entry_kind)
            .flat_map(|e| e.tags.iter().map(|t| t.as_str()))
            .collect()
    }

    /// Retourne la liste des entry_kinds connus
    pub fn entry_kinds(&self) -> Vec<&str> {
        let mut v = Vec::new();
        for e in &self.entries {
            if !v.contains(&e.entry_kind.as_str()) {
                v.push(e.entry_kind.as_str());
            }
        }
        v
    }

    /// Sauvegarde l'index global dans un fichier JSON
    pub fn save_to_file(&self, path: &str) -> io::Result<()> {
        let json = serde_json::to_string_pretty(self)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
        let mut file = fs::File::create(path)?;
        file.write_all(json.as_bytes())
    }

    /// Charge l'index global depuis un fichier JSON
    pub fn load_from_file(path: &str) -> io::Result<Self> {
        let mut file = fs::File::open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let index =
            serde_json::from_str(&contents).map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
        Ok(index)
    }
}
