use crate::shared::Plan;

/// Value = format commun que *tous* les experts doivent renvoyer.
/// Garde-le parcimonieux ; ajoute des variantes seulement si nécessaires.
#[derive(Clone, Debug)]
pub enum Value {
    /// Réponses textuelles / rationales
    Text { schema: u16, data: String },
    /// Plans symboliques structurés
    Plan { schema: u16, data: Plan },
    /// Résultats structurés ad hoc
    Json {
        schema: u16,
        data: serde_json::Value,
    },
    /// Représentations vectorielles
    Embedding { schema: u16, data: Vec<f32> },
    /// Binaire (images compressées, audio, etc.)
    Bytes { schema: u16, data: Vec<u8> },
    /// Pour signifier "pas de résultat utile"
    None,
}

impl Value {
    /// Crée une nouvelle valeur textuelle versionnée (variant Text).
    /// Usage : Value::text(1, "foo")
    /// - schema : version du format (ex : 1)
    /// - s : contenu textuel
    pub fn text(schema: u16, s: impl Into<String>) -> Self {
        Value::Text {
            schema,
            data: s.into(),
        }
    }

    /// Tente d'extraire une vue (&str) et la version (schema) si self est Text.
    /// Retourne Some((schema, &str)) ou None si ce n'est pas un texte.
    /// Usage : if let Some((schema, txt)) = v.as_text() { ... }
    pub fn as_text(&self) -> Option<(u16, &str)> {
        if let Value::Text { schema, data } = self {
            Some((*schema, data.as_str()))
        } else {
            None
        }
    }

    /// Indique si la valeur est None (pas de résultat utile).
    pub fn is_none(&self) -> bool {
        matches!(self, Value::None)
    }
}
