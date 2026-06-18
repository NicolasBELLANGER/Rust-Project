# Simulation de Collecte de Ressources

## Objectif

Créer une simulation graphique en terminal utilisant **Ratatui** qui simule des robots autonomes collectant des ressources sur une carte générée procéduralement.

---

## Génération de Carte

- Générer une carte avec des **obstacles basés sur du bruit** (Perlin Noise)
- Peupler la carte avec deux types de ressources :
  - Sources d'énergie — symbole `E`
  - Gisements de cristaux — symbole `C`
- Les ressources ont des quantités **aléatoires (50–200 unités)** chacune

---

## Types de Robots

### 🔍 Robots Éclaireurs — `x`

- Explorer la carte de manière aléatoire
- Découvrir et partager les emplacements de ressources
- Éviter les obstacles connus
- ❌ Ne peuvent **pas** collecter de ressources

### 📦 Robots Collecteurs — `o`

- Naviguer vers les emplacements de ressources connus
- Collecter les ressources **une unité à la fois**
- Retourner à la base en portant des ressources
- Décharger les ressources à la base centrale

---

## Système de Base

La base centrale `#` agit comme :

- Point de **départ** pour tous les robots
- Centre de **stockage** des ressources et des connaissances
- Centre de **communication** pour partager les découvertes
- Suivi du total d'énergie et de cristaux collectés

---

## Architecture Concurrente & Gestion des Connaissances

- Chaque robot opère comme une **entité indépendante** avec des connaissances locales limitées
- Les robots commencent **sans information** sur la carte au-delà de leur environnement immédiat
- Le partage d'informations se fait par des **mécanismes de communication asynchrone**

### Comportements distribués clés

| Acteur          | Rôle                                                                      |
| --------------- | ------------------------------------------------------------------------- |
| Éclaireurs      | Diffusent les ressources et obstacles découverts aux autres robots        |
| Collecteurs     | Communiquent les événements de collecte pour mise à jour de l'état global |
| Base            | Coordonne l'agrégation des connaissances de toutes les découvertes        |
| Tous les robots | Synchronisent leurs actions **sans bloquer** les autres                   |

---

## Exigences Techniques

- Utiliser **Ratatui** pour le rendu de l'interface terminal
- Simulation en **temps réel**
- Gérer les entrées utilisateur _(toute pression de touche quitte)_
- Utiliser les fonctionnalités de **concurrence Rust** pour la coordination
- Générer les obstacles via le **bruit de Perlin**

---

## Disposition Visuelle

| Élément     | Symbole | Couleur       |
| ----------- | ------- | ------------- |
| Obstacles   | `O`     | Cyan clair    |
| Énergie     | `E`     | Vert          |
| Cristaux    | `C`     | Magenta clair |
| Base        | `#`     | Vert clair    |
| Éclaireurs  | `x`     | Rouge         |
| Collecteurs | `o`     | Magenta       |

> L'UI affiche en temps réel le **compteur de ressources collectées**.

---

## Critères de Réussite

- [ ] Les robots naviguent de manière autonome et évitent les obstacles
- [ ] Les éclaireurs découvrent et partagent les emplacements de ressources
- [ ] Les collecteurs rassemblent efficacement les ressources et retournent à la base
- [ ] Mises à jour en temps réel du progrès de collecte
- [ ] Rendu terminal propre avec codage couleur approprié

---

## Barème d'Évaluation

### Implémentation de Base — 60 points

| Critère                  | Points | Description                                                         |
| ------------------------ | ------ | ------------------------------------------------------------------- |
| Génération de Carte      | 10     | Génération d'obstacles basée sur le bruit, placement des ressources |
| Comportements des Robots | 20     | Comportements distincts éclaireur/collecteur, pathfinding           |
| Système de Base          | 10     | Stockage des ressources, fonctionnalité de point de départ          |
| Système de Communication | 20     | Passage de messages, partage de connaissances, synchronisation      |

### Qualité Technique — 25 points

| Critère                  | Points | Description                                                 |
| ------------------------ | ------ | ----------------------------------------------------------- |
| Architecture Concurrente | 10     | Entités robotiques indépendantes, opérations non-bloquantes |
| Intégration Ratatui      | 8      | Rendu en temps réel, codage couleur approprié               |
| Qualité du Code          | 7      | Structure propre, gestion d'erreurs, documentation          |

### Fonctionnalités Avancées — 15 points

| Critère                | Points | Description                                                     |
| ---------------------- | ------ | --------------------------------------------------------------- |
| Optimisation           | 5      | Pathfinding efficace, stratégies d'allocation des ressources    |
| Robustesse             | 5      | Cas limites, épuisement des ressources, évitement de collisions |
| Expérience Utilisateur | 5      | Simulation fluide, retour visuel clair                          |

---

**Total : 100 points**
