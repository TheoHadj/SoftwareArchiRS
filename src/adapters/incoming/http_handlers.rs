use axum::{Json, extract::Path, extract::State, http::StatusCode, response::IntoResponse};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

use crate::core::application::services::{AbsenceService, HeuresSuppService};
use crate::core::domain::entities::{ChoixEmploye, HeuresSupplementaires};
use crate::core::domain::value_objects::{MomentDebut, MomentFin, Periode, TypeAbsence};

// --------------------------------------------------------
// Dependency Injection
// --------------------------------------------------------

#[derive(Clone)]
pub struct AppState {
    pub abs_service: Arc<AbsenceService>,
    pub hs_service: Arc<HeuresSuppService>,
}

// --------------------------------------------------------
// Les DTO
// --------------------------------------------------------

#[derive(Serialize)]
pub struct CongeResponseDto {
    pub id: Uuid,
    pub type_absence: TypeAbsence,
    pub periode: Periode,
}

#[derive(Serialize)]
pub struct EmployeDetailResponseDto {
    pub id: Uuid,
    pub nom: String,
    pub prenom: String,
    pub quota_rtt: f32,
    pub quota_conges: f32,
    pub quota_urgence_familiale: f32,
    pub conges: Vec<CongeResponseDto>,
}

#[derive(Deserialize)]
pub struct PoserCongeRequestDto {
    pub id_employe: Uuid,
    pub date_debut: NaiveDate,
    pub moment_debut: MomentDebut,
    pub date_fin: NaiveDate,
    pub moment_fin: MomentFin,
    pub type_absence: TypeAbsence,
}

#[derive(Serialize)]
pub struct PoserCongeResponseDto {
    pub id_demande: Uuid,
    pub jours_deduits: f32,
    pub message: String,
}

#[derive(Deserialize)]
pub struct DeclarerHeuresSuppDto {
    pub id_employe: Uuid,
    pub heures: f32,
    pub date: NaiveDate,
    pub choix: ChoixEmploye,
}

#[derive(Serialize)]
pub struct HeuresSuppResponseDto {
    pub id: Uuid,
    pub message: String,
}

#[derive(Serialize)]
pub struct EmployeResponseDto {
    pub id: Uuid,
    pub nom: String,
    pub prenom: String,
    // On peut cacher des infos internes comme le quota si on veut !
}

// --------------------------------------------------------
// Handlers
// --------------------------------------------------------

pub async fn poser_conge_handler(
    State(state): State<AppState>,
    Json(payload): Json<PoserCongeRequestDto>,
) -> impl IntoResponse {
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

    let resultat_service = state
        .abs_service
        .poser_un_conge(payload.id_employe, periode, payload.type_absence)
        .await;

    println!("result svc : {:?}", resultat_service);

    match resultat_service {
        Ok(demande) => {
            println!("OK match res");

            let response = PoserCongeResponseDto {
                id_demande: demande.0.id,
                jours_deduits: demande.1,
                message: "Demande de congé enregistrée avec succès.".to_string(),
            };
            (StatusCode::CREATED, Json(response)).into_response()
        }
        Err(erreur_metier) => {
            println!("PAS OK match res");

            (StatusCode::BAD_REQUEST, erreur_metier.to_string()).into_response()
        }
    }
}

pub async fn lister_employes_handler(State(state): State<AppState>) -> impl IntoResponse {
    match state.abs_service.lister_employes().await {
        Ok(employes) => {
            let dtos: Vec<EmployeResponseDto> = employes
                .into_iter()
                .map(|e| EmployeResponseDto {
                    id: e.id,
                    nom: e.nom,
                    prenom: e.prenom,
                })
                .collect();

            (StatusCode::OK, Json(dtos)).into_response()
        }
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Erreur serveur".to_string(),
        )
            .into_response(),
    }
}

pub async fn lister_employes_by_id_handler(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    match state.abs_service.lister_employes_by_id(id).await {
        Ok(Some((employe, conges))) => {
            let conges_dto = conges
                .into_iter()
                .map(|c| CongeResponseDto {
                    id: c.id,
                    type_absence: c.type_absence,
                    periode: c.periode,
                })
                .collect();

            let response = EmployeDetailResponseDto {
                id: employe.id,
                nom: employe.nom,
                prenom: employe.prenom,
                quota_rtt: employe.quota_rtt,
                quota_conges: employe.quota_conges,
                quota_urgence_familiale: employe.quota_urgence_familiale,
                conges: conges_dto,
            };

            (StatusCode::OK, Json(response)).into_response()
        }
        Ok(None) => (StatusCode::NOT_FOUND, "Employé introuvable").into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Erreur serveur").into_response(),
    }
}

pub async fn declarer_hs_handler(
    State(state): State<AppState>,
    Json(payload): Json<DeclarerHeuresSuppDto>,
) -> impl IntoResponse {
    let hs = HeuresSupplementaires::declarer(
        payload.id_employe,
        payload.heures,
        payload.date,
        payload.choix,
    );
    let id: Uuid = hs.id;

    match state.hs_service.declarer(hs).await {
        Ok(_) => (
            StatusCode::CREATED,
            Json(HeuresSuppResponseDto {
                id: id,
                message: "Déclaration d'heures supplémentaires enregistrée.".to_string(),
            }),
        )
            .into_response(),
        Err(e) => (StatusCode::BAD_REQUEST, e.to_string()).into_response(),
    }
}

pub async fn valider_hs_handler(
    State(state): State<AppState>,
    Path(id_hs): Path<Uuid>,
) -> impl IntoResponse {
    match state.hs_service.valider_et_appliquer(id_hs).await {
        Ok(_) => (
            StatusCode::OK,
            Json(HeuresSuppResponseDto {
                id: id_hs,
                message: "Heures supplémentaires validées et appliquées au solde.".to_string(),
            }),
        )
            .into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

pub async fn lister_hs_by_employe_handler(
    State(state): State<AppState>,
    Path(id_employe): Path<Uuid>,
) -> impl IntoResponse {
    match state.hs_service.lister_par_employe(id_employe).await {
        Ok(liste_hs) => (
            StatusCode::OK,
            Json(liste_hs),
        )
            .into_response(),

        Err(e) => (StatusCode::NOT_FOUND, e.to_string()).into_response(),
    }
}
