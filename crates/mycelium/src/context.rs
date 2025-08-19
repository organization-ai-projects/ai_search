use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Context {
    pub input_initial: String,
    pub output_current: String,
    pub history: Vec<String>,
    pub feedback: Option<String>,
    pub results_quality_judge: Vec<String>,
}

impl Context {
    pub fn new(input: &str) -> Self {
        Self {
            input_initial: input.to_string(),
            output_current: input.to_string(),
            history: vec![],
            feedback: None,
            results_quality_judge: vec![],
        }
    }
    pub fn update_output(&mut self, output: String) {
        self.history.push(self.output_current.clone());
        self.output_current = output;
    }
    pub fn set_feedback(&mut self, fb: String) {
        self.feedback = Some(fb);
    }
}
