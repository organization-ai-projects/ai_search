use crate::experts::expert_trait::Expert;
use std::error::Error;

pub struct CaseDetectorExpert;

impl Expert for CaseDetectorExpert {
    fn name(&self) -> &'static str {
        "CaseDetectorExpert"
    }

    fn process(&self, input: &str) -> Result<String, Box<dyn Error + Send + Sync>> {
        if input
            .chars()
            .all(|c| c.is_uppercase() || !c.is_alphabetic())
        {
            Ok("All uppercase".to_string())
        } else if input
            .chars()
            .all(|c| c.is_lowercase() || !c.is_alphabetic())
        {
            Ok("All lowercase".to_string())
        } else {
            Ok("Mixed case".to_string())
        }
    }
}
