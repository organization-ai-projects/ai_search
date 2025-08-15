use crate::experts::anagram::anagram_expert::AnagramExpert;
use crate::experts::case_detector::case_detector_expert::CaseDetectorExpert;
use crate::experts::digit_sum::digit_sum_expert::DigitSumExpert;
use crate::experts::expert_trait::Expert;
use crate::experts::keyword::keyword_expert::KeywordExpert;
use crate::experts::palindrome::palindrome_expert::PalindromeExpert;
use crate::experts::reverse::reverse_expert::ReverseExpert;
use crate::experts::slow::slow_expert::SlowExpert;
use crate::experts::text_cleaner::text_cleaner_expert::TextCleanerExpert;
use crate::experts::uppercase::uppercase_expert::UppercaseExpert;
use crate::experts::word_count::word_count_expert::WordCountExpert;
use std::sync::Arc;

use std::collections::HashMap;

/// Retourne les factories pour une liste de noms d'experts (aucune closure créée si non sollicitée)
pub fn get_factories(
    names: &[&str],
) -> HashMap<String, Arc<dyn Fn() -> Box<dyn Expert + Sync> + Send + Sync>> {
    let mut map = HashMap::new();
    for &name in names {
        let factory: Option<Arc<dyn Fn() -> Box<dyn Expert + Sync> + Send + Sync>> = match name {
            "uppercase" => Some(Arc::new(|| {
                Box::new(UppercaseExpert) as Box<dyn Expert + Sync>
            })),
            "reverse" => Some(Arc::new(|| {
                Box::new(ReverseExpert) as Box<dyn Expert + Sync>
            })),
            "slow" => Some(Arc::new(|| Box::new(SlowExpert) as Box<dyn Expert + Sync>)),
            "word_count" => Some(Arc::new(|| {
                Box::new(WordCountExpert) as Box<dyn Expert + Sync>
            })),
            "palindrome" => Some(Arc::new(|| {
                Box::new(PalindromeExpert) as Box<dyn Expert + Sync>
            })),
            "anagram" => Some(Arc::new(|| {
                Box::new(AnagramExpert) as Box<dyn Expert + Sync>
            })),
            "case_detector" => Some(Arc::new(|| {
                Box::new(CaseDetectorExpert) as Box<dyn Expert + Sync>
            })),
            "digit_sum" => Some(Arc::new(|| {
                Box::new(DigitSumExpert) as Box<dyn Expert + Sync>
            })),
            "keyword" => Some(Arc::new(|| {
                Box::new(KeywordExpert) as Box<dyn Expert + Sync>
            })),
            "text_cleaner" => Some(Arc::new(|| {
                Box::new(TextCleanerExpert) as Box<dyn Expert + Sync>
            })),
            _ => None,
        };
        if let Some(f) = factory {
            map.insert(name.to_string(), f);
        }
    }
    map
}
/// Retourne la liste des dépendances pour un expert donné (par nom)
pub fn get_dependencies(expert: &str) -> Vec<&'static str> {
    match expert {
        "slow" => SlowExpert::dependencies(),
        "uppercase" => UppercaseExpert::dependencies(),
        "reverse" => ReverseExpert::dependencies(),
        "word_count" => WordCountExpert::dependencies(),
        "palindrome" => PalindromeExpert::dependencies(),
        "anagram" => AnagramExpert::dependencies(),
        "case_detector" => CaseDetectorExpert::dependencies(),
        "digit_sum" => DigitSumExpert::dependencies(),
        "keyword" => KeywordExpert::dependencies(),
        "text_cleaner" => TextCleanerExpert::dependencies(),
        _ => vec![],
    }
}
