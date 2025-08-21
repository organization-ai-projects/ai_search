use std::ops::Range;

#[derive(Debug, Clone)]
pub struct CharMeta {
    pub ch: char,                      // le caractère (Unicode)
    pub byte_span: Range<usize>,       // où il se trouve dans le buffer UTF-8
    pub cat_id: usize,                 // catégorie symbolique (modifiable)
    pub flags: u8,                     // ex: uppercase, etc.
    pub association_id: Option<usize>, // groupe d'association (None = aucun)
}