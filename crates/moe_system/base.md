# Architecture Hi-MoE : Pipeline et conventions

## 1. Pipeline général
```text
Input → Orchestrateur
    → choisit le routeur principal (mais ne route pas lui-même)
    → Routeur_k(Input_encodé)
            → Top-k experts
            → Appels experts (parallélisés)
            → Agrégation locale (pondérée par gates)
            → Output_k, Meta_k (scores, coûts, latences…)
    → (Optionnel) Routeur_j en shadow (mêmes étapes, pas visible utilisateur)
    → Orchestrateur.Synthèse([Output_k, Meta_k], [Shadow_*?])
    → Réponse finale + Feedback vers routeurs/experts
```

---
## 3. Crédit/Apprentissage (sans bypass)

Router.loss = task_loss + λ·load_balance + μ·entropy

task_loss : renvoyée par l’orchestrateur (via qualité finale / reward).

load_balance : évite la “mode collapse” (tous sur le même expert).

entropy : encourage diversité de gating.

Expert.loss (NN) : backprop standard si sélectionné.
Expert.score (symbolique) : reward style bandit (latence/qualité).

Shadow routing : l’orchestrateur compare primary vs shadow (offline credit assignment) → met à jour router.gate et experts sans influencer la réponse utilisateur.


---

## 4. Points clés d’implémentation

Jamais de bypass : l’orchestrateur n’appelle pas d’expert directement. Tout passe par un Router.

Agrégation locale au Routeur : cohérent avec le gating (le routeur connaît ses poids).

Synthèse finale à l’Orchestrateur : utile si tu enchaînes plusieurs routeurs (multi-étapes) ou si tu combines la sortie primaire avec de la mémoire/contexte/contraintes globales.

Hétérogénéité experts : impose un Value commun (p.ex. enum structuré) + un adapter léger côté routeur pour normaliser (ex : texte ↔ plan symbolique).

Tracing & budget : mets latence/coût dans ExpertAux et AggregationMetadata pour réguler le top-k, stopper tôt, ou re-router si SLA menacé.



---

## 5. Boucle d’exécution (résumée)

```rust
r = orch.choose_router(x)

// À l'appelant de préparer le RouterInput selon le type de router (neuronal : encode, symbolique : raw, etc.)
// Exemples :
// let router_input = RouterInput::Encoded(encoder.encode(&x)?);
// let router_input = RouterInput::Raw(x.clone());

scores = r.gate(&router_input, ctx) → topk = r.pick_topk(scores, k)

calls = r.call_experts(&router_input, topk, ctx) (parallèle)

agg = r.aggregate(calls, scores)

(optionnel) répéter 2-5 sur shadow routers

y = orch.synthesize((r, agg, meta), shadow?)

orch.feedback(...) → r.train_signal(...) → experts update (sélectionnés)
```

---

## 6. Métriques/guardrails essentiels

Quality (task-specific), Calibration (confiance vs exactitude), Load balance, Utilisation par expert, Latence, Coût, Stabilité du gating (drift), Contradiction inter-experts (désaccord utile comme signal d’incertitude).

---

# Bonnes pratiques pour Value et Context (ajout simple, non bloquant)

## Value (sorties hétérogènes des experts)

- Définis un enum Value central (ex : enum Value { Texte(String), Plan(PlanStruct), Structure(Struct), ... }) dans un module partagé.
- Pour chaque expert, ajoute une conversion explicite (implémentation de From<SortieExpert> for Value ou un petit trait Adapter).
- Le routeur convertit systématiquement les sorties d’experts en Value.
- Ajoute des helpers (as_text, as_plan, etc.) pour faciliter l’usage.
- Documente les variantes de Value dans le code.

## Context (mémoire, budget, trace, etc.)

- Structure Context comme une struct avec des champs typés (Budget, Trace, Deadline, etc.).
- Si possible, rends Context immuable (chaque modif crée une nouvelle instance, via clone ou Arc/Rc).
- Pour la trace/mémoire longue, ajoute un mécanisme de nettoyage (TTL, LRU, etc.).
- Si besoin, découpe Context en sous-structs (ex : Context { mémoire: Mémoire, budget: Budget, ... }).
- Ajoute des tests simples pour vérifier la cohérence de Context.

Ces ajouts sont progressifs et n’imposent pas de tout changer d’un coup. Ils rendent le système plus robuste sans complexifier l’existant.

---

# Convention stricte de structuration des fichiers et dossiers

**Cette convention est essentielle et doit être respectée dans tout le projet.**

- Chaque grande composante a son dossier dédié : `orchestrator/`, `router/`, `experts/`.
- Les experts sont organisés par domaine dans des sous-dossiers : par exemple `experts/nlp/`, `experts/vision/`, etc.
- Chaque struct ou enum est définie dans un fichier qui porte son nom en snake_case : par exemple, la struct `AggregatedOut` va dans `aggregated_out.rs`.
- Chaque trait est défini dans un fichier nommé `xxx_trait.rs` où `xxx` est le nom du trait en snake_case : par exemple, le trait `Orchestrator` va dans `orchestrator_trait.rs`.
- Cette organisation permet une navigation claire, une évolutivité maximale et évite les conflits ou la confusion lors de l’ajout de nouvelles fonctionnalités.

**Tout nouveau code ou refactor doit suivre cette convention.**

---




> Seuls les experts métiers (par exemple NlpFrenchTagger) sont exposés directement au routeur via le trait `Expert`. Les modèles (neuronaux, symboliques, etc.) et autres composants internes sont utilisés comme dépendances par les experts, orchestrators ou routers, mais ne sont jamais exposés comme agents autonomes dans le pipeline MoE. Cette séparation garantit une interface claire, typée et stable pour le routage, tout en permettant une composition flexible des briques internes (symboliques, neuronales, hybrides, etc.).

---

#### Note :

Cette logique d'encapsulation des composants internes (modèles, moteurs, etc.) s'applique aussi à Orchestrator et Router : ils peuvent être symboliques, neuronaux, hybrides, etc. L’important est d’exposer une interface claire et typée, quel que soit l’agent ou la technologie interne.

---

# Perfectionnement MoE : patterns avancés (obligatoires)


Ces patterns sont à appliquer dès le départ pour garantir la robustesse, la scalabilité et le future-proof du MoE. Ils font partie intégrante du standard du projet.


## 1. Séparation stricte des rôles et encapsulation

- Les experts exposés au routeur implémentent le trait `Expert` et sont les seuls visibles pour le pipeline principal.
- Les modèles, moteurs ou autres composants internes sont utilisés comme dépendances, jamais exposés comme agents autonomes.

## 2. Contrats d’interface explicites et versionnés

- `InputData` : helpers (`as_text`, `from_text`, etc.), type d’entrée unique pour tous les experts.
- `Value` : variantes claires, possibilité d’évolution via un champ de version.

## 3. Budget/Deadline guards natifs

- Le routeur vérifie `ctx.deadline_ms` et coupe tôt si besoin.
- Option : réduire dynamiquement top_k si le budget chute.

## 4. Gating déterministe & stable

- Softmax( logits / temperature ).
- Tie-break stable (tri + hash expert_id) pour éviter le jitter.
- Ajoute une proba “explore” min (ε-greedy) pour lutter contre le mode-collapse.

## 5. Load-balancing régularisé

```rust
// Exemple : loss_lb = α * (variance(utilisation_experts) / mean^2)
```
Loggue `utilisation_experts` dans `AggregationMetadata` pour suivre le drift.

## 6. Télémetrie minimale (traçabilité)

- `trace_id` dans `Context`, propagé jusqu’à chaque `ExpertAux`.
- `AggregationMetadata` : `entropy`, `topk`, `lat_total_ms`, `drop_count` (experts non appelés faute de budget).

## 7. Shadow routing cloisonné

- Exécute les shadows après la voie primaire si budget restant > seuil.
- Feedback “offline” (jamais dans le chemin critique).

---


> Ces patterns sont obligatoires pour tout développement ou refactor du MoE. Ils assurent la solidité, la maintenabilité et la capacité d’évolution du système dès le départ.