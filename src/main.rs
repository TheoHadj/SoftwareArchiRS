use axum::{
    Router,
    routing::{get, post},
};
use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
use std::str::FromStr;
use std::{env, sync::Arc};

mod core {
    pub mod domain {
        pub mod entities;
        pub mod error;
        pub mod value_objects;
    }
    pub mod application {
        pub mod ports;
        pub mod services;
    }
}
mod adapters {
    pub mod incoming {
        pub mod http_handlers;
    }
    pub mod outgoing {
        pub mod mock_repository;
        pub mod sqlite_repository;
    }
}

use adapters::incoming::http_handlers::{
    AppState, declarer_hs_handler, lister_employes_by_id_handler, lister_employes_handler,
    lister_hs_by_employe_handler, poser_conge_handler, valider_hs_handler,
};
use core::application::ports::{AbsenceRepository, EmployeeRepository, HeuresSuppRepository};

#[tokio::main]
async fn main() {
    println!("🚀 Démarrage du SIRH...");

    // var d'env $env = true ==> sqlite X Mock V
    let use_mock = env::var("USE_MOCK").unwrap_or_else(|_| "false".to_string()) == "true";

    // 2. Préparation des variables qui vont tenir nos bases de données
    let employee_repo: Arc<dyn EmployeeRepository>;
    let abs_repo: Arc<dyn AbsenceRepository>;
    let hs_repo: Arc<dyn HeuresSuppRepository>;

    if use_mock {
        println!("🛠️ MODE TEST : Base de données en mémoire (Mock)");
        let mock = Arc::new(adapters::outgoing::mock_repository::MockRepository::new());
        employee_repo = mock.clone();
        abs_repo = mock.clone();
        hs_repo = mock.clone();
    } else {
        println!("🛢️ MODE PRODUCTION : Connexion à SQLite...");

        // 1. On demande à SQLx de créer le fichier s'il n'existe pas
        let options = SqliteConnectOptions::from_str("sqlite://mon_sirh.db")
            .unwrap()
            .create_if_missing(true);

        let pool = SqlitePoolOptions::new()
            .connect_with(options)
            .await
            .expect("❌ Impossible de se connecter ou créer la base SQLite");

        // 2. On exécute le script d'initialisation (Tables + Données par défaut)
        println!("🏗️ Vérification et création des tables...");
        sqlx::query(
            r#"
            DROP TABLE IF EXISTS heures_supplementaires;
            DROP TABLE IF EXISTS conges;
            DROP TABLE IF EXISTS employes;

            CREATE TABLE IF NOT EXISTS employes (
                id TEXT PRIMARY KEY,
                nom TEXT NOT NULL,
                prenom TEXT NOT NULL,
                quota_urgence_familiale INTEGER NOT NULL,
                quota_conges INTEGER NOT NULL,
                quota_rtt INTEGER NOT NULL
            );

            CREATE TABLE IF NOT EXISTS conges (
                id TEXT PRIMARY KEY,
                id_employe TEXT NOT NULL,
                periode TEXT NOT NULL,
                type_absence TEXT NOT NULL,
                FOREIGN KEY (id_employe) REFERENCES employes(id)
            );

            CREATE TABLE IF NOT EXISTS heures_supplementaires (
                id TEXT PRIMARY KEY,
                id_employe TEXT NOT NULL,
                heures REAL NOT NULL,
                date TEXT NOT NULL,
                choix TEXT NOT NULL,
                validation_manager BOOLEAN NOT NULL CHECK (validation_manager IN (0, 1)),
                FOREIGN KEY (id_employe) REFERENCES employes(id)
            );

            INSERT INTO employes (id, nom, prenom, quota_urgence_familiale, quota_conges, quota_rtt) 
            VALUES ('f098ab96-3799-4481-b0d5-d0c3bfe509b0', 'Dupont', 'Jean', 3.0, 25.0, 0.0)
            ON CONFLICT(id) DO NOTHING;
            "#,
        )
        .execute(&pool)
        .await
        .expect("❌ Impossible d'initialiser les tables SQLite");

        println!("✅ Base de données prête !");

        let sqlite = Arc::new(adapters::outgoing::sqlite_repository::SqliteRepository::new(pool));

        employee_repo = sqlite.clone();
        abs_repo = sqlite.clone();
        hs_repo = sqlite.clone();
    }

    // Injection des repository dans service
    let hs_service = Arc::new(core::application::services::HeuresSuppService::new(
        employee_repo.clone(),
        hs_repo.clone(),
    ));

    let abs_service = Arc::new(core::application::services::AbsenceService::new(
        employee_repo.clone(),
        abs_repo.clone(),
    ));

    let state = AppState {
        abs_service,
        hs_service,
    };

    // 4. Lancement du serveur Axum
    let app = Router::new()
        .route("/conges", post(poser_conge_handler))
        .route("/employes", get(lister_employes_handler))
        .route("/employes/:id", get(lister_employes_by_id_handler))
        .route("/heures-supp", post(declarer_hs_handler))
        .route("/heures-supp/:id", post(valider_hs_handler))
        .route(
            "/employes/:id/heures-supp",
            get(lister_hs_by_employe_handler),
        )
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("🌐 Serveur lancé sur http://127.0.0.1:3000");
    axum::serve(listener, app).await.unwrap();
}
