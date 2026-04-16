// src/adapters/outgoing/sqlite_repository.rs
use async_trait::async_trait;
use sqlx::Row;
use sqlx::SqlitePool;
use uuid::Uuid;

use crate::core::application::ports::{EmployeeRepository, AbsenceRepository};
use crate::core::domain::entities::{DemandeConge, Employe};
use crate::core::domain::error::ErreurMetier;

pub struct SqliteRepository {
    pool: SqlitePool,
}

impl SqliteRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub fn insert_test_employee(&self, employe: Employe) {
        println!("L'employé {} a été créé (enfin, simulé !)", employe.nom);
    }
}

#[async_trait]
impl EmployeeRepository for SqliteRepository {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Employe>, ErreurMetier> {
        let result = sqlx::query(
            "SELECT id, nom, prenom, quota_urgence_familiale FROM employes WHERE id = ?",
        )
        .bind(id.to_string())
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| {
            println!("🚨 ERREUR SQLITE find_by_id : {:?}", e);
            ErreurMetier::EngineError
        })?;

        if let Some(row) = result {
            let id_str: String = row.get("id");
            Ok(Some(Employe {
                id: Uuid::parse_str(&id_str).unwrap(),
                nom: row.get("nom"),
                prenom: row.get("prenom"),
                quota_urgence_familiale: row.get::<i64, _>("quota_urgence_familiale") as u32,
            }))
        } else {
            Ok(None)
        }
    }

    async fn save(&self, employe: Employe) -> Result<(), ErreurMetier> {
        // "Upsert" : Insère, ou met à jour si l'ID existe déjà (pratique pour le quota !)
        sqlx::query(
            r#"
            INSERT INTO employes (id, nom, prenom, quota_urgence_familiale)
            VALUES (?, ?, ?, ?)
            ON CONFLICT(id) DO UPDATE SET 
                quota_urgence_familiale = excluded.quota_urgence_familiale
            "#,
        )
        .bind(employe.id.to_string())
        .bind(&employe.nom)
        .bind(&employe.prenom)
        .bind(employe.quota_urgence_familiale as i64) // SQLite préfère le i64
        .execute(&self.pool)
        .await
        .map_err(|e| {
            println!("🚨 ERREUR SQLITE save_employe : {:?}", e);
            ErreurMetier::EngineError
        })?;

        Ok(())
    }

    async fn get_all(&self) -> Result<Vec<Employe>, ErreurMetier> {
        let rows = sqlx::query("SELECT id, nom, prenom, quota_urgence_familiale FROM employes")
            .fetch_all(&self.pool)
            .await
            .map_err(|e| {
                println!("🚨 ERREUR SQLITE get_all : {:?}", e);
                ErreurMetier::EngineError
            })?;

        let mut employes = Vec::new();
        for row in rows {
            let id_str: String = row.get("id");
            employes.push(Employe {
                id: Uuid::parse_str(&id_str).unwrap(),
                nom: row.get("nom"),
                prenom: row.get("prenom"),
                quota_urgence_familiale: row.get::<i64, _>("quota_urgence_familiale") as u32,
            });
        }
        Ok(employes)
    }
}

#[async_trait]
impl AbsenceRepository for SqliteRepository {
    async fn get_by_id(&self, id_employe: Uuid) -> Result<Vec<DemandeConge>, ErreurMetier> {
        let rows = sqlx::query(
            "SELECT id, id_employe, periode, type_absence FROM conges WHERE id_employe = ?",
        )
        .bind(id_employe.to_string())
        .fetch_all(&self.pool)
        .await
        .map_err(|e| {
            println!("🚨 ERREUR SQLITE lister_par_employe : {:?}", e);
            ErreurMetier::EngineError
        })?;

        let mut conges = Vec::new();
        for row in rows {
            let id_str: String = row.get("id");
            let id_emp_str: String = row.get("id_employe");
            let periode_str: String = row.get("periode");
            let type_absence_str: String = row.get("type_absence");

            conges.push(DemandeConge {
                id: Uuid::parse_str(&id_str).unwrap(),
                id_employe: Uuid::parse_str(&id_emp_str).unwrap(),
                // On redécode le JSON stocké en texte dans la base
                periode: serde_json::from_str(&periode_str).unwrap(),
                type_absence: serde_json::from_str(&type_absence_str).unwrap(),
            });
        }
        Ok(conges)
    }

    async fn save(&self, demande: DemandeConge) -> Result<(), ErreurMetier> {
        let type_absence_str = serde_json::to_string(&demande.type_absence).unwrap();
        let periode_str = serde_json::to_string(&demande.periode).unwrap();

        // 1. On utilise query() SANS le point d'exclamation
        // 2. On attache les variables avec .bind()
        sqlx::query(
            r#"
            INSERT INTO conges (id, id_employe, periode, type_absence)
            VALUES (?, ?, ?, ?)
            "#,
        )
        .bind(demande.id.to_string())
        .bind(demande.id_employe.to_string())
        .bind(periode_str)
        .bind(type_absence_str)
        .execute(&self.pool)
        .await
        .map_err(|_| ErreurMetier::EngineError)?;

        println!("💾 Congé sauvegardé dans SQLite !");
        Ok(())
    }
}
