use super::router_index_entry::RouterIndexEntry;
use std::fs;
use std::path::Path;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct RoutersIndex {
    pub entries: Vec<RouterIndexEntry>,
}

impl RoutersIndex {
    pub fn load_from_file<P: AsRef<Path>>(path: P) -> std::io::Result<Self> {
        let data = fs::read_to_string(path)?;
        Ok(serde_json::from_str(&data)?)
    }

    pub fn save_to_file<P: AsRef<Path>>(&self, path: P) -> std::io::Result<()> {
        let data = serde_json::to_string_pretty(self)?;
        fs::write(path, data)
    }
}
