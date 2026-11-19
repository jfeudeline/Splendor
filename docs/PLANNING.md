# Planning d'avancement

Dernière mise à jour : 2026-11-19  
Dernier commit connu : `7c46ec3 Add catalog dump example`

## Règle de suivi

- Une **séance** = environ 1,5 à 2 heures : un petit pas, des tests, un commit.
- En fin de séance, noter :
  - la durée réelle ;
  - le commit de début et le commit de fin ;
  - l'étape terminée ;
  - les blocages, détours et décisions.
- Ne jamais inventer une durée : si elle n'est pas donnée, écrire `à renseigner`.
- Recalculer la projection après quelques séances, pas à chaque petite modification.

## État actuel

- Phase 0 : terminée.
- Phase 1 : en cours, environ 70 %.
- Phase 2 : pas commencée.

Déjà fait :

- [x] `Gem`, `Level`
- [x] `Token`
- [x] `TokenCounts`
- [x] `GemCounts`
- [x] `Card`, `CardId`
- [x] `Noble`, `NobleId`
- [x] RNG reproductible (`seed`, `below`, `shuffle`, golden tests)
- [x] Catalogue RON (90 cartes, 10 nobles) + chargeur + tests

Reste en phase 1 :

- [ ] `Config` + `PlayerCount`
- [ ] `PlayerId` + `Player`
- [ ] `Bank`, `Market`, `Decks`
- [ ] Setup 2–4 joueurs
- [ ] Tests de setup et de déterminisme

## Planning prévisionnel

| Phase | Contenu | Séances estimées | Statut | Séances réelles |
|---|---|---:|---|---:|
| Fin phase 1 | Config, Player, Bank, Market, Decks, setup, tests | 3–5 | en cours | 0 |
| Phase 2 | `Game`/`Decision`, actions légales, `apply`, défausse, nobles, fin de partie, vues | 10–15 | à faire | 0 |
| Phase 3 | CLI jouable : rendu, saisie, hot-seat, agent aléatoire | 5–7 | à faire | 0 |
| Phase 4 | Invariants, simulateur, benchmarks | 3–4 | à faire | 0 |
| Phase 5 | Records : sauvegarde, reprise, replay | 2–3 | à faire | 0 |
| Phase 6 | IA greedy | 3–4 | à faire | 0 |

### Jalons

| Jalon | Séances cumulées estimées |
|---|---:|
| Fin phase 1 | 3–5 |
| Premier jeu jouable (fin phase 3) | 18–27 |
| MVP (fin phase 6) | 26–38 |
| MVP avec marge de 10–20 % | 29–46 |

## Détail des prochaines séances

| # | Étape | Objectif | Séances estimées |
|---:|---|---|---:|
| 1 | `Config` + `PlayerCount` | 2–4 joueurs, validation, tests | 1 |
| 2 | `PlayerId` + `Player` | jetons, bonus, points, cartes achetées/réservées | 1–2 |
| 3 | `Bank`, `Market`, `Decks` | compteurs, marché 4×3, decks 40/30/20 | 1–2 |
| 4 | Setup | mélange déterministe, distribution, nobles, premier joueur | 1–2 |
| 5 | Tests de setup | conservation des jetons, counts, reproductibilité 2/3/4 joueurs | 1 |

## Journal des séances

Modèle de ligne :

```text
| N | Date | Durée | Commit début | Commit fin | Étape terminée | Notes |
```

| # | Date | Durée | Commit début | Commit fin | Étape terminée | Notes |
|---:|---|---|---|---|---|---|
| 1 | 2026-11-19 | à renseigner |  | `7c46ec3` | Catalogue + tests + planning | À compléter |
| 2 |  |  |  |  |  |  |
| 3 |  |  |  |  |  |  |
| 4 |  |  |  |  |  |  |
| 5 |  |  |  |  |  |  |

## Projection après quelques séances

Quand 3 à 5 séances sont renseignées :

1. comparer les séances réelles aux séances estimées pour les étapes terminées ;
2. calculer un ratio :

```text
ratio = séances réelles / séances estimées
projection restante = séances restantes estimées × ratio
```

3. ajuster la colonne « Séances estimées » du tableau principal ;
4. garder une marge de 10 à 20 %.

## Rappel de fin de séance

À chaque fin de séance, donner :

- la durée totale ;
- le commit de début et le commit de fin ;
- l'étape terminée ;
- les blocages ou détours ;
- ce qui reste pour la prochaine séance.

L'assistant doit demander ces informations et mettre ce fichier à jour.
