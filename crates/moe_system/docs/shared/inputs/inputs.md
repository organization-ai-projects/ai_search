### 2.1 Type d'entrée unique : `InputData`
Peut contenir du texte, des features, des vecteurs, etc.
```rust
//src/shared/inputs/input_data.rs
#[derive(Clone, Debug)]
pub enum InputData {
    Text(String),
    Bytes(Vec<u8>),
    Features(Vec<f32>),
    // Ajoute d'autres variantes selon tes besoins
}

impl InputData {
    pub fn as_text(&self) -> Option<&str> {
        match self { InputData::Text(s) => Some(s), _ => None }
    }
    // Ajoute d'autres helpers si besoin
}
```