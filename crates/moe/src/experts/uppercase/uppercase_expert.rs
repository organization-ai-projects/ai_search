use crate::experts::expert_trait::Expert;
use std::error::Error;

pub struct UppercaseExpert;

impl Expert for UppercaseExpert {
    fn process(&self, input: &str) -> Result<String, Box<dyn Error + Send + Sync>> {
        Ok(input.to_uppercase())
    }
    fn name(&self) -> &'static str {
        "UppercaseExpert"
    }
}
