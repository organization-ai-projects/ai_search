use crate::experts::expert_trait::Expert;
use std::error::Error;

pub struct DigitSumExpert;

impl Expert for DigitSumExpert {
    fn name(&self) -> &'static str {
        "DigitSumExpert"
    }

    fn process(&self, input: &str) -> Result<String, Box<dyn Error + Send + Sync>> {
        let sum: u32 = input.chars().filter_map(|c| c.to_digit(10)).sum();
        Ok(format!("Digit sum: {}", sum))
    }
}
