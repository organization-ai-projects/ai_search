# Convention d’extension des rôles dans le mycelium

## 1. Un domaine = un dossier + un enum
Chaque domaine fonctionnel (ex : MonDomaine, AutreDomaine, etc.) a :
- Un dossier `roles/<domaine>/`
- Un fichier `roles/<domaine>/<domaine>.rs` avec un enum `<Domaine>` listant les variantes du rôle
- Un mod.rs qui fait `pub use <domaine>::<Domaine>;`

## 2. Extension d’un domaine
Pour ajouter une capacité dans un domaine, il suffit d’ajouter une variante à l’enum du domaine (ex : `MonDomaine::VarianteA`, `MonDomaine::VarianteB`, etc.)

## 3. Ajout d’un nouveau domaine
1. Créer le dossier et le fichier enum
2. Ajouter le module dans `roles/mod.rs` et le ré-exporter avec `pub use`
3. Ajouter une variante dans l’enum central `Roles` : `Roles::MonDomaine(MonDomaine)`
4. Étendre le dispatch central (ex : `role_enum_to_action_call.rs`) pour router la variante

## 4. Utilisation
On utilise toujours les rôles via l’enum central `Roles` :
```rust
roles_set.insert(Roles::MonDomaine(MonDomaine::VarianteA));
roles_set.insert(Roles::AutreDomaine(AutreDomaine::VarianteX));
```

## 5. Bonnes pratiques
- Un seul enum par domaine, pas de doublons
- Toujours dériver Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash
- Utiliser pub use dans les mod.rs pour une API ergonomique
- Documenter chaque domaine et chaque variante
- Ajouter des tests unitaires pour chaque domaine et pour le dispatch central
