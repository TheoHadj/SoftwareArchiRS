# Système de Gestion des Congés (SIRH) - Architecture Logicielle

## 👤 Identification
- **Apprenant** : [Votre Nom / Prénom]
- [cite_start]**Formation** : EPSI Ingénierie 1 EISI [cite: 1]
- [cite_start]**Intervenant à ajouter sur GitHub** : `3rgo` [cite: 27]

---

## 🎯 Sujet et Contexte
[cite_start]Ce projet a été réalisé dans le cadre du module **Architectures Applicatives**[cite: 1]. [cite_start]Le but est de mettre en pratique les concepts de **Domain Driven Design (DDD)** et de **Clean Architecture** à travers la création d'une API REST robuste[cite: 3, 7, 12].

### [cite_start]Sujet Choisi : Sujet E - Système de Gestion des Congés (SIRH) [cite: 68]
L'application permet aux employés de gérer leurs absences et heures supplémentaires avec des règles métier strictes :
- [cite_start]**Calcul des jours réels** : Déduction automatique des week-ends et jours fériés[cite: 72].
- [cite_start]**Urgences Familiales** : Seul motif permettant une pose de congé le jour même[cite: 73].
- [cite_start]**Flux d'approbation** : Validation par le manager pour les heures supplémentaires (paiement ou récupération)[cite: 74].

---

## 🛠 Contraintes Techniques Respectées
[cite_start]Conformément au barème de notation[cite: 34, 35]:

### [cite_start]1. Coeur Métier - DDD [cite: 12]
Le domaine est isolé de toute dépendance technique et contient :
- [cite_start]**3 Entités** : `Employee`, `LeaveRequest`, `OvertimeDeclaration`[cite: 17].
- [cite_start]**2 Value Objects** : `DateRange` (logique de calcul des jours ouvrés) et `LeaveType`[cite: 18].

### [cite_start]2. Design Patterns (hors MVC) [cite: 20]
- [cite_start]**Repository Pattern** : Utilisation de Traits Rust pour rendre la persistance interchangeable entre **SQLite** et un système de **Fichiers**[cite: 10].
- **Strategy Pattern** : Pour le calcul dynamique des jours fériés (permettant d'injecter différentes règles de calendrier).

### [cite_start]3. Testabilité [cite: 21, 22]
- **Stub** : Utilisé pour simuler le calendrier des jours fériés dans les tests de calcul de durée.
- **Mock** : Utilisé pour vérifier que le `OvertimeRepository` n'est appelé que si les règles de validation manager sont satisfaites.

### [cite_start]4. Architecture Globale & SoC [cite: 11]
L'application suit une **Architecture Hexagonale** :
- `domain/` : Logique pure, entités et interfaces (Traits).
- `application/` : Cas d'utilisation (Services).
- `infrastructure/` : Implémentations concrètes (SQLite via SQLx, File System).
- [cite_start]`presentation/` : API REST (Axum)[cite: 7].

---

## 🚀 Technologies utilisées
- [cite_start]**Langage** : Rust (pour la sûreté du typage et la gestion des états)[cite: 23].
- **Framework Web** : Axum.
- [cite_start]**Persistance** : SQLite (via SQLx) ou Fichiers JSON (interchangeable)[cite: 10, 23].
- **Validation** : Serde & Validator.

---

## 📂 Installation et Lancement

### Prérequis
- Rust & Cargo installés.
- SQLite (optionnel, géré par le driver).

### Lancer l'application
```bash
cargo run