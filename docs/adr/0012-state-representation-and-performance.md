# 0012. State representation and performance budget

- Status: Proposed (target confirmed by the phase 4 benchmarks)
- Date: 2026-11-03

## Context

ADR 0005 accepts `Vec` allocations in `status()` and `legal_actions()` and defers
optimization to phase 10. Review and the boundary spike show that the real cost for
ISMCTS is the state clone and the allocations in `legal_actions`, not only
`status()`. Fixing the representation after the web and server layers are built is
expensive. Splendor is small enough to make a compact state cheap now: 90 cards, 10
nobles, 2–4 players.

## Decision

### State

- The state is `Clone + Copy` where possible and contains no `Vec`, `String`,
  `HashMap`, `HashSet` or `Rc`/`Arc`. Use fixed arrays, `ArrayVec` with a fixed
  capacity, `CardId(u8)`, `NobleId(u8)` and `PlayerSet(u16)`.
- Target: **≤ 512 bytes** for a Splendor state. The toy spike measured 216 bytes. If
  the target is missed, revisit with measurements, not guesses.
- The card and noble catalog is a `static` table; the state stores indices, never card
  structs or names.
- Deck: fixed array plus cursor (`[CardId; 40]`, `u8 len`) or `ArrayVec`; never
  `remove(0)`. Market: `[[Option<CardId>; 4]; 3]`. Player: `bonuses: [u8; 5]`,
  `tokens: [u8; 6]`, `points: u8`, `cards_bought: u8`,
  `reserved: ArrayVec<ReservedCard, 3>`. The exact fields are settled in phases 1–2;
  these constraints are not.
- No `HashMap`/`HashSet` in `game-core` or the game crates; enforce with
  `clippy.toml` (`disallowed-types`). Iteration order must be deterministic.
- The RNG is not stored as a live `ChaCha8Rng`. Store `seed` and semantic
  counters/indices, and derive `sub_rng(seed, domain, index)`. Cloning a state for
  search must not reveal future draws.

### Status and outcomes

- `Status` uses `PlayerSet`, not `Vec<PlayerId>`.
- `Outcome` stores a fixed `[u8; MAX_PLAYERS]` and serializes a slice; this keeps
  `Status: Copy`.

### Legal actions

- `legal_actions_into(&self, player, out: &mut Vec<Action>)` is the primitive; it
  appends in a deterministic order.
- `decision_for` is the convenience for the UI and simple agents; it may allocate a
  view and a `Vec<Action>`.
- Search uses `legal_actions_into` and reuses the buffer across nodes.

### Benchmark and exit criterion

In phase 4 (not phase 10), measure on a mid-game state:

- `size_of::<State>()`;
- `clone` (should be a memcpy);
- `apply` plus `legal_actions_into` in a loop;
- a Random-vs-Random simulation, reported in plies per second.

Target: **1–2 M plies/s** in a native release build for the simulator, and a state
under 512 bytes. If the target is missed, fix the representation before the web and
server layers are built. Record the numbers in the phase 4 notes; the benchmark is an
exit criterion, not an aspiration.

## Alternatives considered

- **`Vec<Card>` and `Vec<Player>`.** Simple but allocates on every clone; rejected for
  search.
- **`Arc<[Card]>` plus copy-on-write.** Cheap clone but allocations on mutation; a
  `Copy` state is simpler for MCTS. Rejected.
- **Undo log.** Fast rollouts but complex with simultaneous decisions, snapshots and
  serialization; rejected for now.
- **Optimize in phase 10.** The web and server would be built on the slow
  representation; rejected.
- **`HashMap` for token counts.** Nondeterministic iteration; rejected.
- **`SmallVec` everywhere.** Use it only where the bound is clear; `ArrayVec` and
  fixed arrays are simpler.

## Consequences

- Cloning the state is a memcpy; MCTS and simulations are viable.
- Some rules code is less ergonomic (indices, fixed arrays); acceptable for a small
  game.
- The card data is a static table; a test verifies 40/30/20 cards and 10 nobles
  (ADR 0013).
- The phase 4 benchmark becomes a gate; a missed target is caught before the web and
  server.
- The budget is per game: if a later game needs a larger state, measure it and
  revisit; do not generalize the Splendor numbers.
