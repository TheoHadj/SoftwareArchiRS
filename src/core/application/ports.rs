use async_trait::async_trait;
use uuid::Uuid;

use crate::core::domain::entities::{DemandeConge, Employe, HeuresSupplementaires};
use crate::core::domain::error::ErreurMetier;

// Port pour gérer les employés (nécessaire pour lire/sauvegarder le quota d'urgence)
#[async_trait]
pub trait EmployeeRepository: Send + Sync {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Employe>, ErreurMetier>;
    async fn save(&self, employe: Employe) -> Result<(), ErreurMetier>;
    async fn get_all(&self) -> Result<Vec<Employe>, ErreurMetier>;
}

// Port pour gérer les demandes de congés
#[async_trait]
pub trait LeaveRepository: Send + Sync {
    async fn save(&self, demande: DemandeConge) -> Result<(), ErreurMetier>;

    async fn get_by_id(&self, id_employe: Uuid) -> Result<Vec<DemandeConge>, ErreurMetier>;
}

// Port pour gérer les heures supplémentaires
#[async_trait]
pub trait OvertimeRepository: Send + Sync {
    async fn save(&self, heures: HeuresSupplementaires) -> Result<(), ErreurMetier>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<HeuresSupplementaires>, ErreurMetier>;
}
