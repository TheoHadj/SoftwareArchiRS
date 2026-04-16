use thiserror::Error;

#[derive(Debug, Error)]
pub enum ErreurMetier {
    #[error("Interdit de poser pour le jour même hors motif Urgence Familiale.")]
    AnticipationNonRespectee,
    
    #[error("Quota d'urgence familiale insuffisant.")]
    QuotaUrgenceEpuise,
    
    #[error("Transformation impossible : en attente de validation du manager direct.")]
    EnAttenteManager,
    
    #[error("La période saisie est invalide (dates incohérentes ou configuration de demi-journée impossible).")]
    PeriodeInvalide,
    
    #[error("Employé introuvable")]
    EmployeIntrouvable,

    #[error("EngineError")]
    EngineError,
}