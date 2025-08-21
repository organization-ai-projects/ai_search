use std::collections::HashMap;

use crate::{
    category_registry::CategoryRegistry, char_meta::CharMeta, default_category_id, uppercase_flag,
};

/* ========= Encodage ========= */
#[derive(Debug, Clone)]
pub struct Encoded {
    pub bytes: Vec<u8>,       // données “brutes” UTF-8 (source de vérité)
    pub metas: Vec<CharMeta>, // 1 entrée par caractère Unicode
    pub registry: CategoryRegistry,
    /// association_id -> (type, membres)
    pub associations: HashMap<usize, (String, Vec<usize>)>,
}

impl Encoded {
    pub fn decode_utf8(&self) -> String {
        String::from_utf8(self.bytes.clone()).expect("UTF-8 valide")
    }

    /// change la catégorie d’un caractère (par index de meta)
    pub fn set_category(&mut self, meta_index: usize, new_cat: usize) {
        if let Some(m) = self.metas.get_mut(meta_index) {
            m.cat_id = new_cat;
        }
    }

    /// permet des overrides par caractère (ex: ‘!’ => “SpecialPunct”)
    pub fn apply_overrides_by_char(&mut self, overrides: &HashMap<char, usize>) {
        for m in &mut self.metas {
            if let Some(&new_id) = overrides.get(&m.ch) {
                m.cat_id = new_id;
            }
        }
    }
}

/* ========= Encodeur principal ========= */
pub fn encode(text: &str, mut registry: CategoryRegistry) -> Encoded {
    // on garde une copie brute UTF-8 (source de vérité immuable)
    let bytes = text.as_bytes().to_vec();

    // on itère par caractères (code points), en suivant les spans d’octets
    let mut metas = Vec::new();
    let mut idx = 0; // index en bytes
    let chars: Vec<char> = text.chars().collect();
    for &ch in chars.iter() {
        let start = idx;
        let mut buf = [0u8; 4];
        let s = ch.encode_utf8(&mut buf);
        idx += s.len();
        let end = idx;

        let cat_id = default_category_id(&mut registry, ch);
        let flags = uppercase_flag(ch);

        metas.push(CharMeta {
            ch,
            byte_span: start..end,
            cat_id,
            flags,
            association_id: None,
        });
    }

    Encoded {
        bytes,
        metas,
        registry,
        associations: HashMap::new(),
    }
}
