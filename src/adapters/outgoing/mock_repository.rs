// src/adapters/outgoing/in_memory_repository.rs

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use async_trait::async_trait;
use uuid::Uuid;

use crate::core::application::ports::{EmployeeRepository, LeaveRepository};
use crate::core::domain::entities::{DemandeConge, Employe};
use crate::core::domain::error::ErreurMetier;

#[derive(Clone)]
pub struct MockRepository {
    employes: Arc<Mutex<HashMap<Uuid, Employe>>>,
    conges: Arc<Mutex<HashMap<Uuid, DemandeConge>>>,
}

impl MockRepository {
    pub fn new() -> Self {
        Self {
            employes: Arc::new(Mutex::new(HashMap::new())),
            conges: Arc::new(Mutex::new(HashMap::new())),
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
        let lock = self.employes.lock().unwrap(); // On ouvre le cadenas de la RAM
        // On prend toutes les valeurs du HashMap, on les clone, et on en fait une Liste (Vec)
        let list: Vec<Employe> = lock.values().cloned().collect(); 
        Ok(list)
    }
}

#[async_trait]
impl LeaveRepository for MockRepository {
    async fn save(&self, demande: DemandeConge) -> Result<(), ErreurMetier> {
        let mut db = self.conges.lock().unwrap();
        db.insert(demande.id, demande);
        Ok(())
    }
}