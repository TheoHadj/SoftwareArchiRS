use async_trait::async_trait;
use uuid::Uuid;

use crate::core::domain::entities::{DemandeConge, Employe, HeuresSupplementaires};
use crate::core::domain::error::ErreurMetier;

// Port pour gérer les employés (nécessaire pour lire/sauvegarder le quota d'urgence)
#[async_trait]
pub trait EmployeeRepository: Send + Sync {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Employe>, ErreurMetier>;
    async fn save(&self, employe: Employe) -> Result<(), ErreurMetier>;
}

// Port pour gérer les demandes de congés
#[async_trait]
pub trait LeaveRepository: Send + Sync {
    async fn save(&self, demande: DemandeConge) -> Result<(), ErreurMetier>;
    // On pourrait ajouter `find_by_employee_id` pour la route GET plus tard
}

// Port pour gérer les heures supplémentaires
#[async_trait]
pub trait OvertimeRepository: Send + Sync {
    async fn save(&self, heures: HeuresSupplementaires) -> Result<(), ErreurMetier>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<HeuresSupplementaires>, ErreurMetier>;
}