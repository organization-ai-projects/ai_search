## Arborescence cible (exemple type « tree »)

```
src/
    orchestrator/
        orchestrator_trait.rs         // Trait Orchestrator
    router/
        router_trait.rs               // Trait Router
        aggregator_trait.rs           // Trait Aggregator
        default_aggregator.rs         // Implémentation DefaultAggregator
        aggregated_out.rs             // Struct AggregatedOut
        gate_scores.rs                // Struct GateScores
        mix_meta.rs                   // Struct MixMeta
    experts/
        expert_trait.rs               // Trait Expert
        expert_registry_trait.rs      // Trait ExpertRegistry
        expert_ref.rs                 // Struct ExpertRef
        expert_id.rs                  // Struct ExpertId
        expert_aux.rs                 // Struct ExpertAux
        expert_out.rs                 // Struct ExpertOut
        planning/
            planner_rules.rs            // Expert symbolique RulePlanner
        nlp/
            french_tagger.rs            // Expert métier NLP NlpFrenchTagger
    base_models/
        mod.rs
        neural/
            mod.rs
            transformer/
                mod.rs
                transformer.rs        // Struct Transformer
                config.rs             // Struct TransformerConfig
                block.rs              // Struct TransformerBlock
            mamba/
                mod.rs
                mamba.rs              // Struct Mamba
                config.rs             // Struct MambaConfig
                block.rs              // Struct MambaBlock
        symbolic/
            mod.rs
            rules_engine/
                mod.rs
                rules_engine.rs       // Struct RulesEngine
                config.rs             // Struct RulesEngineConfig
    shared/
        mod.rs
        value/
            mod.rs
            value.rs              // Enum Value
            plan.rs               // Struct Plan
            plan_step.rs          // Struct PlanStep
            adapters_trait.rs     // Trait ToValue
            string_to_value.rs    // impl ToValue for String
            plan_to_value.rs      // impl ToValue for Plan
        context/
            mod.rs
            context.rs            // Struct Context
            cancel_token.rs       // Struct CancelToken
        encoding/
            mod.rs
            encoder_trait.rs      // Trait Encoder
            encoded.rs           // Struct Encoded
        error/
            mod.rs
            moe_error.rs          // Enum MoeError
            moe_result.rs         // Type MoeResult
        feedback/
            mod.rs
            router_feedback.rs    // Struct RouterFeedback
            orchestration_feedback.rs // Struct OrchestrationFeedback
        gating/
            mod.rs
            gating_policy.rs      // Struct GatingPolicy
            softmax.rs           // Fonctions softmax_stable, gate_with_policy
```

> Cette arborescence est exhaustive : chaque struct/enum/trait/adaptateur a son propre fichier, à sa place logique, sans types/ ni regroupement ambigu. À suivre strictement pour toute génération ou refactor.

> Cette arborescence est à valider/adapter selon tes besoins exacts avant toute génération de squelette ou découpage effectif.


---