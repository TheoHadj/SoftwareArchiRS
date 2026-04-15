// src/adapters/outgoing/sqlite_repository.rs

use async_trait::async_trait;
use sqlx::SqlitePool;
use uuid::Uuid;

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
}


#[async_trait]
impl LeaveRepository for SqliteRepository {
    async fn save(&self, demande: DemandeConge) -> Result<(), ErreurMetier> {
        // Ici, on écrit du vrai SQL !
        // (Note: on convertit nos enums complexes en String pour SQLite)
        let type_absence_str = serde_json::to_string(&demande.type_absence).unwrap();
        let periode_str = serde_json::to_string(&demande.periode).unwrap();

        sqlx::query!(
            r#"
            INSERT INTO conges (id, id_employe, periode, type_absence)
            VALUES (?, ?, ?, ?)
            "#,
            demande.id.to_string(),
            demande.id_employe.to_string(),
            periode_str,
            type_absence_str
        )
        .execute(&self.pool)
        .await
        .map_err(|_| ErreurMetier::PeriodeInvalide)?; // En vrai on ferait une ErreurMetier::ErreurBaseDeDonnees

        Ok(())
    }
}
