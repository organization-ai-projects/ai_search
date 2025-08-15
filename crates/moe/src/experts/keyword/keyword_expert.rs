use crate::experts::expert_trait::Expert;
use std::error::Error;

pub struct KeywordExpert;

impl Expert for KeywordExpert {
    fn name(&self) -> &'static str {
        "KeywordExpert"
    }

    fn process(&self, input: &str) -> Result<String, Box<dyn Error + Send + Sync>> {
        let keywords = ["rust", "ai", "expert", "moe"];
        let found: Vec<&str> = keywords
            .iter()
            .cloned()
            .filter(|k| input.to_lowercase().contains(k))
            .collect();
        Ok(format!("Keywords found: {}", found.join(", ")))
    }
}
