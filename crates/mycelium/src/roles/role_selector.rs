use std::any;

use super::role_behavior_trait::RoleBehavior;

pub struct RoleSelector;

impl RoleBehavior for RoleSelector {
    fn name(&self) -> &'static str {
        "RoleSelector"
    }
    fn process(&self, input: &str) -> Box<dyn any::Any> {
        Box::new(format!("Rôle sélectionné: {}", input))
    }
}