// src/adapters/outgoing/sqlite_repository.rs

use std::f32::consts::E;

use async_trait::async_trait;
use sqlx::SqlitePool;

// use crate::core::application::ports::{EmployeeRepository, LeaveRepository};
// use crate::core::domain::entities::{DemandeConge, Employe};
use crate::core::application::ports::{EmployeeRepository, LeaveRepository};
use crate::core::domain::entities::{DemandeConge, Employe};
use crate::core::domain::error::ErreurMetier;

pub struct SqliteRepository {
    // Au lieu d'un HashMap, on stocke la connexion à la vraie base de données
    pool: SqlitePool,
}

impl SqliteRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub fn insert_test_employee(&self, employe : Employe){
        println!("L'employé {} a été créé (enfin, simulé !)", employe.nom);    }
}


#[async_trait]
impl LeaveRepository for SqliteRepository {
    async fn save(&self, demande: DemandeConge) -> Result<(), ErreurMetier> {
        let type_absence_str = serde_json::to_string(&demande.type_absence).unwrap();
        let periode_str = serde_json::to_string(&demande.periode).unwrap();

        // 1. On utilise query() SANS le point d'exclamation
        // 2. On attache les variables avec .bind()
        sqlx::query(
            r#"
            INSERT INTO conges (id, id_employe, periode, type_absence)
            VALUES (?, ?, ?, ?)
            "#
        )
        .bind(demande.id.to_string())
        .bind(demande.id_employe.to_string())
        .bind(periode_str)
        .bind(type_absence_str)
        .execute(&self.pool)
        .await
        .map_err(|_| ErreurMetier::PeriodeInvalide)?; 

        println!("💾 Congé sauvegardé dans SQLite !");
        Ok(())
    }
}