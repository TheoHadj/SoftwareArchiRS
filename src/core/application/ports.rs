use async_trait::async_trait;
use uuid::Uuid;

use crate::core::domain::entities::{DemandeConge, Employe, HeuresSupplementaires};
use crate::core::domain::error::ErreurMetier;

#[async_trait]
pub trait EmployeeRepository: Send + Sync {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Employe>, ErreurMetier>;
    async fn save(&self, employe: Employe) -> Result<(), ErreurMetier>;
    async fn get_all(&self) -> Result<Vec<Employe>, ErreurMetier>;
}

#[async_trait]
pub trait AbsenceRepository: Send + Sync {
    async fn save(&self, demande: DemandeConge) -> Result<(), ErreurMetier>;

    async fn get_by_id(&self, id_employe: Uuid) -> Result<Vec<DemandeConge>, ErreurMetier>;
}

#[async_trait]
pub trait HeuresSuppRepository: Send + Sync {
    async fn sauver(&self, hs: HeuresSupplementaires) -> Result<(), ErreurMetier>;
    async fn trouver_par_id(&self, id: Uuid)
    -> Result<Option<HeuresSupplementaires>, ErreurMetier>;
    async fn lister_par_employe(
        &self,
        id_employe: Uuid,
    ) -> Result<Vec<HeuresSupplementaires>, ErreurMetier>;
}
