use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::core::domain::error::ErreurMetier;
use crate::core::domain::value_objects::{Periode, TypeAbsence};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Employe {
    pub id: Uuid,
    pub nom: String,
    pub prenom: String,
    pub quota_urgence_familiale: u32,
}

impl Employe {
    pub fn consommer_quota_urgence(&mut self) -> Result<(), ErreurMetier> {
        if self.quota_urgence_familiale == 0 {
            return Err(ErreurMetier::QuotaUrgenceEpuise);
        }
        self.quota_urgence_familiale -= 1;
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DemandeConge {
    pub id: Uuid,
    pub id_employe: Uuid,
    pub periode: Periode,
    pub type_absence: TypeAbsence,
}

impl DemandeConge {
    pub fn poser(
        employe: &mut Employe,
        periode: Periode,
        type_absence: TypeAbsence,
        aujourd_hui: NaiveDate,
    ) -> Result<Self, ErreurMetier> {
        if periode.date_debut() == aujourd_hui {
            if type_absence == TypeAbsence::UrgenceFamiliale {
                employe.consommer_quota_urgence()?;
            } else {
                return Err(ErreurMetier::AnticipationNonRespectee);
            }
        }

        Ok(Self {
            id: Uuid::new_v4(),
            id_employe: employe.id,
            periode,
            type_absence,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum ChoixEmploye {
    Paiement,
    Recuperation,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HeuresSupplementaires {
    pub id: Uuid,
    pub id_employe: Uuid,
    pub heures: f32,
    pub choix: ChoixEmploye,
    pub validation_manager: bool,
}

impl HeuresSupplementaires {
    pub fn declarer(id_employe: Uuid, heures: f32, choix: ChoixEmploye) -> Self {
        Self {
            id: Uuid::new_v4(),
            id_employe,
            heures,
            choix,
            validation_manager: false,
        }
    }

    pub fn valider_manager_direct(&mut self) {
        self.validation_manager = true;
    }

    pub fn transformer(&self) -> Result<(), ErreurMetier> {
        if !self.validation_manager {
            return Err(ErreurMetier::EnAttenteManager);
        }
        Ok(())
    }
}
