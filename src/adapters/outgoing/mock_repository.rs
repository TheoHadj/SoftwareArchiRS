use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use uuid::Uuid;

use crate::core::application::ports::{
    AbsenceRepository, EmployeeRepository, HeuresSuppRepository,
};
use crate::core::domain::entities::{DemandeConge, Employe, HeuresSupplementaires};
use crate::core::domain::error::ErreurMetier;

#[derive(Clone)]
pub struct MockRepository {
    employes: Arc<Mutex<HashMap<Uuid, Employe>>>,
    conges: Arc<Mutex<HashMap<Uuid, DemandeConge>>>,
    heures_supp: Arc<Mutex<HashMap<Uuid, HeuresSupplementaires>>>,
}

impl MockRepository {
    pub fn new() -> Self {
        Self {
            employes: Arc::new(Mutex::new(HashMap::new())),
            conges: Arc::new(Mutex::new(HashMap::new())),
            heures_supp: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn insert_test_employee(&self, employe: Employe) {
        let mut db = self.employes.lock().unwrap();
        db.insert(employe.id, employe);
    }
}

#[async_trait]
impl EmployeeRepository for MockRepository {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Employe>, ErreurMetier> {
        let db = self.employes.lock().unwrap();
        Ok(db.get(&id).cloned())
    }

    async fn save(&self, employe: Employe) -> Result<(), ErreurMetier> {
        let mut db = self.employes.lock().unwrap();
        db.insert(employe.id, employe);
        Ok(())
    }

    async fn get_all(&self) -> Result<Vec<Employe>, ErreurMetier> {
        let lock = self.employes.lock().unwrap();
        let list: Vec<Employe> = lock.values().cloned().collect();
        Ok(list)
    }
}

#[async_trait]
impl AbsenceRepository for MockRepository {
    async fn save(&self, demande: DemandeConge) -> Result<(), ErreurMetier> {
        let mut db = self.conges.lock().unwrap();
        db.insert(demande.id, demande);
        Ok(())
    }

    async fn get_by_id(&self, id_employe: Uuid) -> Result<Vec<DemandeConge>, ErreurMetier> {
        let db = self.conges.lock().unwrap();

        Ok(db
            .values()
            .filter(|c| c.id_employe == id_employe)
            .cloned()
            .collect())
    }
}

#[async_trait]
impl HeuresSuppRepository for MockRepository {
    async fn sauver(&self, hs: HeuresSupplementaires) -> Result<(), ErreurMetier> {
        let mut store = self.heures_supp.lock().unwrap();

        store.insert(hs.id, hs);
        Ok(())
    }

    async fn trouver_par_id(
        &self,
        id: Uuid,
    ) -> Result<Option<HeuresSupplementaires>, ErreurMetier> {
        let store = self.heures_supp.lock().unwrap();

        Ok(store.get(&id).cloned())
    }

    async fn lister_par_employe(
        &self,
        id_employe: Uuid,
    ) -> Result<Vec<HeuresSupplementaires>, ErreurMetier> {
        let store = self.heures_supp.lock().unwrap();

        let liste = store
            .values()
            .filter(|hs| hs.id_employe == id_employe)
            .cloned()
            .collect();

        Ok(liste)
    }
}
