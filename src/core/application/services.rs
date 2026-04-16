use std::sync::Arc;
use uuid::Uuid;
use chrono::Local;

use crate::core::application::ports::{EmployeeRepository, LeaveRepository};
use crate::core::domain::entities::{DemandeConge, Employe};
use crate::core::domain::error::ErreurMetier;
use crate::core::domain::value_objects::{Periode, TypeAbsence};

pub struct LeaveService {
    // Injection des dépendances via nos Ports (Interfaces)
    employee_repo: Arc<dyn EmployeeRepository>,
    leave_repo: Arc<dyn LeaveRepository>,
}

impl LeaveService {
    pub fn new(
        employee_repo: Arc<dyn EmployeeRepository>,
        leave_repo: Arc<dyn LeaveRepository>,
    ) -> Self {
        Self {
            employee_repo,
            leave_repo,
        }
    }

    /// Le Cas d'Utilisation principal : Poser un congé
    pub async fn poser_un_conge(
        &self,
        id_employe: Uuid,
        periode: Periode,
        type_absence: TypeAbsence,
    ) -> Result<DemandeConge, ErreurMetier> {
        
        // 1. Récupérer l'employé depuis le Port sortant
        // Si find_by_id renvoie None, on lève une erreur (ici on triche un peu en réutilisant une erreur métier existante pour l'exemple)
        let mut employe = self.employee_repo.find_by_id(id_employe).await?
            .ok_or(ErreurMetier::EmployeIntrouvable)?; // En vrai on ferait une ErreurMetier::EmployeIntrouvable

        let aujourd_hui = Local::now().date_naive();

        // 2. Faire travailler le Domaine (Le cœur métier)
        // La méthode `poser` va vérifier l'anticipation et déduire le quota si besoin.
        let nouvelle_demande = DemandeConge::poser(
            &mut employe,
            periode,
            type_absence,
            aujourd_hui,
        )?;

        // 3. Sauvegarder les changements d'état
        // Si le quota a été consommé, il faut sauvegarder l'employé !
        self.employee_repo.save(employe).await?;
        
        // On sauvegarde la nouvelle demande
        self.leave_repo.save(nouvelle_demande.clone()).await?;

        // 4. Retourner le résultat à l'API
        Ok(nouvelle_demande)
    }

    pub async fn lister_employes(&self) -> Result<Vec<Employe>, ErreurMetier> {
        // Le service délègue simplement au repository
        self.employee_repo.get_all().await
    }

}