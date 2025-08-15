use std::collections::HashMap;

#[derive(Default, Clone)]
pub struct Score {
    pub success: f32,
    pub speed: f32,
    pub diversity: f32,
    pub count: u32,
}

impl Score {
    pub fn total(&self, weights: &HashMap<&str, f32>) -> f32 {
        weights.get("success").unwrap_or(&1.0) * self.success
            + weights.get("speed").unwrap_or(&0.0) * self.speed
            + weights.get("diversity").unwrap_or(&0.0) * self.diversity
    }
}
