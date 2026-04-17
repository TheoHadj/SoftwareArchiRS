use crate::core::domain::error::ErreurMetier;
use chrono::{Datelike, Days, NaiveDate, Weekday};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum TypeAbsence {
    CongePaye,
    RTT,
    UrgenceFamiliale,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum MomentDebut {
    Matin,
    ApresMidi,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum MomentFin {
    Midi,
    Soir,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Periode {
    date_debut: NaiveDate,
    moment_debut: MomentDebut,
    date_fin: NaiveDate,
    moment_fin: MomentFin,
}

impl Periode {
    pub fn nouvelle(
        date_debut: NaiveDate,
        moment_debut: MomentDebut,
        date_fin: NaiveDate,
        moment_fin: MomentFin,
    ) -> Result<Self, ErreurMetier> {
        println!("========================================");
        println!("🔍 DEBUG Periode::nouvelle");
        println!("▶️ Début : {} ({:?})", date_debut, moment_debut);
        println!("⏹️ Fin   : {} ({:?})", date_fin, moment_fin);
        println!(
            "🧮 Test mathématique (début > fin) : {}",
            date_debut > date_fin
        );
        println!("========================================");

        if date_debut > date_fin {
            println!("❌ ERREUR : La date de début est après la date de fin !");
            return Err(ErreurMetier::PeriodeInvalide);
        }

        if date_debut == date_fin
            && moment_debut == MomentDebut::ApresMidi
            && moment_fin == MomentFin::Midi
        {
            println!("❌ ERREUR : Même jour, mais on commence l'aprem pour finir le midi !");
            return Err(ErreurMetier::PeriodeInvalide);
        }

        println!("OKOKOK!");

        Ok(Self {
            date_debut,
            moment_debut,
            date_fin,
            moment_fin,
        })
    }

    pub fn date_debut(&self) -> NaiveDate {
        self.date_debut
    }
    pub fn date_fin(&self) -> NaiveDate {
        self.date_fin
    }

    pub fn calculer_jours_reels(&self, jours_feries: &[NaiveDate]) -> f32 {
        let mut total = 0.0;
        let mut jour_courant = self.date_debut;

        while jour_courant <= self.date_fin {
            let est_week_end =
                jour_courant.weekday() == Weekday::Sat || jour_courant.weekday() == Weekday::Sun;
            let est_ferie = jours_feries.contains(&jour_courant);
            print!("jours courant : {} \n", &jour_courant);
            print!("jours feries liste : {:?} \n", jours_feries);
            print!("In list ? : {:?} \n\n", est_ferie);
            // c moche
            if !est_week_end && !est_ferie {
                if self.date_debut == self.date_fin {
                    if self.moment_debut == MomentDebut::Matin && self.moment_fin == MomentFin::Soir
                    {
                        total += 1.0;
                    } else {
                        total += 0.5;
                    }
                } else if jour_courant == self.date_debut {
                    total += if self.moment_debut == MomentDebut::Matin {
                        1.0
                    } else {
                        0.5
                    };
                } else if jour_courant == self.date_fin {
                    total += if self.moment_fin == MomentFin::Soir {
                        1.0
                    } else {
                        0.5
                    };
                } else {
                    total += 1.0;
                }
            }

            if let Some(jour_suivant) = jour_courant.succ_opt() {
                jour_courant = jour_suivant;
            } else {
                break;
            }
        }
        total
    }
}

// Algo de calcul de paques pour pas faire de requete sur datagouv
fn calculer_paques(annee: i32) -> NaiveDate {
    let a = annee % 19;
    let b = annee / 100;
    let c = annee % 100;
    let d = b / 4;
    let e = b % 4;
    let f = (b + 8) / 25;
    let g = (b - f + 1) / 3;
    let h = (19 * a + b - d - g + 15) % 30;
    let i = c / 4;
    let k = c % 4;
    let l = (32 + 2 * e + 2 * i - h - k) % 7;
    let m = (a + 11 * h + 22 * l) / 451;
    let mois = (h + l - 7 * m + 114) / 31;
    let jour = ((h + l - 7 * m + 114) % 31) + 1;

    NaiveDate::from_ymd_opt(annee, mois as u32, jour as u32).unwrap()
}

pub fn generer_jours_feries_france(annee: i32) -> Vec<NaiveDate> {
    let mut jours = vec![
        NaiveDate::from_ymd_opt(annee, 1, 1).unwrap(),
        NaiveDate::from_ymd_opt(annee, 5, 1).unwrap(),
        NaiveDate::from_ymd_opt(annee, 5, 8).unwrap(),
        NaiveDate::from_ymd_opt(annee, 7, 14).unwrap(),
        NaiveDate::from_ymd_opt(annee, 8, 15).unwrap(),
        NaiveDate::from_ymd_opt(annee, 11, 1).unwrap(),
        NaiveDate::from_ymd_opt(annee, 11, 11).unwrap(),
        NaiveDate::from_ymd_opt(annee, 12, 25).unwrap(),
    ];

    let paques = calculer_paques(annee);
    jours.push(paques.checked_add_days(Days::new(1)).unwrap());
    jours.push(paques.checked_add_days(Days::new(39)).unwrap());
    jours.push(paques.checked_add_days(Days::new(50)).unwrap());

    jours
}
