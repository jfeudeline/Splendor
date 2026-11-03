# Architecture Decision Records

See [0001](0001-record-architecture-decisions.md) for the process and [template.md](template.md) for the format.

| # | Decision | Status |
|---|---|---|
| [0001](0001-record-architecture-decisions.md) | Record architecture decisions | Accepted |
| [0002](0002-cargo-workspace-layout.md) | Cargo workspace layout | Accepted (amended by [0011](0011-multi-game-boundary.md)) |
| [0003](0003-pure-deterministic-engine.md) | Pure, deterministic, WASM-compatible engine | Accepted |
| [0004](0004-event-sourced-game-records.md) | Event-sourced game records | Superseded by [0010](0010-game-records-versioning-and-confidentiality.md) |
| [0005](0005-game-trait-and-decision-model.md) | `Game` trait and decision model | Superseded by [0009](0009-decision-protocol-and-agents.md) |
| [0006](0006-hidden-information-and-player-views.md) | Hidden information and player views | Accepted (amended by [0009](0009-decision-protocol-and-agents.md)) |
| [0007](0007-frontend-stack.md) | Front-end stack: React + WASM, then PWA, then Tauri | Proposed (amended by [0011](0011-multi-game-boundary.md)) |
| [0008](0008-reproducible-randomness.md) | Reproducible randomness | Accepted (extended by [0010](0010-game-records-versioning-and-confidentiality.md)) |
| [0009](0009-decision-protocol-and-agents.md) | Decision protocol, agents and information sets | Proposed |
| [0010](0010-game-records-versioning-and-confidentiality.md) | Game records: versions, hashes and confidentiality | Accepted |
| [0011](0011-multi-game-boundary.md) | Multi-game boundary: closed enums, JSON and TypeScript | Accepted |
| [0012](0012-state-representation-and-performance.md) | State representation and performance budget | Proposed |
| [0013](0013-distribution-and-licensing.md) | Distribution, licensing and card data | Accepted |
