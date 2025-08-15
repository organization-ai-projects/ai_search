use crate::experts::expert_trait::Expert;
use std::error::Error;

pub struct PalindromeExpert;

impl Expert for PalindromeExpert {
    fn name(&self) -> &'static str {
        "PalindromeExpert"
    }

    fn process(&self, input: &str) -> Result<String, Box<dyn Error + Send + Sync>> {
        let cleaned: String = input
            .chars()
            .filter(|c| c.is_alphanumeric())
            .collect::<String>()
            .to_lowercase();
        let is_palindrome = cleaned == cleaned.chars().rev().collect::<String>();
        Ok(format!("Palindrome: {}", is_palindrome))
    }
}
