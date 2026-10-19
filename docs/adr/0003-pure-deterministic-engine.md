# 0003. Pure, deterministic, WASM-compatible engine

- Status: Accepted
- Date: 2026-10-15

## Context

The same engine must run in a CLI, in a browser or mobile WebView (WASM), on a server,
and inside search AIs that simulate thousands of games per decision. Games must be
saveable and replayable exactly.

## Decision

Game crates (`game-core`, `splendor`, later others) are **pure**:

- No I/O (files, network, stdin/stdout), no clock, no threads, no global state.
- All randomness comes from a seeded RNG; the seed is part of the game creation input.
  How randomness is made reproducible across versions and platforms is specified in
  [ADR 0008](0008-reproducible-randomness.md).
- Same seed + same actions ⇒ same state, on every platform.
- State is `Clone` (cheap enough for search) and `serde`-serializable.
- Dependencies must compile for `wasm32-unknown-unknown`. CI checks it **from phase 0**
  (`cargo check --target wasm32-unknown-unknown` on the library crates), so that an
  incompatible dependency is caught when it is added, not at the WASM phase.

The `ai` crate is also compiled to WASM (AIs run in the browser, see ADR 0007), so it
follows the same constraints, with one addition:

- **No clock in AI crates either**: `std::time::Instant::now()` panics on
  `wasm32-unknown-unknown`. Search budgets are expressed as a number of iterations, or
  through a clock/deadline provided by the caller (native `Instant` in the CLI,
  `performance.now()` in the browser).

Parallelism (e.g. running many games with rayon) is the caller's business, never the
engine's.

## Alternatives considered

- **Engine using `thread_rng()` and printing directly**: faster to start, but not
  replayable, hard to test, and unusable in WASM or on a server.
- **Checking WASM compatibility only at phase 7**: saves one CI line, but lets
  incompatible dependencies (e.g. `getrandom` without a WASM backend, `std::time`)
  creep in for months.

## Consequences

- Rules are fully unit- and property-testable without mocks.
- Replays and save files only need the seed and the action log (see ADR 0004).
- The CLI, server and WASM layers are thin adapters around the engine.
- The seed is secret information in online games (it determines deck order): the server
  must never send it to clients.
- The WASM toolchain target (`rustup target add wasm32-unknown-unknown`) is needed in CI
  and locally from the start.
