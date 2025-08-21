/* ========= Catégories symboliques (ids usize modifiables) ========= */
// Tu peux renommer/ajouter/supprimer des catégories quand tu veux.
#[derive(Debug, Clone)]
pub struct CategoryRegistry {
    // id  -> nom lisible
    names: Vec<String>,
}
impl CategoryRegistry {
    pub fn default() -> Self {
        // 0..=4 par défaut, mais tu peux en ajouter/retirer
        let names = vec![
            "Letter".into(),     // 0
            "Digit".into(),      // 1
            "Whitespace".into(), // 2
            "Punct".into(),      // 3
            "Other".into(),      // 4
        ];
        Self { names }
    }
    pub fn id_or_insert(&mut self, name: &str) -> usize {
        if let Some(i) = self.names.iter().position(|n| n == name) {
            i
        } else {
            self.names.push(name.into());
            self.names.len() - 1
        }
    }
    pub fn name(&self, id: usize) -> &str {
        self.names.get(id).map(|s| s.as_str()).unwrap_or("?")
    }
}