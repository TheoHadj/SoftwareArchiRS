# 📅 Système de Gestion des Congés (SIRH)

![Rust](https://img.shields.io/badge/language-Rust-orange.svg)
![Axum](https://img.shields.io/badge/framework-Axum-blue.svg)
![Architecture](https://img.shields.io/badge/architecture-Hexagonal-green.svg)
![License](https://img.shields.io/badge/license-MIT-lightgrey.svg)

## 👤 Identification
- **Apprenant** : [Ton Nom / Prénom]
- **Formation** : EPSI Ingénierie 1 EISI
- **Intervenant GitHub** : `3rgo`

---

## 🎯 Présentation du Projet
Ce projet est une **API REST** de gestion de ressources humaines (SIRH) spécialisée dans les congés et les heures supplémentaires. Il a été conçu pour mettre en pratique les principes de la **Clean Architecture** et du **Domain Driven Design (DDD)** en Rust.

### Règles Métier Implémentées (Sujet E)
- **Calcul des jours ouvrés** : Déduction automatique des week-ends et jours fériés lors de la pose d'un congé.
- **Urgence Familiale** : Dérogation aux règles d'anticipation habituelles pour les motifs impérieux.
- **Gestion des Heures Sup'** : Système de choix entre paiement ou récupération, soumis à validation managériale.

---

## 🏗️ Architecture Technique
L'application repose sur une **Architecture Hexagonale (Ports & Adapters)** permettant une isolation totale du domaine métier.

### 1. Domain Driven Design (DDD)
Conformément au barème, le domaine contient :
- **3 Entités** : `Employee`, `LeaveRequest`, `OvertimeDeclaration`.
- **2 Value Objects** : 
    - `DateRange` : Responsable de la cohérence temporelle et du calcul des jours réels.
    - `LeaveType` : Enumération riche portant les règles spécifiques à chaque type d'absence.

### 2. Design Patterns
- **Repository Pattern** : Abstraction de la persistance via des `Traits`. La base est **interchangeable** entre **SQLite** (via SQLx) et un stockage **Fichier JSON**.
- **Strategy Pattern** : Injection de la logique de calendrier (`HolidayStrategy`) pour le calcul des jours fériés.

### 3. Testabilité (Mocks & Stubs)
- **Stub** : Simulation d'un calendrier fixe pour garantir la répétabilité des tests de durée.
- **Mock** : Vérification du comportement des services d'infrastructure lors de la validation des heures supplémentaires.

---

## 💻 Aperçu du Code (Rust)

### Modélisation du Domaine (Value Object)
```rust
pub struct DateRange {
    start: NativeDate,
    end: NativeDate,
}

impl DateRange {
    pub fn new(start: NativeDate, end: NativeDate) -> Result<Self, DomainError> {
        if end < start { return Err(DomainError::InvalidPeriod); }
        Ok(Self { start, end })
    }

    // Calcul métier pur : ignore les week-ends
    pub fn working_days(&self) -> u32 {
        self.start.iter_days()
            .take_while(|&d| d <= self.end)
            .filter(|&d| d.weekday().number_from_monday() <= 5)
            .count() as u32
    }
}```