//contient un enum des rôles pour le système mycélien ia
use super::analysis_quality::AnalysisQuality;
use super::quality_judge::QualityJudge;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Roles {
    /// Détermine quels rôles doivent être activés pour une tâche donnée
    RoleSelector,
    /// Tous les rôles quality_judge du domaine
    QualityJudge(QualityJudge),
    /// Fusionne et synthétise les réponses pour produire une sortie cohérente
    Synthesizer,
    /// Porte la réponse finale vers l'extérieur ou l'utilisateur
    Spokesperson,
    /// Analyse globale de la qualité
    AnalysisQuality(AnalysisQuality),
}

impl Roles {
    /// Retourne les indices d'exécution où ce rôle doit être activé
    pub fn exec_orders(&self) -> Vec<usize> {
        match self {
            Roles::QualityJudge(_) => vec![0, 2], // ex: contrôle avant et après synthèse
            Roles::Synthesizer => vec![1],
            Roles::Spokesperson => vec![3],
            Roles::RoleSelector => vec![99],
            Roles::AnalysisQuality(_) => vec![4],
        }
    }
}
