```rust
//src/experts/expert_ref.rs
#[derive(Clone)]
pub struct ExpertRef {
    pub id: ExpertId,
    pub handle: std::sync::Arc<dyn Expert>,
}
```

### 2.9 Trait unique pour tous les experts (asynchrone, standardisé)
```rust
//src/experts/expert_trait.rs
#[async_trait::async_trait]
pub trait Expert: Send + Sync {
    fn name(&self) -> &'static str;
    fn can_handle(&self, task: &str) -> bool; // hint symbolique
    async fn infer(&self, x: &InputData, ctx: &Context) -> MoeResult<ExpertOut>;
}
```
```rust
//src/experts/expert_registry_trait.rs
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

### 2.8 Sortie d'un expert
```rust
//src/experts/expert_out.rs
pub struct ExpertOut {
    pub value: Value,          // texte, plan, structure…
    pub aux: ExpertAux,        // latence, coût, confiance…
}
```

# Exemples d’implémentation d’experts (symbolique et neuronal)

## Expert symbolique (règles/planification)

```rust
// experts/planning/rule_planner.rs
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
//src/experts/rule_planner.rs
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

```rust
// experts/nlp/nlp_french_tagger.rs (expert métier, exposé au routeur)
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

```rust
//src/experts/expert_aux
#[derive(Clone, Debug)]
pub struct ExpertAux {
    pub latency_ms: u64,
    pub cost_units: f32,
    pub confidence: f32,
    pub trace_id: Option<String>,
}
```