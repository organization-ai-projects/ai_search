//contient un enum des rôles pour le système mycélien ia
use super::quality_judge::QualityJudge;
use super::reviewer::Reviewer;
use super::validator::Validator;
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
    /// Tous les rôles reviewer du domaine
    Reviewer(Reviewer),
    /// Tous les rôles validator du domaine
    Validator(Validator),
}
