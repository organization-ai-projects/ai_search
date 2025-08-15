use crate::experts::expert_trait::Expert;
use std::error::Error;

pub struct AnagramExpert;

impl Expert for AnagramExpert {
    fn name(&self) -> &'static str {
        "AnagramExpert"
    }

    fn process(&self, input: &str) -> Result<String, Box<dyn Error + Send + Sync>> {
        let parts: Vec<&str> = input.split(',').collect();
        if parts.len() != 2 {
            return Err("Input must be two words separated by a comma".into());
        }
        let normalize = |s: &str| {
            let mut chars: Vec<char> = s
                .chars()
                .filter(|c| c.is_alphanumeric())
                .map(|c| c.to_lowercase().next().unwrap())
                .collect();
            chars.sort_unstable();
            chars
        };
        let is_anagram = normalize(parts[0]) == normalize(parts[1]);
        Ok(format!("Anagram: {}", is_anagram))
    }
}
