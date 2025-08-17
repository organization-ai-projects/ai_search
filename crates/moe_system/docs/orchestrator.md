## Structure du dossier `orchestrator/`

Respecter la convention stricte de structuration des fichiers et dossiers : chaque struct, enum ou trait a son propre fichier, à sa place logique, sans regroupement ambigu.

### Arborescence type pour `src/orchestrator/`

```
src/
	orchestrator/
		mod.rs
		orchestrator_trait.rs         // Trait Orchestrator
		orchestration_feedback.rs     // Struct OrchestrationFeedback
	default_synthesizer.rs         // Implémentation DefaultSynthesizer (orchestrator-local impl)
```

## Implémentations et traits de l'orchestrateur (extraits de base.md)

```rust
//src/orchestrator/orchestrator_trait.rs
pub trait Orchestrator: Send + Sync {
	fn choose_router(&self, x: &InputData, ctx: &Context) -> MoeResult<RouterRef>;

	fn synthesize(
		&self,
		primary: (&RouterRef, SynthesizedOut),
		shadow: Option<Vec<(&RouterRef, SynthesizedOut)>>
	) -> MoeResult<Value>;

	fn feedback(&mut self, fb: OrchestrationFeedback);
}
```

```rust
//src/orchestrator/default_synthesizer.rs
/// Synthétiseur par défaut utilisé par l'Orchestrator pour synthétiser les sorties des experts
pub struct DefaultSynthesizer;

impl Synthesizer for DefaultSynthesizer {
	fn id(&self) -> &'static str { "default" }
	fn synthesize(
		&self,
		calls: &[(ExpertRef, ExpertOut)],
		scores: &GateScores
	) -> MoeResult<SynthesizedOut> {
		if calls.is_empty() {
			return Err(MoeError::NoExpertSelected);
		}
		// Exemple : sélectionne la sortie avec le meilleur score
		let (best_id, _) = scores.logits.iter().max_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal)).ok_or(MoeError::SynthesisFailed("no scores".into()))?;
		let out = calls.iter().find(|(r,_)| r.id == *best_id).map(|(_,o)| o).ok_or(MoeError::SynthesisFailed("no matching output".into()))?;
		Ok(SynthesizedOut {
			value: out.value.clone(),
			synthesis_metadata: SynthesisMetadata {
				entropy: 0.0, // à calculer
				topk: calls.len(),
				lat_total_ms: calls.iter().map(|(_,o)| o.aux.latency_ms).sum(),
				drop_count: 0,
				util_by_expert: scores.logits.clone(),
			},
		})
	}
}
```

```rust
//src/orchestrator/orchestration_feedback.rs
pub struct OrchestrationFeedback {
	pub trace_id: String,
	pub primary_router: String,
	pub shadow_better: Option<String>, // id d’un router shadow gagnant
	pub notes: Option<String>,         // libre (erreurs, drift, etc.)
}
```

**Rappel de la convention :**
- Chaque trait dans un fichier `xxx_trait.rs` (ex : `orchestrator_trait.rs`)
- Chaque struct/enum dans un fichier dédié (ex : `orchestration_feedback.rs`)
- Les synthétiseurs utilisés par l'Orchestrator (ex : DefaultSynthesizer) sont définis dans ce dossier.
- Pas de types/groupements ambigus : un type = un fichier = un rôle clair
- Cette organisation permet une navigation claire, une évolutivité maximale et évite les conflits ou la confusion lors de l’ajout de nouvelles fonctionnalités.

**Tout nouveau code ou refactor doit suivre cette convention.**

---

## Exemple d'utilisation du synthétiseur par défaut dans l'Orchestrator

```rust
let synthesizer = DefaultSynthesizer;
let synth_result = synthesizer.synthesize(&calls, &scores)?;
// ... synthèse finale, feedback, etc.
```
