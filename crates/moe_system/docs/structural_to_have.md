## Arborescence cible (exemple type « tree »)

```
src/
    lib.rs                  // Sert uniquement à pub mod ... (pas de logique de bibliothèque)
    main.rs                 // Point d'entrée du binaire
    orchestrator/
        mod.rs
        orchestrator_trait.rs         // Trait Orchestrator
        orchestration_feedback.rs     // Struct OrchestrationFeedback
    default_synthesizer.rs         // Implémentation DefaultSynthesizer (orchestrator-local impl)
    router/
        mod.rs
        router_trait.rs               // Trait Router
        router_ref.rs                 // Struct RouterRef (id + handle)
        router_input.rs               // RouterInput (Encoded | Raw)
        router_feedback.rs            // Struct RouterFeedback
    experts/
        expert_trait.rs               // Trait Expert
        expert_registry_trait.rs      // Trait ExpertRegistry
        expert_ref.rs                 // Struct ExpertRef
        expert_id.rs                  // Struct ExpertId
        expert_aux.rs                 // Struct ExpertAux
        expert_out.rs                 // Struct ExpertOut
        planning/
            rule_planner.rs            // Expert symbolique RulePlanner
        nlp/
            nlp_french_tagger.rs            // Expert métier NLP NlpFrenchTagger
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
        inputs/
            mod.rs                 // module inputs
            input_data.rs           // InputData
        values/
            mod.rs
            value.rs              // Enum Value
            plan.rs               // Struct Plan
            string.rs             // Struct String
            plan_step.rs          // Struct PlanStep
            to_value_trait.rs     // Trait ToValue
        outputs/
            mod.rs                 // module outputs
            synthesis_result.rs  // Struct SynthesisResult
            synthesis_metadata.rs // Struct SynthesisMetadata
        synthesizers/
            mod.rs                 // module synthesizers
            synthesizer_trait.rs    // Trait Synthesizer (présent ici par choix d'anticipation : permet d'éventuelles stratégies de synthèse portées par d'autres modules qu'un orchestrateur, ou par des experts spécialisés, pour une architecture future-proof)
        contexts/
            mod.rs
            context.rs            // Struct Context
            cancel_token.rs       // Struct CancelToken
        encodings/
            mod.rs
            encoder_trait.rs      // Trait Encoder
            encoded.rs            // Struct Encoded
        errors/
            mod.rs
            moe_error.rs          // Enum MoeError
            moe_result.rs         // Type MoeResult
        gatings/
            mod.rs
            gate_scores.rs        // Struct GateScores
            gating_policy.rs      // Struct GatingPolicy
            softmax.rs            // Fonctions softmax_stable, gate_with_policy
```

> Cette arborescence est exhaustive : chaque struct/enum/trait/adaptateur a son propre fichier, à sa place logique, sans types/ ni regroupement ambigu. À suivre strictement pour toute génération ou refactor.

> Cette arborescence est à valider/adapter selon tes besoins exacts avant toute génération de squelette ou découpage effectif.


---