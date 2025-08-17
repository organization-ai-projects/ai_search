use crate::shared::values::{Plan, PlanStep};

pub struct RulePlanner {
    pub max_steps: usize,
}

impl RulePlanner {
    fn plan_from_prompt(&self, prompt: &str) -> Plan {
        let steps = prompt
            .split('.')
            .filter(|s| !s.trim().is_empty())
            .take(self.max_steps)
            .map(|s| PlanStep {
                description: s.trim().to_string(),
                done: false,
            })
            .collect::<Vec<_>>();

        Plan {
            goal: "Synthesize answer".into(),
            steps,
        }
    }
}
