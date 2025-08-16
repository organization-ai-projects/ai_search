## Structure du dossier `router/`


Respecter la convention stricte de structuration des fichiers et dossiers : chaque struct, enum ou trait a son propre fichier, à sa place logique, sans regroupement ambigu.


### Arborescence type pour `src/router/`

```
src/
	router/
		mod.rs
		router_trait.rs               // Trait Router
		router_ref.rs                 // Struct RouterRef (id + handle)
		router_input.rs               // RouterInput (Encoded | Raw)
		router_feedback.rs            // Struct RouterFeedback
```

> **Note :** L'agrégation n'est plus du ressort du Router. Le Router ne fait que router, sélectionner les experts, collecter leurs sorties et transmettre la liste brute à l'Orchestrator.


## Implémentations et traits du router (nouvelle version)

```rust
//src/router/router_input.rs
#[derive(Clone, Debug)]
pub enum RouterInput {
	Encoded(Encoded),
	Raw(InputData),
	// Ajoute d'autres variantes si besoin (ex: features, tokens…)
}
```

```rust
//src/router/router_ref.rs
pub struct RouterRef {
	pub id: String,
	pub handle: std::sync::Arc<dyn Router>,
}
```

```rust
//src/router/router_feedback.rs
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
#[async_trait::async_trait]
pub trait Router: Send + Sync {
	/// Routing/gating sur une entrée générique (Encoded, Raw, etc.)
	fn gate(&self, input: &RouterInput, ctx: &Context) -> MoeResult<GateScores>;
	/// Sélectionne les k meilleurs experts, borne et non-vide, sinon erreur explicite.
	fn pick_topk(&self, scores: &GateScores, k: usize) -> MoeResult<Vec<ExpertRef>> {
		let n = scores.logits.len();
		let k = k.min(n).max(1);
		if n == 0 {
			return Err(MoeError::NoExpertSelected);
		}
		// ... à implémenter dans chaque Router concret ...
		Err(MoeError::AggregationFailed("pick_topk: impl manquante".into()))
	}
	/// Appel des experts en parallèle avec timeout/budget (pattern deadline/cancel obligatoire)
	/// Toute implémentation doit garantir le parallélisme et le respect du budget/timeout pour chaque expert.
	async fn call_experts(&self, input: &RouterInput, picks: &[ExpertRef], ctx: &Context) -> MoeResult<Vec<(ExpertRef, ExpertOut)>>;
	fn train_signal(&mut self, fb: RouterFeedback);
}
```

> **Note :** Le trait Router ne propose plus de méthode d'agrégation ni d'Aggregator associé. Toute agrégation est désormais du ressort de l'Orchestrator.

**Rappel de la convention :**
- Chaque trait dans un fichier `xxx_trait.rs` (ex : `router_trait.rs`)
- Chaque struct/enum dans un fichier dédié (ex : `router_ref.rs`, `router_input.rs`)
- Pas de types/groupements ambigus : un type = un fichier = un rôle clair
- Cette organisation permet une navigation claire, une évolutivité maximale et évite les conflits ou la confusion lors de l’ajout de nouvelles fonctionnalités.

**Tout nouveau code ou refactor doit suivre cette convention.**

---

## Rôle du Router (nouvelle définition)

- Le Router sélectionne les experts pertinents, collecte leurs sorties (Vec<(ExpertRef, ExpertOut)>), et transmet cette liste brute à l'Orchestrator.
- Il ne fait aucune agrégation ni pondération des résultats.
- Il peut être spécialisé (par domaine, par type d'entrée, etc.), mais ne décide jamais de la sortie finale.

## Rôle de l'Orchestrator (rappel)

- L'Orchestrator reçoit les sorties de un ou plusieurs routers, applique la logique d'agrégation (pondération, fusion, sélection, etc.) via un Aggregator global ou local, et synthétise la réponse finale.
- Toute logique d'agrégation, de sélection finale ou de composition multi-router est centralisée ici.

---

> **Ce découpage garantit la flexibilité pour gérer des architectures multi-router, A/B testing, shadow routing, etc.**
