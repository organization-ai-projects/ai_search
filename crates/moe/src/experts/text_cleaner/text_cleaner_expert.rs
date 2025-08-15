use crate::experts::expert_trait::Expert;
use std::error::Error;

pub struct TextCleanerExpert;

impl Expert for TextCleanerExpert {
    fn name(&self) -> &'static str {
        "TextCleanerExpert"
    }

    fn process(&self, input: &str) -> Result<String, Box<dyn Error + Send + Sync>> {
        let cleaned: String = input
            .chars()
            .filter(|c| c.is_alphanumeric() || c.is_whitespace())
            .collect();
        Ok(cleaned)
    }
}
