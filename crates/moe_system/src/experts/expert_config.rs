use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct ExpertConfig {
    pub id: u64,
    pub tag: String,
    pub r#type: String,
    pub path: String,
}
