use super::role_behavior_trait::RoleBehavior;

pub struct Synthesizer;

impl RoleBehavior for Synthesizer {
    fn name(&self) -> &'static str {
        "Synthesizer"
    }
    fn process(&self, input: &str) -> Box<dyn std::any::Any> {
        Box::new(format!("Synthétisé: {}", input))
    }
}
