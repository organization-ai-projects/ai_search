# quantum_ai_futur — Réseau neuronal minimaliste (documentation)

Ce crate fournit une implémentation pédagogique d'un petit réseau neuronal à deux couches
fully-connected (input -> hidden -> output) en Rust, basée sur `ndarray` pour les tenseurs.

Objectif
- Fournir un exemple clair de forward/backprop (SGD) utilisable comme base pour expérimenter
  des architectures plus complexes.

Contenu
- `src/main.rs` : réseau, entraînement (XOR), évaluation et interface CLI simple.
- `Cargo.toml` : manifest du crate (dépendance `ndarray`).

Compilation et exécution
Depuis la racine du workspace (recommandé) :

```bash
cargo build -p quantum_ai_futur
cargo run -p quantum_ai_futur -- [epochs] [batch_size] [lr]
```

Ou depuis le répertoire du crate :

```bash
cd crates/quantum_ai_futur
cargo run -- [epochs] [batch_size] [lr]
```

Paramètres CLI
- epochs (par défaut 10000) : nombre d'itérations d'entraînement.
- batch_size (par défaut 4) : taille de batch; si 0 alors on utilise tout le dataset.
- lr (par défaut 0.8) : taux d'apprentissage.

Exemple

```bash
cargo run -p quantum_ai_futur -- 5000 2 0.5
```

Comportement attendu
- Pendant l'entraînement, le programme affiche périodiquement l'état (loss/accuracy).
- À la fin, les prédictions pour les entrées XOR sont affichées.

API et architecture
- Le réseau est défini dans `NeuralNetwork` avec :
  - `w1: Array2<f64>` poids entre input et hidden
  - `b1: Array1<f64>` biais hidden
  - `w2: Array2<f64>` poids entre hidden et output
  - `b2: Array1<f64>` biais output

- Fonctions importantes :
  - `forward(input) -> (hidden, output)` : propagation avant
  - `train_epoch(inputs, targets)` : applique SGD échantillon par échantillon
  - `train(inputs, targets, epochs, batch_size)` : boucle d'entraînement avec batching
  - `evaluate(inputs, targets) -> (loss, accuracy)` : évalue le modèle

Pourquoi `ndarray` ?
- `ndarray` simplifie la manipulation des vecteurs/matrices (dot, axes, broadcasting).
- Pour un usage IA réel, `ndarray` rend le code plus lisible et permet d'exploiter des
  opérations vectorielles plus efficaces.

Limitations et évolutions possibles
- L'entraînement est basique : SGD sans shuffle, sans mini-batch aléatoire, ni optimiseur
  avancé (Adam, RMSProp).
- Pas de sauvegarde/chargement des poids. On peut ajouter `serde` pour sérialiser les poids.
- Pas d'early-stopping, pas de gestion des jeux de données externes.

Propositions d'améliorations (je peux les implémenter)
- Utiliser `ndarray` pour le batching complet (Array2 pour les entrées batched).
- Ajouter un optimiseur (Adam) et shuffle des batches.
- Ajouter tests unitaires (forward + backward) et bench simples.
- Intégrer le crate dans le workspace racine (section `[workspace] members`) si voulu.

Si tu veux, je peux commencer par :
- convertir l'entraînement pour supporter des batches vectorisés,
- ajouter serialization des poids,
- ou ajouter tests et CI minimal.

Dis-moi quelle amélioration prioriser.
