use chrono::{Datelike, NaiveDate, Weekday};
use serde::{Deserialize, Serialize};
use crate::core::domain::error::ErreurMetier;


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
        if date_fin < date_debut {
            return Err(ErreurMetier::PeriodeInvalide);
        }

        if date_debut == date_fin
            && moment_debut == MomentDebut::ApresMidi
            && moment_fin == MomentFin::Midi
        {
            return Err(ErreurMetier::PeriodeInvalide);
        }

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

    pub fn calculer_jours_reels(&self, jours_feries: &[NaiveDate]) -> f32 {
        let mut total = 0.0;
        let mut jour_courant = self.date_debut;

        while jour_courant <= self.date_fin {
            let est_week_end = jour_courant.weekday() == Weekday::Sat || jour_courant.weekday() == Weekday::Sun;
            let est_ferie = jours_feries.contains(&jour_courant);

            if !est_week_end && !est_ferie {
                if self.date_debut == self.date_fin {
                    if self.moment_debut == MomentDebut::Matin && self.moment_fin == MomentFin::Soir {
                        total += 1.0;
                    } else {
                        total += 0.5;
                    }
                } else if jour_courant == self.date_debut {
                    total += if self.moment_debut == MomentDebut::Matin { 1.0 } else { 0.5 };
                } else if jour_courant == self.date_fin {
                    total += if self.moment_fin == MomentFin::Soir { 1.0 } else { 0.5 };
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