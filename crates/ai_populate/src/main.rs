mod category_registry;
mod char_meta;
mod encoded;
mod image_module;
mod orchestrator;
mod print;

use std::any;
use std::collections::HashMap;
use std::fs;
use strsim::levenshtein;
use unicode_categories::UnicodeCategories;
use unicode_normalization::UnicodeNormalization;

use crate::category_registry::CategoryRegistry;
use crate::encoded::{encode, Encoded};
use crate::image_module::ImageModule;
use crate::orchestrator::{DataPacket, Module, Orchestrator};
use crate::print::{print_metas, print_metas_overrides};
// === Exemple de module textuel pour orchestrateur ===
struct TextNlpModule;

impl Module for TextNlpModule {
    fn name(&self) -> &str {
        "TextNlpModule"
    }
    fn modality(&self) -> &str {
        "text"
    }
    fn process(&self, input: &DataPacket) -> DataPacket {
        // On suppose que le payload est une String
        if let Some(text) = (input.payload.as_ref() as &dyn any::Any).downcast_ref::<String>() {
            let mut enc = encode(text, CategoryRegistry::default());
            // Override direct sur '!'
            override_char_category(&mut enc, '!', "SpecialPunct");
            // Overrides par table
            override_chars_table(&mut enc, &['Ç', 'À', 'É'], "LetterAccented");
            // Associations automatiques (casse et accents)
            let (norm_to_group, group_id_to_type) = build_auto_associations(&enc);
            let mut enc = enc;
            apply_associations(&mut enc, &norm_to_group, &group_id_to_type);
            // On renvoie le résultat dans le payload (encodé dans Box)
            let mut meta = input.meta.clone();
            meta.insert("reconstructed".into(), enc.decode_utf8());
            DataPacket {
                modality: "text".into(),
                payload: Box::new(enc),
                meta,
            }
        } else {
            input.clone()
        }
    }
}

/* ========= Métadonnées par caractère ========= */
const FLAG_UPPERCASE: u8 = 0b0000_0001;

/* ========= Heuristique simple de catégorisation par défaut ========= */
// Note: on reste volontairement simple (sans lib Unicode avancée)
/// Catégorisation Unicode avancée
fn default_category_id(reg: &mut CategoryRegistry, ch: char) -> usize {
    if ch.is_letter() {
        reg.id_or_insert("Letter")
    } else if ch.is_number() {
        reg.id_or_insert("Digit")
    } else if ch.is_whitespace() {
        reg.id_or_insert("Whitespace")
    } else if ch.is_punctuation() {
        reg.id_or_insert("Punct")
    } else if ch.is_symbol() {
        reg.id_or_insert("Symbol")
    } else {
        reg.id_or_insert("Other")
    }
}

fn uppercase_flag(ch: char) -> u8 {
    if ch.is_uppercase() {
        FLAG_UPPERCASE
    } else {
        0
    }
}

/* ========= Démonstration ========= */
/// Applique un override direct sur un caractère donné (ex: '!' -> "SpecialPunct")
fn override_char_category(enc: &mut Encoded, ch: char, cat_name: &str) {
    let cat_id = enc.registry.id_or_insert(cat_name);
    if let Some((i, _)) = enc.metas.iter().enumerate().find(|(_, m)| m.ch == ch) {
        enc.set_category(i, cat_id);
    }
}

/// Applique des overrides par table (ex: ['Ç', 'À', 'É'] -> "LetterAccented")
fn override_chars_table(enc: &mut Encoded, chars: &[char], cat_name: &str) -> HashMap<char, usize> {
    let cat_id = enc.registry.id_or_insert(cat_name);
    let mut ov = HashMap::<char, usize>::new();
    for &c in chars {
        ov.insert(c, cat_id);
    }
    enc.apply_overrides_by_char(&ov);
    ov
}

/// Construit automatiquement des associations intelligentes (casse et accents)
fn build_auto_associations(enc: &Encoded) -> (HashMap<String, usize>, HashMap<usize, String>) {
    let mut norm_to_group: HashMap<String, usize> = HashMap::new();
    let mut group_id_to_type = HashMap::new();
    let mut next_group = 0;

    // Collecte de toutes les formes normalisées uniques (minuscule + sans accents)
    let mut all_norms: Vec<String> = Vec::new();
    for m in &enc.metas {
        let norm =
            m.ch.to_lowercase()
                .nfkd()
                .filter(|c| !c.is_mark_nonspacing())
                .collect::<String>();
        if !all_norms.contains(&norm) {
            all_norms.push(norm);
        }
    }

    // Clustering par similarité de forme (Levenshtein <= 1)
    let mut norm_to_groupid: HashMap<String, usize> = HashMap::new();
    for norm in &all_norms {
        let mut found = false;
        for (g_norm, &gid) in &norm_to_groupid {
            if levenshtein(norm, g_norm) <= 1 {
                norm_to_groupid.insert(norm.clone(), gid);
                found = true;
                break;
            }
        }
        if !found {
            norm_to_groupid.insert(norm.clone(), next_group);
            group_id_to_type.insert(next_group, "auto+sim".to_string());
            next_group += 1;
        }
    }

    // Remplissage final pour chaque caractère
    for m in &enc.metas {
        let norm =
            m.ch.to_lowercase()
                .nfkd()
                .filter(|c| !c.is_mark_nonspacing())
                .collect::<String>();
        let gid = *norm_to_groupid.get(&norm).unwrap();
        norm_to_group.insert(norm, gid);
    }
    (norm_to_group, group_id_to_type)
}

/// Applique les associations de groupes sur les caractères et construit la map des associations
fn apply_associations(
    enc: &mut Encoded,
    norm_to_group: &HashMap<String, usize>,
    group_id_to_type: &HashMap<usize, String>,
) {
    // Associe chaque caractère à son groupe normalisé
    for m in enc.metas.iter_mut() {
        let norm =
            m.ch.to_lowercase()
                .nfkd()
                .filter(|c| !c.is_mark_nonspacing())
                .collect::<String>();
        if let Some(&gid) = norm_to_group.get(&norm) {
            m.association_id = Some(gid);
        }
    }
    // Construit la map association_id -> (type, membres)
    let mut group_members: HashMap<usize, Vec<usize>> = HashMap::new();
    for (i, m) in enc.metas.iter().enumerate() {
        if let Some(gid) = m.association_id {
            group_members.entry(gid).or_default().push(i);
        }
    }
    for (gid, members) in group_members {
        let typ = group_id_to_type
            .get(&gid)
            .cloned()
            .unwrap_or("?".to_string());
        enc.associations.insert(gid, (typ, members));
    }
}

fn main() {
    let text =
        "aAbBcCdDeEfFgGhHiIjJkKlLmMnNoOpPqQrRsStTuUvVwWxXyYzZ Bonjour 123! Ça va ?".to_string();

    // --- Lecture de vraies images depuis le disque ---
    let png_bytes = fs::read("test_image.png").expect("Fichier test_image.png introuvable");
    let jpeg_bytes = fs::read("test_image.jpg").expect("Fichier test_image.jpg introuvable");

    let mut orchestrator = Orchestrator::new();
    orchestrator.add_module(Box::new(TextNlpModule));
    orchestrator.add_module(Box::new(ImageModule));

    // --- Pipeline texte ---
    let packet_text = DataPacket {
        modality: "text".into(),
        payload: Box::new(text.clone()),
        meta: Default::default(),
    };
    let result_text = orchestrator.run(packet_text);

    // Récupération du résultat Encoded
    if let Some(enc) = (result_text.payload.as_ref() as &dyn any::Any).downcast_ref::<Encoded>() {
        println!("Texte original    : {:?}", enc.decode_utf8());
        println!("Bytes (len={})     : {:?}", enc.bytes.len(), enc.bytes);
        print_metas(enc);
        // Affichage des overrides et associations
        let reconstructed = enc.decode_utf8();
        println!("\nReconstruction identique ? {}", reconstructed == text);
        // Pour print_metas_overrides, on doit régénérer la table ov (ici simplifié)
        let ov = override_chars_table(&mut enc.clone(), &['Ç', 'À', 'É'], "LetterAccented");
        print_metas_overrides(enc, &ov);
    } else {
        println!("Erreur : le module n'a pas renvoyé un Encoded valide.");
    }

    // --- Pipeline image PNG ---
    let packet_png = DataPacket {
        modality: "image".into(),
        payload: Box::new(png_bytes.clone()),
        meta: Default::default(),
    };
    let result_png = orchestrator.run(packet_png);
    if let Some(pixels) = (result_png.payload.as_ref() as &dyn any::Any).downcast_ref::<Vec<u8>>() {
        println!(
            "\n[Image PNG] Format: {} | {}x{} | Color: {} | Pixels: {} octets",
            result_png
                .meta
                .get("image_format")
                .unwrap_or(&"?".to_string()),
            result_png.meta.get("width").unwrap_or(&"?".to_string()),
            result_png.meta.get("height").unwrap_or(&"?".to_string()),
            result_png
                .meta
                .get("color_type")
                .unwrap_or(&"?".to_string()),
            pixels.len()
        );
    }

    // --- Pipeline image JPEG ---
    let packet_jpeg = DataPacket {
        modality: "image".into(),
        payload: Box::new(jpeg_bytes.clone()),
        meta: Default::default(),
    };
    let result_jpeg = orchestrator.run(packet_jpeg);
    if let Some(pixels) = (result_jpeg.payload.as_ref() as &dyn any::Any).downcast_ref::<Vec<u8>>()
    {
        println!(
            "\n[Image JPEG] Format: {} | {}x{} | Color: {} | Pixels: {} octets",
            result_jpeg
                .meta
                .get("image_format")
                .unwrap_or(&"?".to_string()),
            result_jpeg.meta.get("width").unwrap_or(&"?".to_string()),
            result_jpeg.meta.get("height").unwrap_or(&"?".to_string()),
            result_jpeg
                .meta
                .get("color_type")
                .unwrap_or(&"?".to_string()),
            pixels.len()
        );
    }
}
