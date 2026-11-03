# 0011. Multi-game boundary: closed enums, JSON and TypeScript

- Status: Accepted (review at the third game)
- Date: 2026-11-03
- Amends: [0002](0002-cargo-workspace-layout.md), [0007](0007-frontend-stack.md)

## Context

ADR 0002 plans `splendor-wasm` and a `server` that depends on `splendor`. The platform
is multi-game from phase 8, and the second game arrives in phase 7. `Game` has
associated types, a `Clone` supertrait and `new -> Self`, so it is not object-safe: a
server with heterogeneous rooms cannot store `Box<dyn Game>`, and `GameRecord<G>` is
statically typed. The boundary spike showed that JSON strings at the WASM edge work,
that adjacent serde tags produce a usable TypeScript union, and that hand-written TS
types drift.

## Decision

### Keep the generic trait, add a closed enum at the edge

Do not make `Game` object-safe. Keep it generic for the engine, AI, CLI and tests. At
the application boundary (server, WASM, multi-game CLI), use a closed enum per game:

```rust
enum AnyGame {
    Splendor(splendor::Game),
    SixNimmt(six_nimmt::Game),
}

enum AnyRecord {
    Splendor(GameRecord<SplendorConfig, SplendorAction>),
    SixNimmt(GameRecord<SixNimmtConfig, SixNimmtAction>),
}
```

A `match` is exhaustive, fast and explicit. Adding a game produces a compile error at
every match, which is the desired review. Revisit only at the third game (Gang of
Four) if the duplication is measured to be painful.

### JSON adapter

A thin `platform` crate owns the enum, the JSON adapter and the game registry. It is
used by both `platform-wasm` and `server`:

```rust
trait JsonSession: Send {
    fn game_id(&self) -> &'static str;
    fn status_json(&self) -> String;
    fn decision_json(&self, player: PlayerId) -> Option<String>;
    fn apply_json(
        &mut self,
        player: PlayerId,
        decision: DecisionId,
        action: &str,
    ) -> Result<(), String>;
    fn view_json(&self, player: PlayerId) -> String;
    fn record_json(&self) -> String;
}
```

One generic implementation `impl<G: Game + Recordable> JsonSession for Runner<G>`
covers the shared behavior; the enum dispatches. The adapter lives outside
`game-core`, so the engine stays pure. `record_json` is for the server and local
saves; the WASM binding must not expose it to clients (ADR 0010).

### Crate names

- `splendor-wasm` becomes `platform-wasm` (or `games-wasm`): it depends on all game
  crates, the AI crate and the `platform` crate.
- `server` depends on `platform`, not on a specific game.
- Add `platform` to the workspace in phase 7/8; it is the only crate that knows all
  games.
- The `ai` crate stays generic and depends only on `game-core`.

### JSON conventions

- Adjacent tagging for enums crossing the boundary:
  `#[serde(tag = "type", content = "value", rename_all = "snake_case")]`.
- `DecisionId` is `u32`; player ids are numbers; **no `u64`** in client JSON. Seeds
  and hashes are hex strings when they must be transmitted; the seed is normally
  server-only (ADR 0010).
- `Option<T>` is `T | null` in JSON. At the WASM boundary a `None` from
  `decision_json` is `undefined`; document the difference per field.
- No maps with non-string keys; no floats in state or actions.
- `Decision` is `Serialize` only; `Action` is `Serialize + Deserialize`; `View` is
  `Serialize`.
- Errors are structured JSON with a `type` tag, for example
  `{"type":"stale_decision","expected":4,"got":1004}`.

### TypeScript types

- Generate the payload types from Rust with `ts-rs` or `typeshare`; check the
  generated file in CI (`git diff --exit-code`).
- If the generator cannot handle the generic `Decision<V, A>`, generate the concrete
  `Decision<View, Action>` per game.
- The `.d.ts` from wasm-bindgen is not enough: it types the methods as `string`, not
  the payloads.
- Hand-written types plus `tsc` are a fallback, but the spike showed they drift from
  the Rust output; do not rely on them alone.

### Tooling

Pin `wasm-bindgen`, its CLI and `wasm-pack` in CI and in the docs. The generated JS
API changes between versions: `wasm-bindgen` 0.2.130 deprecated `init(bytes)` in
favour of `init({ module_or_path })`.

## Alternatives considered

- **`dyn Game` / `Box<dyn Game>`.** Impossible with associated types, `Clone` and
  `Self`. An erased facade with `serde_json::Value` now adds a pseudo-abstraction for
  two games. Rejected for now.
- **One WASM crate per game.** Duplicates the adapter, the TypeScript generation and
  the registry. Rejected.
- **Server depends on each game.** It grows a match per game anyway; a `platform`
  crate makes the dependency explicit. Rejected as the default.
- **`serde-wasm-bindgen` and `JsValue`.** Convenient but less explicit; JSON strings
  are exactly what the server sends, so they are the better boundary. Revisit if the
  JSON overhead matters.
- **Hand-written TypeScript only.** Drift; rejected.
- **Object-safe `DynGame` immediately.** Defer; measure at the third game.

## Consequences

- Adding a game touches the game crate, the `platform` enum, the UI and the tests;
  the compiler lists every place.
- WASM and server share one adapter; the engine crates stay independent and pure.
- The JSON boundary is explicit and testable with fixtures; no downcasting.
- TypeScript generation adds a build step and a CI check; worth it for a multi-game
  client.
- The `platform` crate is the only place that knows all games; keep it thin.
