# 0002. Cargo workspace layout

- Status: Accepted
- Date: 2026-10-15

## Context

The project will contain a game engine, AI players, a CLI, and later WASM bindings, a
server and a web front-end. A second game (Agricola) is planned. These parts have
different dependencies and compilation targets.

## Decision

Use a single Cargo workspace with one crate per responsibility, under `crates/`:

| Crate | Kind | Responsibility | Depends on |
|---|---|---|---|
| `game-core` | lib | `Game` trait, `PlayerId`, `Status`, `Outcome`, `GameRecord` | — |
| `splendor` | lib | Splendor rules, card data, player views | `game-core` |
| `ai` | lib | `Agent` trait, Random / Greedy / ISMCTS agents, generic over `Game` | `game-core` |
| `cli` | bin | Play, hot-seat, save/load, replay, simulation | all of the above |
| `splendor-wasm` | lib (later) | wasm-bindgen bindings for the web front | `splendor`, `ai` |
| `server` | bin (later) | axum + WebSocket multiplayer server | `game-core`, `splendor`, `ai` |

The web front-end lives in `web/`, outside the Cargo workspace.

Game-specific AI heuristics (e.g. Splendor greedy scoring) may live in the game crate or
in a dedicated module of `ai` behind the game's types; this will be decided in phase 6.

## Alternatives considered

- **Single crate with modules**: simpler at first, but mixes targets (WASM vs native,
  binary vs library) and makes dependency boundaries implicit.
- **One crate per game including its AI and UI**: does not let generic AIs be reused
  across games.

## Consequences

- Dependency direction is enforced by Cargo: the engine cannot depend on the CLI or I/O.
- Adding a game means adding a crate that implements `game-core::Game`.
- Shared dependency versions are declared once in the workspace `Cargo.toml`.
