use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;
use chrono::NaiveDate;

use crate::core::application::services::LeaveService; // Ou use_cases::LeaveService si tu as renommé !
use crate::core::domain::value_objects::{MomentDebut, MomentFin, Periode, TypeAbsence};

// --------------------------------------------------------
// L'État de l'Application (Dependency Injection)
// --------------------------------------------------------
/// Cette structure va contenir notre Service. Elle sera partagée
/// à travers toutes nos routes Axum.
#[derive(Clone)]
pub struct AppState {
    pub leave_service: Arc<LeaveService>,
}

// --------------------------------------------------------
// Les DTOs (Data Transfer Objects)
// --------------------------------------------------------
/// Ce que le client envoie dans le body de sa requête POST
#[derive(Deserialize)]
pub struct PoserCongeRequestDto {
    pub id_employe: Uuid,
    pub date_debut: NaiveDate,
    pub moment_debut: MomentDebut,
    pub date_fin: NaiveDate,
    pub moment_fin: MomentFin,
    pub type_absence: TypeAbsence,
}

/// Ce que l'API renvoie en cas de succès
#[derive(Serialize)]
pub struct PoserCongeResponseDto {
    pub id_demande: Uuid,
    pub jours_deduits: f32,
    pub message: String,
}

// --------------------------------------------------------
// Le Contrôleur (Handler)
// --------------------------------------------------------
pub async fn poser_conge_handler(
    State(state): State<AppState>,
    Json(payload): Json<PoserCongeRequestDto>,
) -> impl IntoResponse {
    
    // 1. Transformation du DTO en Objet de Valeur (Validation de 1er niveau)
    let periode_result = Periode::nouvelle(
        payload.date_debut,
        payload.moment_debut,
        payload.date_fin,
        payload.moment_fin,
    );
    println!("Périoderes : {:?}", periode_result);


    let periode = match periode_result {
        Ok(p) => p,
        Err(e) => return (StatusCode::BAD_REQUEST, e.to_string()).into_response(),
    };

    println!("Période : {:?}", periode);


    // 2. Appel du Cas d'Utilisation (Le Service)
    let resultat_service = state.leave_service.poser_un_conge(
        payload.id_employe,
        periode,
        payload.type_absence,
    ).await;

    println!("result svc : {:?}", resultat_service);


    // 3. Gestion de la réponse (Traduction du Domaine vers le Web)
    match resultat_service {
        Ok(demande) => {
            println!("OK match res");

            // Pour l'exemple, on simule l'absence de jours fériés. 
            // En vrai, tu passerais l'instance de ta stratégie ici.
            let jours_feries_vides = vec![]; 
            let jours_deduits = demande.periode.calculer_jours_reels(&jours_feries_vides);

            let response = PoserCongeResponseDto {
                id_demande: demande.id,
                jours_deduits,
                message: "Demande de congé enregistrée avec succès.".to_string(),
            };
            (StatusCode::CREATED, Json(response)).into_response()
        }
        Err(erreur_metier) => {
            println!("PAS OK match res");

            // Toutes nos erreurs métier sont des BAD_REQUEST ou CONFLICT
            (StatusCode::BAD_REQUEST, erreur_metier.to_string()).into_response()
        }
    }
}
// 1. Le DTO (Ce qu'on va renvoyer en JSON)
#[derive(Serialize)]
pub struct EmployeResponseDto {
    pub id: Uuid,
    pub nom: String,
    pub prenom: String,
    // On peut cacher des infos internes comme le quota si on veut !
}

// 2. Le Contrôleur
pub async fn lister_employes_handler(
    State(state): State<AppState>,
) -> impl IntoResponse {
    
    match state.leave_service.lister_employes().await {
        Ok(employes) => {
            // On transforme nos Entités métier en DTOs pour le Web
            let dtos: Vec<EmployeResponseDto> = employes.into_iter().map(|e| EmployeResponseDto {
                id: e.id,
                nom: e.nom,
                prenom: e.prenom,
            }).collect();
            
            (StatusCode::OK, Json(dtos)).into_response()
        }
        Err(_) => {
            (StatusCode::INTERNAL_SERVER_ERROR, "Erreur serveur".to_string()).into_response()
        }
    }
}