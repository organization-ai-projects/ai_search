# Rust AI Search

Ce repository est un workspace Rust multi-crates pour des projets d'IA et d'expérimentation.

## Structure du workspace
- `Cargo.toml` : configuration du workspace et dépendances globales
- `crates/` : contient toutes les crates du projet
	- Chaque sous-dossier dans `crates/` est une crate indépendante (ex : `ai_vec_hybrid`, `ai_vm`, `mycelium`, ...)
	- Chaque crate possède son propre `Cargo.toml` et son dossier `src/`

## Commandes utiles
- `cargo build` : compiler toutes les crates du workspace
- `cargo run -p <nom_de_la_crate>` : exécuter la crate spécifiée
- `cargo test -p <nom_de_la_crate>` : lancer les tests pour une crate
- `cargo check` : vérifier la compilation sans générer de binaire

## Prérequis
- [Rust](https://www.rust-lang.org/tools/install) doit être installé

---

N'hésitez pas à modifier ce fichier pour ajouter des informations spécifiques à chaque crate ou à la documentation globale du workspace.
