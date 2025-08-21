use std::collections::HashMap;

use crate::encoded::Encoded;

pub fn print_metas(enc: &Encoded) {
    println!("\n--- METAS ---");
    for (i, m) in enc.metas.iter().enumerate() {
        println!(
            "#{:02} '{}'  span={:?}  cat={}  flags={:08b}",
            i,
            m.ch,
            m.byte_span,
            enc.registry.name(m.cat_id),
            m.flags
        );
    }
}

pub fn print_metas_overrides(enc: &Encoded, ov: &HashMap<char, usize>) {
    println!("\n--- ASSOCIATIONS SYNTHÉTIQUES (groupes & overrides) ---");
    // Afficher chaque groupe d'association une seule fois
    let mut printed_groups = std::collections::HashSet::new();
    for (&gid, (typ, members)) in &enc.associations {
        if printed_groups.contains(&gid) {
            continue;
        }
        printed_groups.insert(gid);
        // Liste unique des caractères du groupe
        let mut chars: Vec<char> = members.iter().map(|&j| enc.metas[j].ch).collect();
        chars.sort();
        chars.dedup();
        // Catégories des membres (synthétique)
        let mut cats: Vec<&str> = members
            .iter()
            .map(|&j| enc.registry.name(enc.metas[j].cat_id))
            .collect();
        cats.sort();
        cats.dedup();
        print!("Association id={} type={} | membres: ", gid, typ);
        for c in &chars {
            print!("'{}' ", c);
        }
        print!("| catégories: ");
        for cat in &cats {
            print!("{} ", cat);
        }
        println!();
    }
    // Afficher les overrides directs (hors associations)
    if !ov.is_empty() {
        println!("\nOverrides directs:");
        for (&ch, &cat_id) in ov {
            println!("'{}' -> {}", ch, enc.registry.name(cat_id));
        }
    }
}
