use crate::experts::expert_trait::Expert;
use std::error::Error;
use std::thread;
use std::time::Duration;

pub struct SlowExpert;

impl Expert for SlowExpert {
    fn process(&self, input: &str) -> Result<String, Box<dyn Error + Send + Sync>> {
        thread::sleep(Duration::from_secs(2));
        Ok(format!("[slow] {}", input))
    }
    fn name(&self) -> &'static str {
        "SlowExpert"
    }
    fn dependencies() -> Vec<&'static str> {
        vec!["uppercase"]
    }
}
