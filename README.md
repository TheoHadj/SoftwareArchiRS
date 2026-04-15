📅 Système de Gestion des Congés (SIRH) - Architecture Hexagonale
👤 Identification
Apprenant : [Ton Nom / Prénom]

Formation : EPSI Ingénierie 1 EISI

Intervenant GitHub : 3rgo

🎯 Présentation du Projet
Ce projet implémente un système de gestion des congés et des heures supplémentaires. L'objectif principal est de démontrer une séparation stricte entre la logique métier (le "Domaine") et les détails techniques (l'infrastructure), tout en utilisant la puissance du typage de Rust.

Sujet : Gestion des Congés & SIRH
L'application gère les demandes de congés et les déclarations d'heures supplémentaires avec des règles de gestion réelles :

Calcul intelligent : Déduction automatique des week-ends et jours fériés pour obtenir les jours "réels".

Contrôle d'anticipation : Interdiction de poser un congé pour le jour même, sauf motif "Urgence Familiale".

Flux d'approbation : Les heures supplémentaires nécessitent une validation manager avant d'être converties en paiement ou récupération.

🏗️ Architecture & Design Patterns
Le projet suit les principes de l'Architecture Hexagonale (Ports & Adapters) pour garantir que le cœur métier reste indépendant des frameworks et des bases de données.

1. DDD (Domain Driven Design)
Entités (3) : Employee, LeaveRequest, OvertimeDeclaration.

Value Objects (2) :

DateRange : Encapsule la logique complexe de calcul des jours ouvrés.

LeaveType : Gère les invariants liés aux types de congés (RTT, Congé Payé, Urgence).

Invariants métier : La validation des règles est faite au sein du domaine, rendant impossible la création d'un état invalide.

2. Design Patterns implémentés
Repository Pattern : Les accès aux données sont abstraits par des Traits Rust. Cela permet l'interchangeabilité entre une persistance SQLite et un système de Fichiers JSON.

Strategy Pattern : Utilisé pour le calcul des jours fériés (HolidayStrategy), permettant de changer de calendrier (ex: France, International) sans modifier le code de calcul des congés.

3. Tests & Qualité
Stub : Utilisé pour injecter un calendrier fixe dans les tests de calcul de dates.

Mock : Utilisé pour vérifier que le système n'enregistre pas une demande d'heures supplémentaires tant que le choix (Paiement/Récupération) n'est pas validé.

🛠️ Stack Technique
Langage : Rust (Édition 2021)

API REST : Axum (basé sur Tokio)

Persistance : SQLx (SQLite) ou Filesystem (JSON)

Séreilisation : Serde

📂 Structure du Code
Plaintext
src/
├── core/                # L'Hexagone (Logique Pure)
│   ├── domain/          # Entités, Value Objects, Logic
│   └── application/     # Ports (Interfaces) & Cas d'utilisation
├── adapters/            # L'Extérieur (Détails)
│   ├── incoming/        # API REST (Axum Handlers)
│   └── outgoing/        # Persistance (SQLite & JSON)
└── main.rs              # Assemblage (Injection de dépendances)
🚀 Installation
Cloner le dépôt :

Bash
git clone [URL_DU_REPO]
Lancer les tests (Vérification des Mocks/Stubs) :

Bash
cargo test
Démarrer l'API :

Bash
cargo run