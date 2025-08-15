use crate::experts::expert_config::ExpertConfig;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub experts: Vec<ExpertConfig>,
}
