use super::router_rule::RouterRule;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct RouterConfig {
    pub rules: Vec<RouterRule>,
    pub fallback: Vec<String>,
}
