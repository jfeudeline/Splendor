# 0001. Spike: the WASM / JSON boundary

- Status: Done (throwaway code, removed; this document is the record)
- Date: 2026-11-03
- Code: removed after the experiment — **nothing in the repository depends on it**
- Toolchain: Rust 1.99.1, wasm32-unknown-unknown, wasm-pack 0.15.1 (via npx),
  wasm-bindgen 0.2.130, Node 24.22, TypeScript 5.x

> The spike code (a toy game, a WASM adapter and a small web page) was deleted on
> 2026-11-03 after the experiment. The measurements and payloads below were
> produced by that code; the contract is restated here so that the ADRs do not
> depend on it. Rebuilding it is only worth doing if a later decision needs a
> measurement that is not recorded here.

## What was tested

A toy game ("Draft", 2–4 players, 3–8 rounds) that reproduces the cases the
ADRs disagree on:

- a hidden deck: only the count and the **multiset** of unseen cards are public;
- a sequential phase (one player at a time);
- a simultaneous phase where choices stay secret until every player submits;
- in-play randomness drawn from a semantic sub-stream;
- a `Copy` state with no heap allocation;
- a `Decision` bundle carrying `{ id, view, legal }` from the same instant.

It is deliberately **not** Splendor. The spike validates the contract and the
toolchain, not the rules or the card data.

## Results

| Check | Result |
|---|---|
| `cargo test` (contract + toy) | 18 tests, all green |
| `cargo clippy --workspace --all-targets -D warnings` | clean |
| `cargo check --target wasm32-unknown-unknown` | clean |
| `wasm-pack build --target web` | 161 KB `.wasm`, ~2 s incremental |
| `wasm-pack build --target nodejs` | same payload, played from Node |
| Node smoke test (real WASM) | 7 boundary calls, 30+ assertions, game finishes |
| `tsc --noEmit` on hand-written types | clean |
| `size_of::<DraftGame>()` | **216 bytes**, `Copy` |
| `size_of::<Status<Outcome>>()` | `Copy` (compile-time assertion) |

## Findings

### 1. `Decision { id, view, legal }` works — keep it

The client and the agent receive one object, so the view and the legal actions
cannot be from different instants. `decision_for(p) -> Option<Decision>` also
handles the spectator / already-submitted case cleanly: `None`, not an empty
list with an ambiguous meaning.

`DecisionId(u32)` works end to end. A stale submission produces a structured
error and leaves the state unchanged:

```json
{"type":"stale_decision","expected":4,"got":1004}
```

The toy assigns the whole simultaneous batch of ids **when the phase starts**,
not when a player submits. This is what makes the resolved state independent of
arrival order; the test applies the same choices in both orders and compares
the states byte for byte. Keep this rule in the ADR.

### 2. `Status: Copy` is feasible, but `Outcome` must be fixed-size

`Status<Outcome>` is `Copy` only because `Outcome` stores `[u8; MAX_PLAYERS]`
internally and serializes a slice:

```json
{"type":"finished","value":{"ranks":[2,1]}}
```

The `Vec<u8>` version proposed in the review would make `Status` non-`Copy`.
Decision: fixed array internally, slice on the wire. Consequence: `Outcome`
needs a hand-written `Serialize` (about 8 lines), which is acceptable.

### 3. `PlayerSet` needs a custom serde impl

Internally a `u16` bitset (`Copy`, no allocation); on the wire a list:

```json
{"type":"awaiting_decision","value":[0,1]}
```

About 40 lines of `Serialize`/`Deserialize`, including range validation
(`[10]` is rejected). Worth it: `Status` and every `awaiting` payload stay
allocation-free.

### 4. The view must be a *sufficient statistic*, not just counts

This is the main architectural finding. The toy's `View` includes the
**multiset** of unseen cards (`unseen`), never their order. `determinize` can
then rebuild a consistent deck. The strong invariant passes:

```
view(state, p) == view(determinize(state, p), p)
legal(state, p) == legal(determinize(state, p), p)
```

ADR 0006 currently says "only deck sizes are shown". That is **not enough** for
ISMCTS: a view with only sizes cannot reconstruct a legal state. For Splendor
the unseen multiset can be derived from the 90-card catalog and the public
cards, so the engine can keep the `View` small; for 6 nimmt! the unseen cards
are public too. For Gang of Four, the private knowledge is a **constraint**
("the winner holds my best card"), not a multiset, so `determinize(view)` may
not be expressible generically.

Decision to record: the view is the player's full information set; it must be
sufficient to sample a consistent state, or the game must provide a
game-specific determinization. Add the `view == view ∘ determinize` property
test to phase 4.

### 5. `u64` crosses WASM as `BigInt`

Confirmed by the generated `.d.ts`:

```ts
static echo_u64(value: bigint): bigint;
```

`Session.echo_u64(123)` throws; `123n` works. `JSON.parse` never produces a
`BigInt`, so a `u64` must never appear in a client JSON payload.

Consequences:

- the seed stays server-side; when it must be transmitted (save export after
  the game), use a hex string;
- `DecisionId` is `u32` (safe in JS), not `u64`;
- state hashes, if exposed at all, are hex strings.

`Option<String>` works as expected: `decision_json` generates
`string | undefined` when the player is not awaited.

### 6. Serde adjacent tags produce a usable TS union — with a caveat

`#[serde(tag = "type", content = "value", rename_all = "snake_case")]` gives:

```json
{"type":"take","value":{"slot":0}}
{"type":"pass"}
{"type":"bonus","value":1}
```

The TypeScript union handles all three, and the exhaustive `switch` compiles.
The caveat: the shape of `value` differs between struct and newtype variants
(`{slot}` vs `1`). It is fine, but it must be documented; if uniformity
matters, make every variant a struct variant.

### 7. Hand-written TypeScript types drift — generate them

The spike validates the actual JSON at runtime (Node smoke test) and the
hand-written types at compile time (`tsc`), but nothing ties the two together.
A renamed Rust field would pass `tsc` and only fail at runtime. Recommendation
for phase 8: `ts-rs` or `typeshare`, with the generated file checked in CI. If
neither handles generic `Decision<V, A>` cleanly, generate the concrete
`Decision<View, Action>` per game.

### 8. `serde_json` at the WASM edge costs size, not correctness

161 KB for the toy, dominated by wasm-bindgen + serde_json. Acceptable for a
card game; if the PWA budget matters, `serde-wasm-bindgen` or a compact binary
protocol is a later option. The JSON strings are exactly what a WebSocket
server will send, which is why the spike used them.

### 9. `Decision` is convenient for UI, too costly for search

`decision_for` builds a view (four `Vec`s in the toy) and a `Vec<Action>` on
every call. Fine for a CLI or a browser panel; not for MCTS. Keep two entry
points:

```rust
fn decision_for(&self, p: PlayerId) -> Option<Decision<Self::View, Self::Action>>; // UI, simple agents
fn legal_actions_into(&self, p: PlayerId, out: &mut Vec<Self::Action>);           // search
```

The spike did not implement the second one; it is the next thing to benchmark.

### 10. `rand_core` 0.10 changed the trait names

The bounded RNG helper must use `rand_core::Rng` for `next_u32`/`next_u64`;
`RngCore` now exposes fallible `try_next_*`. ADR 0008's snippet should be
updated. `sub_rng(seed, domain, index)` works, and the semantic index (round,
deck id) is what keeps a refactoring in one domain from shifting another.

### 11. Tooling is reproducible but must be pinned

`wasm-pack` is not installed globally; `npx --yes wasm-pack` (0.15.1) works.
The generated web target's `init(bytes)` is deprecated in wasm-bindgen 0.2.130:
use `init({ module_or_path: bytes })` when testing from Node. CI must pin
`wasm-pack` and `wasm-bindgen`, otherwise the generated `.d.ts` changes under
the client's feet.

### 12. `apply` atomicity is testable and worth a property

The spike rejects an illegal or stale action before mutating, and the test
asserts the state is unchanged. This must be part of the `Game` contract, not
an implementation detail: a half-applied state is fatal server-side.

## What this spike does **not** prove

- Splendor's rules, card data, token limits, nobles, end of game.
- 6 nimmt!'s simultaneous hidden hands and row choice.
- Gang of Four's constraints (the "winner holds my best card" knowledge).
- The network protocol, reconnection, persistence, cheat checks.
- React, PWA, browser execution. The web target was executed in Node with the
  wasm bytes, and the HTTP server returned 200 for the page, the JS and the
  wasm; no browser was available in this environment.
- MCTS performance. Only the state size was measured (216 bytes for the toy);
  `legal_actions_into`, clone cost and rollouts per second still need a
  benchmark in phase 4.

## Recommended ADR changes

| ADR | Change |
|---|---|
| 0005 → 0009 | `Decision { id, view, legal }`, `DecisionId(u32)`, batch ids at phase start, canonical resolution order, `status()` lists only pending players, `decision_for -> Option`, atomic `apply` |
| 0006 | View is the full information set and a sufficient statistic for determinization; add the `view == view ∘ determinize` and `legal == legal ∘ determinize` tests to phase 4; document the Gang of Four constraint case |
| 0008 | Use `rand_core::Rng` (0.10); semantic `sub_rng(seed, domain, index)`; seeds are hex strings at the boundary, never `u64` in client JSON |
| 0007 / 0011 | Adjacent serde tags; `u32` ids; generated TS types checked in CI; `platform-wasm` (not `splendor-wasm`) |
| 0012 | Target a `Copy` state (~216 B for the toy, ~300 B estimated for Splendor); add `legal_actions_into` and benchmark clone + legal actions in phase 4 |
| 0004 / 0010 | Per-command hash and checkpoints; authoritative record private; the seed is server-only until the game is finished |

## Open questions to settle in the ADRs

1. Splendor: does the `View` expose the unseen multiset, or does `determinize`
   derive it from the catalog? The invariant is the same; the payload size and
   the coupling differ.
2. 6 nimmt!: what exactly must the view contain for a consistent
   determinization (played cards, unseen multiset, current trick)?
3. Gang of Four: is a generic `determinize(view)` enough, or does the game need
   a `SearchGame` trait with game-specific constraints?
4. TS generation: `ts-rs`, `typeshare`, or hand-written + a runtime schema
   check? The spike shows hand-written alone is not enough.
5. Errors over WASM: `JsError` carries a JSON string; is that enough for the
   client, or should the adapter return a structured error payload?
