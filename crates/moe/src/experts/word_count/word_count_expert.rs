use crate::experts::expert_trait::Expert;
use std::error::Error;

pub struct WordCountExpert;

impl Expert for WordCountExpert {
    fn name(&self) -> &'static str {
        "WordCountExpert"
    }

    fn process(&self, input: &str) -> Result<String, Box<dyn Error + Send + Sync>> {
        let count = input.split_whitespace().count();
        Ok(format!("Word count: {}", count))
    }
}
