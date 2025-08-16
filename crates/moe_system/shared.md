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
```
```rust
/// Plan structuré pour la sortie symbolique
#[derive(Clone, Debug)]
pub struct Plan {
    pub goal: String,
    pub steps: Vec<PlanStep>,
}
```
```rust
#[derive(Clone, Debug)]
pub struct PlanStep {
    pub description: String,
    pub done: bool,
}
```
```rust
/// Trait d’adaptation vers Value
pub trait ToValue {
    fn to_value(self) -> Value;
}
```
```rust
impl ToValue for String {
    fn to_value(self) -> Value { Value::Text(self) }
}
```
```rust
impl ToValue for Plan {
    fn to_value(self) -> Value { Value::Plan(self) }
}
```
```rust
#[derive(Clone, Debug)]
pub struct AggregationMetadata {
    pub entropy: f32,
    pub topk: usize,
    pub lat_total_ms: u64,
    pub drop_count: usize,
    pub util_by_expert: Vec<(ExpertId, f32)>, // pour load-balance
}
```
```rust
#[derive(Clone, Debug)]
pub struct AggregationResult {
    /// Sortie unique après agrégation locale du routeur
    pub value: Value,
    /// Métadonnées d'agrégation / télémétrie
    pub aggregation_metadata: AggregationMetadata,
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
```

```rust
pub struct Encoded(pub Vec<f32>); // alias simple, change plus tard si besoin
```
```rust
pub trait Encoder: Send + Sync {
    fn encode(&self, x: &InputData) -> MoeResult<Encoded>;
}
```

### 2.2 Gestion d'erreur normalisée
```rust
pub type MoeResult<T> = Result<T, MoeError>;
```

```rust
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
```

```rust
#[derive(Clone, Debug)]
pub struct GateScores {
    // scores/softmax par expert_id
    pub logits: Vec<(ExpertId, f32)>,
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