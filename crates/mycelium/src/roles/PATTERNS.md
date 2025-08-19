# Patterns d’extension des rôles dans le système

## Ajouter un nouveau rôle (exemple : Reviewer)

1. Créer un dossier et un fichier pour le rôle :
   - `roles/reviewer/reviewer.rs`
   - `roles/reviewer/mod.rs` avec `pub use reviewer::Reviewer;`
2. Définir un enum pour le rôle et ses variantes.
3. Implémenter les méthodes associées (ex : `review`).
4. Ajouter le module dans `roles/mod.rs` et ré-exporter l’enum avec `pub use`.
5. Ajouter une variante dans l’enum central `Roles` (ex : `Reviewer(Reviewer)`).
6. Étendre le router central (`role_enum_to_action_call.rs`) pour dispatcher ce rôle.
7. Utiliser le rôle dans `main.rs` ou ailleurs via `Roles::Reviewer(Reviewer::...)`.

## Exemple de code
```rust
// Ajout dans l’enum Roles
Reviewer(Reviewer),

// Dispatch dans le router
Roles::Reviewer(reviewer) => Box::new(reviewer.review(input)),

// Utilisation
roles_set.insert(Roles::Reviewer(Reviewer::SimpleReview));
```

## Bonnes pratiques
- Toujours dériver `Serialize`, `Deserialize`, `Debug`, `Clone`, `PartialEq`, `Eq`, `Hash` sur les enums de rôles.
- Utiliser `pub use` dans les mod.rs pour une API ergonomique.
- Documenter chaque rôle et ses variantes.
- Ajouter des tests unitaires pour chaque rôle et pour le router central.
