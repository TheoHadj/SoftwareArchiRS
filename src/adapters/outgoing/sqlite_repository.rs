use async_trait::async_trait;
use chrono::NaiveDate;
use sqlx::Row;
use sqlx::SqlitePool;
use uuid::Uuid;

use crate::core::application::ports::{
    AbsenceRepository, EmployeeRepository, HeuresSuppRepository,
};
use crate::core::domain::entities::{ChoixEmploye, DemandeConge, Employe, HeuresSupplementaires};
use crate::core::domain::error::ErreurMetier;

pub struct SqliteRepository {
    pool: SqlitePool,
}

impl SqliteRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl EmployeeRepository for SqliteRepository {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Employe>, ErreurMetier> {
        let result = sqlx::query(
            "SELECT id, nom, prenom, quota_urgence_familiale, quota_conges, quota_rtt FROM employes WHERE id = ?",
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
                quota_urgence_familiale: row.get::<i64, _>("quota_urgence_familiale") as f32,
                quota_conges: row.get::<i64, _>("quota_conges") as f32,
                quota_rtt: row.get::<i64, _>("quota_rtt") as f32,
            }))
        } else {
            Ok(None)
        }
    }

    async fn save(&self, employe: Employe) -> Result<(), ErreurMetier> {
        sqlx::query(
            r#"
            INSERT INTO employes (id, nom, prenom, quota_urgence_familiale, quota_conges, quota_rtt)
            VALUES (?, ?, ?, ?, ?, ?)
            ON CONFLICT(id) DO UPDATE SET 
                quota_urgence_familiale = excluded.quota_urgence_familiale
                quota_conges = excluded.quota_conges,
                quota_rtt = excluded.quota_rtt
                
            "#,
        )
        .bind(employe.id.to_string())
        .bind(&employe.nom)
        .bind(&employe.prenom)
        .bind(employe.quota_urgence_familiale as i64)
        .bind(employe.quota_conges as i64)
        .bind(employe.quota_rtt as i64)
        .execute(&self.pool)
        .await
        .map_err(|e| {
            println!("🚨 ERREUR SQLITE save_employe : {:?}", e);
            ErreurMetier::EngineError
        })?;

        Ok(())
    }

    async fn get_all(&self) -> Result<Vec<Employe>, ErreurMetier> {
        let rows = sqlx::query("SELECT id, nom, prenom, quota_urgence_familiale, quota_conges, quota_rtt FROM employes")
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
                quota_urgence_familiale: row.get::<i64, _>("quota_urgence_familiale") as f32,
                quota_conges: row.get::<i64, _>("quota_conges") as f32,
                quota_rtt: row.get::<i64, _>("quota_rtt") as f32,
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
                periode: serde_json::from_str(&periode_str).unwrap(),
                type_absence: serde_json::from_str(&type_absence_str).unwrap(),
            });
        }
        Ok(conges)
    }

    async fn save(&self, demande: DemandeConge) -> Result<(), ErreurMetier> {
        let type_absence_str = serde_json::to_string(&demande.type_absence).unwrap();
        let periode_str = serde_json::to_string(&demande.periode).unwrap();

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

#[async_trait]
impl HeuresSuppRepository for SqliteRepository {
    async fn sauver(&self, hs: HeuresSupplementaires) -> Result<(), ErreurMetier> {
        let id_str = hs.id.to_string();
        let id_employe_str = hs.id_employe.to_string();
        let date_str = hs.date.to_string();

        let choix_str = match hs.choix {
            ChoixEmploye::Paiement => "Paiement",
            ChoixEmploye::Recuperation => "Recuperation",
        };

        sqlx::query(
            r#"
            INSERT INTO heures_supplementaires (id, id_employe, heures, date, choix, validation_manager)
            VALUES ($1, $2, $3, $4, $5, $6)
            ON CONFLICT(id) DO UPDATE SET
                heures = excluded.heures,
                date = excluded.date,
                choix = excluded.choix,
                validation_manager = excluded.validation_manager
            "#
        )
        .bind(id_str)
        .bind(id_employe_str)
        .bind(hs.heures)
        .bind(date_str)
        .bind(choix_str)
        .bind(hs.validation_manager)
        .execute(&self.pool)
        .await
        .map_err(|e| {
            println!("Erreur DB (Sauvegarde HS) : {:?}", e);
            ErreurMetier::EmployeIntrouvable
        })?;

        Ok(())
    }

    async fn trouver_par_id(
        &self,
        id: Uuid,
    ) -> Result<Option<HeuresSupplementaires>, ErreurMetier> {
        let id_str = id.to_string();

        let row = sqlx::query(
            "SELECT id, id_employe, heures, date, choix, validation_manager FROM heures_supplementaires WHERE id = $1"
        )
        .bind(id_str)
        .fetch_optional(&self.pool)
        .await
        .map_err(|_| ErreurMetier::EmployeIntrouvable)?;

        if let Some(row) = row {
            use sqlx::Row;

            let hs_id = Uuid::parse_str(row.get("id")).unwrap();
            let emp_id = Uuid::parse_str(row.get("id_employe")).unwrap();
            let date = NaiveDate::parse_from_str(row.get("date"), "%Y-%m-%d").unwrap();

            let choix_str: String = row.get("choix");
            let choix = if choix_str == "Paiement" {
                ChoixEmploye::Paiement
            } else {
                ChoixEmploye::Recuperation
            };

            Ok(Some(HeuresSupplementaires {
                id: hs_id,
                id_employe: emp_id,
                heures: row.get("heures"),
                date,
                choix,
                validation_manager: row.get("validation_manager"),
            }))
        } else {
            Ok(None)
        }
    }

    //à supprimer
    async fn lister_par_employe(
        &self,
        _id_employe: Uuid,
    ) -> Result<Vec<HeuresSupplementaires>, ErreurMetier> {
        Ok(vec![])
    }
}
