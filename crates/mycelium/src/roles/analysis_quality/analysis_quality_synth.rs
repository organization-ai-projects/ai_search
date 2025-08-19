use super::super::role_behavior_trait::RoleBehavior;
use crate::context::Context;
use serde_json;

pub struct AnalysisQualityReviewStruct;

impl RoleBehavior for AnalysisQualityReviewStruct {
    fn name(&self) -> &'static str {
        "AnalysisQualityReviewStruct"
    }

    fn process(&self, input: &str) -> Box<dyn std::any::Any> {
        fn synthese(ctx: &Context) -> String {
            if ctx.results_quality_judge.is_empty() {
                return "Aucun contrôle qualité effectué.".to_string();
            }
            if ctx.results_quality_judge.iter().all(|r| r == "Vide") {
                return "Tous les contrôles qualité sont vides.".to_string();
            }
            if ctx.results_quality_judge.iter().all(|r| r == "Valide") {
                return "Tous les contrôles qualité sont valides.".to_string();
            }
            let mut details = String::from("Synthèse des contrôles qualité :\n");
            for (i, res) in ctx.results_quality_judge.iter().enumerate() {
                details.push_str(&format!("- Contrôle {} : {}\n", i + 1, res));
            }
            details
        }
        match serde_json::from_str::<Context>(input) {
            Ok(ctx) => Box::new(synthese(&ctx)),
            Err(_) => Box::new("[Erreur: input non convertible en Context]".to_string()),
        }
    }
}
