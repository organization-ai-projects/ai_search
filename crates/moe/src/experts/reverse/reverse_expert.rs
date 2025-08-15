use crate::experts::expert_trait::Expert;
use std::error::Error;

pub struct ReverseExpert;

impl Expert for ReverseExpert {
    fn process(&self, input: &str) -> Result<String, Box<dyn Error + Send + Sync>> {
        Ok(input.chars().rev().collect())
    }
    fn name(&self) -> &'static str {
        "ReverseExpert"
    }
}
