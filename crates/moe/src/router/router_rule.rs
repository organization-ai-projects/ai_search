use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct RouterRule {
    pub keyword: String,
    pub experts: Vec<String>,
}
