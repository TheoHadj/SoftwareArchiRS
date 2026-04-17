use chrono::{Datelike, Local, NaiveDate, Utc};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use uuid::Uuid;

use crate::core::application::ports::{
    AbsenceRepository, EmployeeRepository, HeuresSuppRepository,
};
use crate::core::domain::entities::{ChoixEmploye, DemandeConge, Employe, HeuresSupplementaires};
use crate::core::domain::error::ErreurMetier;
use crate::core::domain::value_objects::generer_jours_feries_france;
use crate::core::domain::value_objects::{Periode, TypeAbsence};

pub struct AbsenceService {
    // Injection des dépendances via nos Ports (Interfaces)
    employee_repo: Arc<dyn EmployeeRepository>,
    abs_repo: Arc<dyn AbsenceRepository>,
    // en cache =>
    cache_jours_feries: RwLock<HashMap<i32, Vec<NaiveDate>>>,
}

impl AbsenceService {
    pub fn new(
        employee_repo: Arc<dyn EmployeeRepository>,
        abs_repo: Arc<dyn AbsenceRepository>,
    ) -> Self {
        let annee_actuelle = Utc::now().year();
        let mut cache = HashMap::new();
        cache.insert(annee_actuelle, generer_jours_feries_france(annee_actuelle));
        Self {
            employee_repo,
            abs_repo,
            cache_jours_feries: RwLock::new(cache),
        }
    }

    pub async fn poser_un_conge(
        &self,
        id_employe: Uuid,
        periode: Periode,
        type_absence: TypeAbsence,
    ) -> Result<(DemandeConge, f32), ErreurMetier> {
        // 1. Récupérer l'employé depuis le Port sortant
        let mut employe: Employe = self
            .employee_repo
            .find_by_id(id_employe)
            .await?
            .ok_or(ErreurMetier::EmployeIntrouvable)?;

        let aujourd_hui = Local::now().date_naive();

        let jours_feries = self.get_jours_feries(periode.date_debut(), periode.date_fin());
        let jours_a_deduire = periode.calculer_jours_reels(&jours_feries);

        if jours_a_deduire == 0.0 {
            return Err(ErreurMetier::PeriodeInvalide);
        }

        let nouvelle_demande =
            DemandeConge::poser(&mut employe, periode, type_absence, aujourd_hui)?;

        self.employee_repo.save(employe).await?;

        self.abs_repo.save(nouvelle_demande.clone()).await?;

        Ok((nouvelle_demande, jours_a_deduire))
    }

    pub async fn lister_employes(&self) -> Result<Vec<Employe>, ErreurMetier> {
        self.employee_repo.get_all().await
    }

    pub async fn lister_employes_by_id(
        &self,
        id_employe: Uuid,
    ) -> Result<Option<(Employe, Vec<DemandeConge>)>, ErreurMetier> {
        let employe_opt = self.employee_repo.find_by_id(id_employe).await?;

        if let Some(employe) = employe_opt {
            let conges = self.abs_repo.get_by_id(id_employe).await?;
            Ok(Some((employe, conges)))
        } else {
            Ok(None)
        }
    }
    //On ajoute les jours fériés non calculé au cache
    fn get_jours_feries(&self, date_debut: NaiveDate, date_fin: NaiveDate) -> Vec<NaiveDate> {
        let mut tous_les_jours: Vec<NaiveDate> = Vec::new();
        let annee_debut = date_debut.year();
        let annee_fin = date_fin.year();

        for annee in annee_debut..=annee_fin {
            let cache_lu = self.cache_jours_feries.read().unwrap();
            if let Some(jours) = cache_lu.get(&annee) {
                tous_les_jours.extend(jours);
                continue;
            }
            drop(cache_lu); // drop du lock sur le cache

            println!(
                "⚙️ Calcul dynamique des jours fériés pour l'année {}",
                annee
            );
            let nouveaux_jours = generer_jours_feries_france(annee);
            tous_les_jours.extend(nouveaux_jours.iter());

            let mut cache_ecrit = self.cache_jours_feries.write().unwrap();
            cache_ecrit.insert(annee, nouveaux_jours);
        }

        tous_les_jours
    }
}

pub struct HeuresSuppService {
    employee_repo: Arc<dyn EmployeeRepository>,
    hs_repo: Arc<dyn HeuresSuppRepository>,
}

impl HeuresSuppService {
    pub fn new(
        employee_repo: Arc<dyn EmployeeRepository>,
        hs_repo: Arc<dyn HeuresSuppRepository>,
    ) -> Self {
        Self {
            employee_repo,
            hs_repo,
        }
    }

    pub async fn declarer(&self, hs: HeuresSupplementaires) -> Result<(), ErreurMetier> {
        self.hs_repo.sauver(hs).await
    }

    pub async fn valider_et_appliquer(&self, id_hs: Uuid) -> Result<(), ErreurMetier> {
        let mut hs = self
            .hs_repo
            .trouver_par_id(id_hs)
            .await?
            .ok_or(ErreurMetier::HsIntrouvable)?;

        hs.valider_manager_direct();

        if hs.choix == ChoixEmploye::Recuperation {
            let mut employe = self
                .employee_repo
                .find_by_id(hs.id_employe)
                .await?
                .ok_or(ErreurMetier::EmployeIntrouvable)?;

            let jours_rtt_gagnes = hs.heures / 7.0;

            employe.modifier_quota(jours_rtt_gagnes, TypeAbsence::RTT, true);

            self.employee_repo.save(employe).await?;
        }

        // 4. On persiste l'état validé de la déclaration HS
        self.hs_repo.sauver(hs).await
    }

    pub async fn lister_par_employe(
        &self,
        id_employe: Uuid,
    ) -> Result<Vec<HeuresSupplementaires>, ErreurMetier> {
        // On passe simplement l'appel au repository
        self.hs_repo.lister_par_employe(id_employe).await
    }
}
