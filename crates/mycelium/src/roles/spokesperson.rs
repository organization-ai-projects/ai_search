use super::role_behavior_trait::RoleBehavior;

pub struct Spokesperson;

impl RoleBehavior for Spokesperson {
    fn name(&self) -> &'static str {
        "Spokesperson"
    }
    fn process(&self, input: &str) -> Box<dyn std::any::Any> {
        Box::new(format!("Réponse portée: {}", input))
    }
}
