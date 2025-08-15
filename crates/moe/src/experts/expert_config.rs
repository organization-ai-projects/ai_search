use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct ExpertConfig {
    pub name: String,
    pub kind: String,
}
