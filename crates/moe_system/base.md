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

## 2. Interfaces (pseudo-Rust, orienté traits)

### 2.1 Type d'entrée unique : `InputData`
Peut contenir du texte, des features, des vecteurs, etc.
```rust
#[derive(Clone, Debug)]
pub enum InputData {
    Text(String),
    Bytes(Vec<u8>),
    Features(Vec<f32>),
    // Ajoute d'autres variantes selon tes besoins
}

impl InputData {
    pub fn as_text(&self) -> Option<&str> {
        match self { InputData::Text(s) => Some(s), _ => None }
    }
    // Ajoute d'autres helpers si besoin
}
```



### 2.2 Gestion d'erreur normalisée
```rust
pub type MoeResult<T> = Result<T, MoeError>;

#[derive(thiserror::Error, Debug)]
pub enum MoeError {
    #[error("deadline exceeded")]
    DeadlineExceeded,
    #[error("budget exceeded")]
    BudgetExceeded,
    #[error("encode error: {0}")]
    EncodeError(String),
    #[error("expert {name} failed: {cause}")]
    ExpertFailed { name: &'static str, cause: String },
    #[error("no expert selected")]
    NoExpertSelected,
    #[error("aggregation failed: {0}")]
    AggregationFailed(String),
}

pub struct Encoded(pub Vec<f32>); // alias simple, change plus tard si besoin

pub trait Encoder: Send + Sync {
    fn encode(&self, x: &InputData) -> MoeResult<Encoded>;
}
```

### 2.3 Enum d'entrée pour router symbolique, neuronal, hybride…

```rust
#[derive(Clone, Debug)]
pub enum RouterInput {
    Encoded(Encoded),
    Raw(InputData),
    // Ajoute d'autres variantes si besoin (ex: features, tokens…)
}
```

### 2.4 Contexte partagé (budget, trace, etc.)

```rust
/// Contexte partagé pour la gestion du budget, du tracing, etc.
pub struct Context {
    pub budget_ms: u64,
    pub deadline_at: std::time::Instant,
    pub trace_id: String,
    pub cancel: CancelToken, // défini en 2.7
    // mémoire, kv, etc.
}

pub fn remaining_ms(ctx: &Context) -> Option<u64> {
    ctx.deadline_at.checked_duration_since(std::time::Instant::now())
        .map(|d| d.as_millis() as u64)
}
```

### 2.5 Stratégie d’agrégation indépendante du router (injection de politique)
```rust
pub trait Aggregator: Send + Sync {
    fn id(&self) -> &'static str;
    fn combine(
        &self,
        calls: &[(ExpertRef, ExpertOut)],
        scores: &GateScores
    ) -> MoeResult<AggregatedOut>;
}

/// Exemple : agrégateur par défaut (texte/plan/json, pondération simple)
pub struct DefaultAggregator;
impl Aggregator for DefaultAggregator {
    fn id(&self) -> &'static str { "default" }
    fn combine(
        &self,
        calls: &[(ExpertRef, ExpertOut)],
        scores: &GateScores
    ) -> MoeResult<AggregatedOut> {
        // Ex : pondération softmax, sélection du meilleur, fallback None
        // À adapter selon Value (texte, plan, etc.)
        if calls.is_empty() {
            return Err(MoeError::NoExpertSelected);
        }
        // Ici, simple : on prend la sortie avec le meilleur score
        let (best_id, _) = scores.logits.iter().max_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal)).ok_or(MoeError::AggregationFailed("no scores".into()))?;
        let out = calls.iter().find(|(r,_)| r.id == *best_id).map(|(_,o)| o).ok_or(MoeError::AggregationFailed("no matching output".into()))?;
        Ok(AggregatedOut {
            value: out.value.clone(),
            mix_info: MixMeta {
                entropy: 0.0, // à calculer
                topk: calls.len(),
                lat_total_ms: calls.iter().map(|(_,o)| o.aux.latency_ms).sum(),
                drop_count: 0,
                util_by_expert: scores.logits.clone(),
            },
        })
    }
}

#[async_trait::async_trait]
pub trait Router: Send + Sync {
    /// Routing/gating sur une entrée générique (Encoded, Raw, etc.)
    fn gate(&self, input: &RouterInput, ctx: &Context) -> MoeResult<GateScores>;
    /// Sélectionne les k meilleurs experts, borne et non-vide, sinon erreur explicite.
    fn pick_topk(&self, scores: &GateScores, k: usize) -> MoeResult<Vec<ExpertRef>> {
        let n = scores.logits.len();
        let k = k.min(n).max(1);
        // Ici, on suppose qu'on a un mapping id->ExpertRef (à adapter selon l'impl)
        // Placeholder : retourne une erreur si aucun score
        if n == 0 {
            return Err(MoeError::NoExpertSelected);
        }
        // ... à implémenter dans chaque Router concret ...
        Err(MoeError::AggregationFailed("pick_topk: impl manquante".into()))
    }
    /// Appel des experts en parallèle avec timeout/budget (pattern deadline/cancel obligatoire)
    /// Toute implémentation doit garantir le parallélisme et le respect du budget/timeout pour chaque expert.
    async fn call_experts(&self, input: &RouterInput, picks: &[ExpertRef], ctx: &Context) -> MoeResult<Vec<(ExpertRef, ExpertOut)>>;
    /// Délègue l’agrégation à l’Aggregator associé
    fn aggregator(&self) -> &dyn Aggregator;
    fn aggregate(&self, calls: &[(ExpertRef, ExpertOut)], scores: &GateScores) -> MoeResult<AggregatedOut> {
        self.aggregator().combine(calls, scores)
    }
    fn train_signal(&mut self, fb: RouterFeedback);
}
```

### 2.6 Référence thread-safe à un routeur
```rust
pub struct RouterRef {
    pub id: String,
    pub handle: std::sync::Arc<dyn Router>,
}

pub trait Orchestrator: Send + Sync {
    fn choose_router(&self, x: &InputData, ctx: &Context) -> MoeResult<RouterRef>;

    fn synthesize(
        &self,
        primary: (&RouterRef, AggregatedOut, MixMeta),
        shadow: Option<Vec<(&RouterRef, AggregatedOut, MixMeta)>>
    ) -> MoeResult<Value>;

    fn feedback(&mut self, fb: OrchestrationFeedback);
}
```

### 2.7 Token d'annulation thread-safe pour propagation du cancel (coopératif, sans lock)
```rust
/// Token d'annulation thread-safe pour propagation du cancel (coopératif, sans lock)
#[derive(Clone)]
pub struct CancelToken(pub std::sync::Arc<std::sync::atomic::AtomicBool>);
impl CancelToken {
    pub fn cancel(&self) { self.0.store(true, std::sync::atomic::Ordering::SeqCst) }
    pub fn is_cancelled(&self) -> bool { self.0.load(std::sync::atomic::Ordering::SeqCst) }
}
```
```text
Le token d'annulation (`CancelToken`) permet de propager un signal d'annulation de façon thread-safe et non bloquante à travers tout le pipeline (orchestrateur, routeur, experts). Il s'agit d'un pattern coopératif : chaque composant doit vérifier régulièrement l'état du token (`is_cancelled`) et interrompre proprement ses traitements si besoin (ex : timeout, budget dépassé, shutdown).

Ce mécanisme est essentiel pour garantir le respect des contraintes de budget/latence et éviter les fuites de ressources lors d'une exécution parallèle ou asynchrone.

Voir la section 2.4 pour l'utilisation de CancelToken dans Context et remaining_ms.
```

### 2.8 Sortie d'un expert
```rust
pub struct ExpertOut {
    pub value: Value,          // texte, plan, structure…
    pub aux: ExpertAux,        // latence, coût, confiance…
}
```


### 2.9 Trait unique pour tous les experts (asynchrone, standardisé)
```rust
#[async_trait::async_trait]
pub trait Expert: Send + Sync {
    fn name(&self) -> &'static str;
    fn can_handle(&self, task: &str) -> bool; // hint symbolique
    async fn infer(&self, x: &InputData, ctx: &Context) -> MoeResult<ExpertOut>;
}
```
```rust
/// Registry déterministe pour la gestion des experts (ExpertId stables, lookup rapide).
/// Permet d'assurer la stabilité des IDs et la découverte des experts par nom ou id.
pub trait ExpertRegistry: Send + Sync {
    /// Retourne la liste complète des experts connus (id + nom).
    fn list(&self) -> Vec<(ExpertId, &'static str)>;
    /// Lookup rapide par id (stable) ou nom.
    fn get_by_id(&self, id: ExpertId) -> Option<ExpertRef>;
    fn get_by_name(&self, name: &str) -> Option<ExpertRef>;
}

/// Exemple d'implémentation : id stable par hash du nom
/*
impl ExpertRegistry for MyRegistry {
    fn list(&self) -> Vec<(ExpertId, &'static str)> {
        self.experts.iter().map(|e| (ExpertId(hash(e.name())), e.name())).collect()
    }
    fn get_by_id(&self, id: ExpertId) -> Option<ExpertRef> { ... }
    fn get_by_name(&self, name: &str) -> Option<ExpertRef> { ... }
}
*/
```
### 2.10 Types auxiliaires référencés dans le pipeline (squelettes minimalistes)
```rust
/// Value = format commun que *tous* les experts doivent renvoyer.
/// Garde-le parcimonieux ; ajoute des variantes seulement si nécessaires.
#[derive(Clone, Debug)]
pub enum Value {
    /// Réponses textuelles / rationales
    Text { schema: u16, data: String },
    /// Plans symboliques structurés
    Plan { schema: u16, data: Plan },
    /// Résultats structurés ad hoc
    Json { schema: u16, data: serde_json::Value },
    /// Représentations vectorielles
    Embedding { schema: u16, data: Vec<f32> },
    /// Binaire (images compressées, audio, etc.)
    Bytes { schema: u16, data: Vec<u8> },
    /// Pour signifier "pas de résultat utile"
    None,
}

impl Value {
    /// Crée une nouvelle valeur textuelle versionnée (variant Text).
    /// Usage : Value::text(1, "foo")
    /// - schema : version du format (ex : 1)
    /// - s : contenu textuel
    pub fn text(schema: u16, s: impl Into<String>) -> Self {
        Value::Text { schema, data: s.into() }
    }

    /// Tente d'extraire une vue (&str) et la version (schema) si self est Text.
    /// Retourne Some((schema, &str)) ou None si ce n'est pas un texte.
    /// Usage : if let Some((schema, txt)) = v.as_text() { ... }
    pub fn as_text(&self) -> Option<(u16, &str)> {
        if let Value::Text { schema, data } = self {
            Some((*schema, data.as_str()))
        } else {
            None
        }
    }

    /// Indique si la valeur est None (pas de résultat utile).
    pub fn is_none(&self) -> bool { matches!(self, Value::None) }
}

/// Plan structuré pour la sortie symbolique
#[derive(Clone, Debug)]
pub struct Plan {
    pub goal: String,
    pub steps: Vec<PlanStep>,
}

#[derive(Clone, Debug)]
pub struct PlanStep {
    pub description: String,
    pub done: bool,
}

/// Trait d’adaptation vers Value
pub trait ToValue {
    fn to_value(self) -> Value;
}

impl ToValue for String {
    fn to_value(self) -> Value { Value::Text(self) }
}

impl ToValue for Plan {
    fn to_value(self) -> Value { Value::Plan(self) }
}
```
```rust
#[derive(Clone, Debug)]
pub struct AggregatedOut {
    /// Sortie unique après agrégation locale du routeur
    pub value: Value,
    /// Métadonnées de mélange / télémétrie
    pub mix_info: MixMeta,
}
```

```rust
#[derive(Clone, Debug)]
pub struct ExpertAux {
    pub latency_ms: u64,
    pub cost_units: f32,
    pub confidence: f32,
    pub trace_id: Option<String>,
}
```
```rust
#[derive(Clone, Debug)]
pub struct GateScores {
    // scores/softmax par expert_id
    pub logits: Vec<(ExpertId, f32)>,
}
```
```rust
#[derive(Clone, Debug)]
pub struct MixMeta {
    pub entropy: f32,
    pub topk: usize,
    pub lat_total_ms: u64,
    pub drop_count: usize,
    pub util_by_expert: Vec<(ExpertId, f32)>, // pour load-balance
}
```
```rust
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ExpertId(pub u64);
```
```rust
#[derive(Clone)]
pub struct ExpertRef {
    pub id: ExpertId,
    pub handle: std::sync::Arc<dyn Expert>,
}
```
```rust
/// Politique de gating standard (softmax stable, tie-break, epsilon-greedy)
#[derive(Clone, Debug)]
pub struct GatingPolicy {
    pub temperature: f32,  // >0
    pub epsilon: f32,      // 0..1, proba d’exploration min
    pub seed: u64,         // pour tie-break stable
}

pub fn softmax_stable(xs: &[f32], temp: f32) -> Vec<f32> {
    let invt = 1.0_f32 / temp.max(1e-6);
    let m = xs.iter().cloned().fold(f32::NEG_INFINITY, f32::max);
    let exps: Vec<f32> = xs.iter().map(|&x| ((x - m) * invt).exp()).collect();
    let sum = exps.iter().sum::<f32>().max(1e-12);
    exps.into_iter().map(|e| e / sum).collect()
}

/// Applique softmax + epsilon, puis tri avec tie-break stable.
pub fn gate_with_policy(
    scores: &[(ExpertId, f32)],
    pol: &GatingPolicy
) -> Vec<(ExpertId, f32)> {
    let ws = softmax_stable(&scores.iter().map(|(_, s)| *s).collect::<Vec<_>>(), pol.temperature);
    let mut v: Vec<(ExpertId, f32)> = scores.iter().zip(ws).map(|((id,_), w)| (*id, (1.0 - pol.epsilon)*w + pol.epsilon/(scores.len() as f32))).collect();
    // tri stable par poids puis par hash(id, seed)
    v.sort_by(|(a,wa), (b,wb)| {
        wb.partial_cmp(wa).unwrap_or(std::cmp::Ordering::Equal)
            .then_with(|| {
                let ha = a.0 ^ pol.seed;
                let hb = b.0 ^ pol.seed;
                ha.cmp(&hb)
            })
    });
    v
}
```
```rust
/// Feedback structuré pour l'apprentissage/monitoring
#[derive(Clone, Debug)]
pub struct RouterFeedback {
    pub trace_id: String,
    pub task_loss: f32,        // qualité finale (retour orch)
    pub load_balance_grad: f32,
    pub entropy_grad: f32,
    pub util_by_expert: Vec<(ExpertId, f32)>, // stats d'usage pour régul.
}
```
```rust
#[derive(Clone, Debug)]
pub struct OrchestrationFeedback {
    pub trace_id: String,
    pub primary_router: String,
    pub shadow_better: Option<String>, // id d’un router shadow gagnant
    pub notes: Option<String>,         // libre (erreurs, drift, etc.)
}
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

Tracing & budget : mets latence/coût dans ExpertAux et MixMeta pour réguler le top-k, stopper tôt, ou re-router si SLA menacé.



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



# Exemples d’implémentation d’experts (symbolique et neuronal)

## Expert symbolique (règles/planification)

```rust
// experts/planning/planner_rules.rs
pub struct RulePlanner {
    pub max_steps: usize,
}

impl RulePlanner {
    fn plan_from_prompt(&self, prompt: &str) -> Plan {
        // pseudo-raisonnement symbolique basique
        let steps = prompt
            .split('.')
            .filter(|s| !s.trim().is_empty())
            .take(self.max_steps)
            .map(|s| PlanStep { description: s.trim().to_string(), done: false })
            .collect::<Vec<_>>();

        Plan { goal: "Synthesize answer".into(), steps }
    }
}
```
```rust
impl Expert for RulePlanner {
    fn name(&self) -> &'static str { "rule_planner" }

    fn can_handle(&self, task: &str) -> bool {
        // Ex: tâches "plan:", "strategy:", "todo:" etc.
        task.starts_with("plan:") || task.contains("strategy")
    }

    async fn infer(&self, x: &InputData, _ctx: &Context) -> ExpertOut {
        let now = std::time::Instant::now();
        let prompt = x.as_text().unwrap_or("");
        let plan = self.plan_from_prompt(prompt);

        ExpertOut {
            value: Value::Plan(plan),
            aux: ExpertAux {
                latency_ms: now.elapsed().as_millis() as u64,
                cost_units: 0.1,          // symbolique pas cher
                confidence: 0.7,          // calibrage simple
            },
        }
    }
}
```

## Modèle interne (neuronal, etc.) et expert métier exposé

```rust
// base_models/neural/transformer.rs (modèle générique, non exposé au routeur)
pub struct Transformer {
    pub dim: usize,
    pub layers: usize,
    // poids/params réels dans ton implémentation
}

impl Transformer {
    pub fn forward_text(&self, prompt: &str) -> String {
        // Stub: dans le vrai code, passe par ton backend (CPU/GPU)
        format!("[gen:{}layers:{}] {}", self.dim, self.layers, prompt)
    }
}
```
```rust
// experts/nlp/french_tagger.rs (expert métier, exposé au routeur)
pub struct NlpFrenchTagger {
    pub model: Transformer,
    // autres params spécifiques
}

impl Expert for NlpFrenchTagger {
    fn name(&self) -> &'static str { "nlp_french_tagger" }

    fn can_handle(&self, task: &str) -> bool {
        // Ex: tâches "fr:tag", "fr:pos", etc.
        task.contains("fr:tag")
    }

    async fn infer(&self, x: &InputData, _ctx: &Context) -> ExpertOut {
        let now = std::time::Instant::now();
        let prompt = x.as_text().unwrap_or("");
        let out = self.model.forward_text(prompt);

        ExpertOut {
            value: Value::Text(out),
            aux: ExpertAux {
                latency_ms: now.elapsed().as_millis() as u64,
                cost_units: (self.model.layers as f32) * 0.5, // ex: coût ∝ profondeur
                confidence: 0.6,                         // à calibrer
            },
        }
    }
}
```

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
Loggue `utilisation_experts` dans `MixMeta` pour suivre le drift.

## 6. Télémetrie minimale (traçabilité)

- `trace_id` dans `Context`, propagé jusqu’à chaque `ExpertAux`.
- `MixMeta` : `entropy_gates`, `topk`, `lat_total_ms`, `drop_count` (experts non appelés faute de budget).

## 7. Shadow routing cloisonné

- Exécute les shadows après la voie primaire si budget restant > seuil.
- Feedback “offline” (jamais dans le chemin critique).

---


> Ces patterns sont obligatoires pour tout développement ou refactor du MoE. Ils assurent la solidité, la maintenabilité et la capacité d’évolution du système dès le départ.