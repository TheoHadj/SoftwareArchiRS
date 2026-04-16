use axum::{routing::post, Router};
use std::sync::Arc;
use tokio::net::TcpListener;
use uuid::Uuid;

// Déclaration de notre arborescence de modules
mod core {
    pub mod domain {
        pub mod error;
        pub mod value_objects;
        pub mod entities;
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

use adapters::outgoing::mock_repository::MockRepository;
use adapters::outgoing::sqlite_repository::SqliteRepository;
use core::application::services::LeaveService;
use adapters::incoming::http_handlers::{poser_conge_handler, AppState};
use core::domain::entities::Employe;

#[tokio::main]
async fn main() {
    println!("🚀 Démarrage du SIRH...");


    let mock_repo = Arc::new(MockRepository::new());
    
    let id_employe_test = Uuid::new_v4();
    println!("uuid {id_employe_test}");
    let employe_test = Employe {
        id: id_employe_test,
        nom: "Hadj".to_string(),
        prenom: "Théo".to_string(),
        quota_urgence_familiale: 3,
    };
    mock_repo.insert_test_employee(employe_test.clone());

    

    // 1. Initialisation de l'Adaptateur Sortant (La Base de Données)
    // C'est ici qu'on fait le choix technologique de l'infrastructure
    // --- APRÈS (Quand tu auras créé ta base SQLite) ---
    // 1. On se connecte au fichier .sqlite
    let pool = sqlx::SqlitePool::connect("sqlite://mon_sirh.db").await.unwrap();
    // 2. On instancie le nouveau repository
    let sqlite_repo = Arc::new(SqliteRepository::new(pool));

    // --- SETUP DE TEST ---
    // On crée un faux employé pour pouvoir tester notre API
    let id_employe_test = Uuid::new_v4();
    let employe_test = Employe {
        id: id_employe_test,
        nom: "Dupont".to_string(),
        prenom: "Jean".to_string(),
        quota_urgence_familiale: 3, // Il a droit à 3 urgences
    };

    sqlite_repo.insert_test_employee(employe_test.clone());
    println!("👤 Employé de test créé avec l'ID : {}", id_employe_test);
    // ---------------------

    // 2. Initialisation du Cœur Métier (Injection des dépendances)
    // On passe le même repository pour les deux ports (Employee et Leave)
    let leave_service = Arc::new(LeaveService::new(
        mock_repo.clone(), 
        sqlite_repo.clone()
    ));

    // 3. Initialisation de l'Adaptateur Entrant (Le serveur Web)
    let state = AppState { leave_service };

    let app = Router::new()
        .route("/conges", post(poser_conge_handler))
        .with_state(state);

    // 4. Lancement du serveur
    let adresse = "127.0.0.1:3000";
    let listener = TcpListener::bind(adresse).await.unwrap();
    println!("🌐 Serveur lancé sur http://{}", adresse);
    
    axum::serve(listener, app).await.unwrap();
}