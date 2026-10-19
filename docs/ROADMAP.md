# Roadmap

This project implements the board game **Splendor** in Rust, as a learning project.
The long-term goal is a small board game platform (in the spirit of Board Game Arena)
playable on web and mobile, in multiplayer and/or against AI players.

## Vision

| Horizon | Target |
|---|---|
| **MVP** | Local CLI game of base Splendor (2–4 players) against simple AI players (random, greedy). |
| Mid term | A second, small game (**6 nimmt!**) to validate the generic model, then web app (solo vs AI in the browser, offline-capable PWA), real-time online multiplayer, and stronger AI (ISMCTS). |
| Long term | Desktop/mobile apps (Tauri), async play, external bots, a medium card game (**Gang of Four**), and a much larger game (**Agricola**) that drives the extraction of a generic platform. |

The order favours a **vertical slice** (engine → CLI → web → server) over AI strength:
integration (WASM boundary, player views, game records, network sync) is where the
architecture can prove wrong, while stronger AIs plug in later through the `Agent` trait
without changing the web or server code.

### MVP functional scope

- Full base rules of Splendor, 2–4 players (90 development cards, 10 nobles, reservations,
  gold, 10-token limit, end of game at 15 prestige points with final round and tie-breaker).
  Clarifications and deliberate deviations are listed in [rules/splendor.md](rules/splendor.md).
- Any mix of human (hot-seat) and AI players.
- AI levels: **random** and **greedy/heuristic** (ISMCTS comes after the online phases).
- AIs are fair: they only see what a human in their seat would see.
- Headless AI-vs-AI simulation of many games (minimal: robustness check and basic
  results; detailed statistics come with the arena in phase 10).
- Save / resume a game in progress.
- Replay a finished game step by step.
- Plain text CLI. No TUI is planned: the web app is the rich interface.

## Guiding principles

1. **Pure engine**: no I/O, no threads, no clock. Randomness comes from a seeded RNG,
   with our own shuffle so that replays survive dependency upgrades.
   The engine compiles to WASM (checked in CI from day one) and is testable, replayable
   and fast enough for search AIs. AIs follow the same rules: search budgets are
   iterations or a caller-provided clock.
   See [ADR 0003](adr/0003-pure-deterministic-engine.md) and [ADR 0008](adr/0008-reproducible-randomness.md).
2. **Event-sourced games**: a game is `config + seed + action log`. Save, replay,
   multiplayer sync and cheat checks all rely on it. See [ADR 0004](adr/0004-event-sourced-game-records.md).
3. **Player views**: players (human or AI) only ever receive a `PlayerView`, never the
   full state. See [ADR 0006](adr/0006-hidden-information-and-player-views.md).
4. **"Who must decide what"** rather than "whose turn is it", to be ready for games with
   multi-step turns, interrupts and simultaneous decisions. A player asked to decide
   always has at least one legal action. See [ADR 0005](adr/0005-game-trait-and-decision-model.md).
5. **Minimal abstraction now, the rest when the second game arrives.**

## Target architecture

```
Splendor/                 Cargo workspace
├── crates/
│   ├── game-core/        Game trait, PlayerId, Status, Outcome, GameRecord, RNG helpers
│   ├── splendor/         Splendor rules, card data, player views
│   ├── six-nimmt/        (phase 7) 6 nimmt! rules, second game validating the model
│   ├── gang-of-four/     (phase 11) Gang of Four rules
│   ├── ai/               Agent trait + Random, Greedy, ISMCTS (generic over Game)
│   ├── cli/              binary: play, hot-seat, save/load, replay, simulator
│   ├── splendor-wasm/    (later) wasm-bindgen bindings
│   └── server/           (later) axum + WebSocket + sqlx
├── web/                  (later) React + TypeScript + Vite, PWA, then Tauri
└── docs/
    ├── ROADMAP.md
    ├── adr/              Architecture Decision Records
    └── rules/            rules notes and clarifications
```

See [ADR 0002](adr/0002-cargo-workspace-layout.md) and [ADR 0007](adr/0007-frontend-stack.md).

## Phases

Sizes are relative: **S** = 1–2 sessions, **M** = 3–5 sessions, **L** = 6+ sessions.

| # | Phase | Content | Size | Rust topics |
|---|---|---|---|---|
| 0 | Foundations | Cargo workspace with empty crates, CI (fmt, clippy, tests, `wasm32` check) | S | Cargo, workspaces, tooling |
| 1 | Splendor model | Reproducible RNG helpers with golden tests (ADR 0008), types (gems, tokens, cards, nobles), data for 90 cards and 10 nobles, setup per player count | M | enums, structs, newtypes, traits, serde, unit tests |
| 2 | Rules | Legal actions, apply, follow-up decisions (discard, noble choice), pass, end of game, player views; settle open entries of [rules/splendor.md](rules/splendor.md) | L | `Result`, error handling, traits, ownership |
| 3 | **Playable CLI** 🎯 | Text rendering, move input, hot-seat, random agent | M | I/O, parsing, clap |
| 4 | Invariants + minimal simulator | proptest over thousands of random games: token conservation, no illegal action accepted, never an empty action list, games always terminate…; CLI command to run N headless AI-vs-AI games | S | property-based testing |
| 5 | Game records | Save/resume and replay through a versioned JSON `GameRecord` (also the basis of the server in phase 9) | S | serialization, format versioning |
| 6 | **Greedy AI** 🏁 | Heuristic agent (buy the best affordable card, otherwise take tokens towards a target card…), checked against random with the minimal simulator | S | traits, scoring functions |
| | *MVP complete* | | | |
| 7 | Second game: 6 nimmt! | Rules of *6 nimmt!* in its own crate, playable through the existing generic CLI, simulator, records and random agent (plus a simple greedy agent); fix the `Game` trait where it does not fit and confirm ADR 0005 | M | generics, trait design |
| 8 | Web solo | `splendor-wasm`, React + TS front, AI in a Web Worker, offline PWA; game selection from the start; confirm ADR 0007 | L | wasm-bindgen, FFI boundary |
| 9 | Real-time server | axum, WebSocket rooms, reconnection, SQLite then Postgres, simple accounts; multi-game from the start | L | async, tokio, shared state, sqlx |
| 10 | Arena + ISMCTS | Parallel N-game arena with statistics, criterion benchmarks, information-set MCTS with determinization from the player view and iteration budget, measured against greedy; optimize allocations if benchmarks require it (ADR 0005) | L | rayon, criterion, generics, performance |
| 11 | Gang of Four | Third game, added end to end (engine, web UI, online play) on the existing platform; ISMCTS with determinizations consistent with private knowledge (cards received in the exchange) | M | combinatorics, algorithms, performance |
| 12 | Advanced multiplayer | Async (turn-based) games with notifications, AI replacement of disconnected players, JSON protocol for external bots | M | API design |
| 13 | Apps | Tauri 2 packaging (desktop, then mobile); Capacitor as fallback | M | Tauri, cross-compilation |
| 14 | Agricola | Fourth game, then extraction of platform abstractions (lobby, rendering, game options) | XL | effect systems, large-scale design |

🎯 first playable milestone (play against a random AI) — 🏁 MVP.

The playable CLI comes before property-based tests so that the game can be played as
early as possible; unit tests written during phases 1–2 cover the rules in the meantime.

A second game comes right after the MVP, before the web and server layers are built on
the `Game` trait. Splendor only exercises part of the decision model (ADR 0005); *6 nimmt!*
is small but covers what Splendor does not:

| Model aspect | Splendor | 6 nimmt! |
|---|---|---|
| Simultaneous decisions (`AwaitingDecision` with several players) | no, always one player | yes, everyone picks a card secretly |
| Follow-up decision by a player other than the last one to act | no | yes, the owner of a too-low card picks a row during resolution |
| Randomness during the game | no, everything shuffled at setup | yes, new deal every round |
| Hidden hand for every player | partial (reserved cards) | yes |
| Player count | 2–4 | 2–10 |

Fixing the trait at this point is cheap: only the CLI, the simulator and the simple agents
depend on it.

*Gang of Four* (phase 11) adds little to the decision model, which *6 nimmt!* already
validates, but tests two other things:

- **The platform**: once the web app and server exist, adding a complete game (engine,
  UI, online play) should be cheap. Measuring that on a medium game is safer than
  discovering it with Agricola.
- **The AI**: a combinatorial action space (singles, pairs, straights, full houses,
  flushes, "gangs" of up to 7 cards), a trick structure (lead, beat or pass), and private
  knowledge that accumulates (after the card exchange, the loser knows the winner holds
  their best card), so ISMCTS must sample hands consistent with what the player knows,
  not merely shuffle the unknown cards.

A greedy AI is kept in the MVP because a random opponent makes solo play pointless once
the rules are known; it is cheap and makes solo play credible on the web too.

Known trade-off of this order: if ISMCTS benchmarks (phase 10) require changing the `Game`
trait for performance (see ADR 0005), the web and server code built in phases 8–9 will
have to follow. The risk is limited because both use the trait through generic code.

### Pace

At a few hours per week, the MVP (phases 0–6) represents roughly 15–25 sessions.
Phase 3 is the first moment the game can actually be played; everything before it should
stay as lean as possible. Phase 7 (second game) is medium; phases 8 and 9 are each large:
expect several months between the MVP and online play.

## Open questions

These will be settled (and recorded as ADRs or in [rules/splendor.md](rules/splendor.md))
when the corresponding phase starts.

- **Rules clarifications**: taking fewer than 3 gems, all players passing in a row —
  see [rules/splendor.md](rules/splendor.md). Payment is automatic (deliberate deviation,
  documented there).
- **Card data**: the 90 cards and 10 nobles must be checked against the official list;
  a test will verify totals per level.
- **Licensing**: the code is MIT, but Splendor, 6 nimmt!, Gang of Four and Agricola
  (rules, card data, artwork) belong to their publishers. The public repository contains
  card data; this is fine for a personal learning project, but a public release of the
  games would require a license from the publishers.

## Way of working

- Small, reviewable steps: each phase is split into focused commits.
- Each step comes with an explanation of the Rust concepts involved and the reasons behind
  design choices.
- Significant decisions are recorded in [docs/adr](adr/); ADRs stay *Proposed* while
  their details are still expected to change (see [ADR 0001](adr/0001-record-architecture-decisions.md)).
