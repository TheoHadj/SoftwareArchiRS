# 🚀 Projet SIRH - Architecture Hexagonale en Rust

Ce projet est une API backend robuste développée en **Rust**, destinée à la gestion des Ressources Humaines (SIRH). Il permet de gérer les employés, leurs quotas de congés, ainsi que la déclaration et la conversion des heures supplémentaires en RTT.

Il a été conçu en respectant les principes de l'**Architecture Hexagonale** (Ports et Adaptateurs), garantissant un code testable, évolutif et indépendant des technologies d'infrastructure.

Dislcaimer : Rust n'a pas à proprement parler de classe : les données et les méthodes sont strictement séparées : Les structs et leur implémentation, ce qui suit exactement les recommandations du DDD.

---

## 🏗️ Architecture du Projet

Le code est organisé en couches. La règle d'or est que les dépendances pointent toujours vers l'intérieur (le domaine métier), qui reste pur et isolé de la technique.

### 1. Le Cœur (Core / Domaine)
C'est le centre de l'application, agnostique du web ou des bases de données.
* **Domain (`core::domain`)** : Contient les entités pures (`Employe`, `HeuresSupplementaires`) et les erreurs métier (`ErreurMetier`).
* **Application (`core::application`)** : Coordonne les cas d'utilisation.
    * **Ports** : Traits Rust définissant les contrats que l'infrastructure doit remplir (`EmployeeRepository`, `HeuresSuppRepository`).
    * **Services** : Orchestrent la logique (`AbsenceService`, `HeuresSuppService`).

### 2. Les Adaptateurs (Adapters / Infrastructure)
Ils font le pont entre le monde extérieur et le métier.
* **Incoming (HTTP)** : API REST avec le framework **Axum**. Réceptionne le JSON, appelle les Services, et renvoie les réponses formatées.
* **Outgoing (Persistance)** : 
    * **SQLite** : Persistance réelle avec **SQLx**. Gère le schéma, les jointures et les `UPSERT`.
    * **Mock** : Base de données en mémoire via des `HashMap` protégées par des `Arc<Mutex<...>>` pour le développement et les tests. Utilisable avec des arguments au run

---

## ✨ Fonctionnalités implémentées

* **Profils Employés** : Consultation des quotas (Congés, RTT, Urgences familiales).
* **Heures Supplémentaires** : 
    * Déclaration (Paiement ou Récupération).
    * Validation par le manager.
    * **Logique Métier** : Conversion automatique des heures validées en jours de RTT (7h = 1j) ajoutés au solde de l'employé.

---

## 🛠️ Installation et Lancement

### 1. Préparation
L'application crée automatiquement une base SQLite `mon_sirh.db` au premier lancement avec un employé de test (**Jean Dupont**).

### 2. Lancement (Mode SQLite par défaut)
```bash
cargo run
```

### 3. Lancement (Mode Mock)
```bash
# Windows (PowerShell)
$env:USE_MOCK="true"; cargo run

# Linux / MacOS
USE_MOCK=true cargo run
```

---

## 🧪 Exemples de Test (API)

### Fichiers de test recommandés
Créer ces fichiers à la racine de ton projet pour simplifier les appels `curl`.

**test_hs.json**
```json
{
    "id_employe": "f098ab96-3799-4481-b0d5-d0c3bfe509b0",
    "heures": 7.0,
    "date": "2026-04-17",
    "choix": "Recuperation"
}
```

**test_conge.json**
```json
{
  "id_employe": "f098ab96-3799-4481-b0d5-d0c3bfe509b0",
  "date_debut": "2026-05-1",
  "moment_debut": "Matin",
  "date_fin": "2026-05-8",
  "moment_fin": "Soir",
  "type_absence": "CongePaye"
}
```

### Commandes de test (Scénario complet)

1. **Vérifier l'état de l'employé :**
   ```bash
   curl -X GET http://127.0.0.1:3000/employes/f098ab96-3799-4481-b0d5-d0c3bfe509b0
   ```

2. **Déclarer des Heures Supplémentaires :**
   ```bash
   curl -X POST http://127.0.0.1:3000/heures-supp -H "Content-Type: application/json" -d @test_hs.json
   ```
   *(Note : Copie l'ID `"id":"..."` reçu dans la réponse).*

3. **Valider les Heures (Manager) :**
   ```bash
   # Remplace <ID_HS> par l'ID reçu à l'étape précédente
   curl -X POST http://127.0.0.1:3000/heures-supp/<ID_HS>/valider
   ```

4. **Vérifier le gain de RTT :**
   ```bash
   curl -X GET http://127.0.0.1:3000/employes/f098ab96-3799-4481-b0d5-d0c3bfe509b0
   ```
   *(Le champ `quota_rtt` doit être passé de 0.0 à 1.0).*

5. **Lister toutes les déclarations de l'employé :**
   ```bash
   curl -X GET http://127.0.0.1:3000/employes/f098ab96-3799-4481-b0d5-d0c3bfe509b0/heures-supp
   ```



   ## 1. Architecture Clean Code
Visualisation de la structure des dossiers et de la séparation des responsabilités.

```mermaid
graph TD
    %% Dossier Racine
    Root[src/] --> Main[main.rs - Point d'entrée]
    Root --> Domain[domain/ - Coeur métier pur]
    Root --> App[application/ - Cas d'utilisation]
    Root --> Infra[infrastructure/ - Détails techniques]
    Root --> Pres[presentation/ - API REST Axum]

    %% Domaine
    Domain --> Entities[entities/ - Employee, LeaveRequest, Overtime]
    Domain --> VOs[value_objects/ - DateRange, LeaveType]
    Domain --> Repos[repositories/ - Traits/Interfaces des repos]
    Domain --> Services[services/ - Logique de calcul pure]

    %% Application
    App --> UseCases[services/ - Orchestration des actions]

    %% Infrastructure
    Infra --> Persistence[persistence/]
    Persistence --> SQLite[sqlite/ - Implémentation SQLx]
    Persistence --> FileSys[file/ - Implémentation JSON/Fichier]

    %% Présentation
    Pres --> Handlers[handlers/ - Contrôleurs Axum]
    Pres --> DTOs[dtos/ - Objets JSON Input/Output]

    style Domain fill:#ddf9f,stroke:#333,stroke-width:2px
    style App fill:#ddbbf,stroke:#333,stroke-width:1px
    style Infra fill:#dddfd,stroke:#333,stroke-width:1px
    style Pres fill:#ddffd,stroke:#333,stroke-width:1px
```

## 2. Architecture Hexagonale (Ports & Adaptateurs)
Représentation des flux entre les adaptateurs et le cœur de l'application.

```mermaid
graph LR
    subgraph "Adaptateurs Entrants (Web)"
        API["Axum Handler\n(POST /conges)"]
        DTO["PoserCongeRequestDto"]
    end

    subgraph "Cœur de l'Application (L'Hexagone)"
        subgraph "Application"
            Service["LeaveService\n(Use Case)"]
            PortLeave{"« Port »\nLeaveRepository"}
            PortEmp{"« Port »\nEmployeeRepository"}
        end
        
        subgraph "Domaine"
            Entites["Entités\n(DemandeConge, Employe)"]
            VO["Objets de Valeur\n(Periode, Moment)"]
        end
    end

    subgraph "Adaptateurs Sortants (Persistance)"
        Mock["MockRepository\n(En mémoire)"]
        SQLite["SqliteRepository\n(SQLx)"]
    end

    %% Flux d'exécution (Entrant)
    API -->|Valide DTO et appelle| Service
    
    %% Règle de dépendance du DDD
    Service -->|Orchestre| Entites
    Entites -->|Composé de| VO

    %% Injection de dépendances (Ports et Adaptateurs)
    Service -->|Utilise| PortLeave
    Service -->|Utilise| PortEmp
    
    Mock -.->|Implémente| PortEmp
    Mock -.->|Implémente| PortLeave
    SQLite -.->|Implémente| PortLeave

    classDef core fill:#e1f5fe,stroke:#01579b,stroke-width:2px;
    classDef adapter fill:#fff3e0,stroke:#e65100,stroke-width:2px;
    class Service,PortLeave,PortEmp,Entites,VO core;
    class API,DTO,Mock,SQLite adapter;
```

## 3. Diagramme de Classes
Détail des entités et des objets de valeur.

```mermaid
classDiagram
    %% Entités Principales
    class Employe {
        +id: Uuid
        +nom: String
        +prenom: String
        +quota_urgence_familiale: f32
        +quota_conges: f32
        +quota_rtt: f32
        +consommer_quota_urgence(): Result
        +modifier_quota(jours: f32, type_absence: TypeAbsence, ajout: bool)
    }

    class DemandeConge {
        +id: Uuid
        +poser(employe: Employe, periode: Periode, type_absence: TypeAbsence, aujourd_hui: NaiveDate): Result$
    }

    class HeuresSupplementaires {
        +id: Uuid
        +heures: f32
        +date: NaiveDate
        +validation_manager: bool
        +declarer(id_employe: Uuid, heures: f32, date: NaiveDate, choix: ChoixEmploye): HeuresSupplementaires$
        +valider_manager_direct()
    }

    %% Objets de Valeur et Énumérations
    class ChoixEmploye {
        <<enumeration>>
        Paiement
        Recuperation
    }

    class TypeAbsence {
        <<enumeration>>
        CongePaye
        RTT
        UrgenceFamiliale
    }

    class Periode {
        <<Value Object>>
        +date_debut: NaiveDate
        +date_fin: NaiveDate
        +calculer_jours_reels(): f32
    }

    %% Relations Structurelles (Agrégation et Composition)
    DemandeConge *-- Periode : contient
    
    %% Relations d'Association avec Cardinalités
    Employe "1" --> "*" DemandeConge : effectue
    Employe "1" --> "*" HeuresSupplementaires : déclare
    
    DemandeConge "*" --> "1" TypeAbsence : est de type
    HeuresSupplementaires "*" --> "1" ChoixEmploye : implique
    
    %% Relations de Dépendance (Utilisation temporaire dans les méthodes)
    Employe ..> TypeAbsence : utilise pour maj quota
```

## 4. Diagramme de Séquence (Poser un congé)
Flux d'exécution complet lors d'une requête HTTP.

```mermaid
sequenceDiagram
    actor Client HTTP
    participant Axum as Handler (Axum)
    participant Service as LeaveService
    participant Domaine as Entités (Domaine)
    participant Repo as BDD (Mock / SQLite)

    Client HTTP->>Axum: POST /conges (JSON)
    activate Axum
    
    Axum->>Axum: Parse le DTO
    Axum->>Domaine: Periode::nouvelle(...)
    Domaine-->>Axum: Ok(Periode) ou Erreur
    
    Axum->>Service: poser_un_conge(id, Periode, type)
    activate Service
    
    Service->>Repo: find_by_id(id_employe)
    Repo-->>Service: Retourne l'Employé
    
    Service->>Domaine: DemandeConge::poser(Employe, Periode, ...)
    activate Domaine
    Note over Domaine: Vérification (Anticipation,<br/>Quota d'urgence)
    Domaine-->>Service: Retourne la DemandeConge
    deactivate Domaine
    
    Service->>Repo: save(Employe) (Mise à jour quota)
    Service->>Repo: save(DemandeConge)
    Repo-->>Service: Ok()
    
    Service-->>Axum: Ok(DemandeConge)
    deactivate Service
    
    Note over Axum: Calcul des jours à déduire<br/>pour l'affichage
    Axum->>Domaine: periode.calculer_jours_reels()
    Domaine-->>Axum: f32 (ex: 2.5 jours)
    
    Axum-->>Client HTTP: 201 Created (JSON Response)
    deactivate Axum
```

***