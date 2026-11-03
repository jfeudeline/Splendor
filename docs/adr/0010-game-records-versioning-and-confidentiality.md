# 0010. Game records: versions, hashes and confidentiality

- Status: Accepted
- Date: 2026-11-03
- Supersedes: [0004](0004-event-sourced-game-records.md)
- Extends: [0008](0008-reproducible-randomness.md)

## Context

ADR 0004 represents a game as `format_version + game + config + seed + actions` and
replays the actions. Review and the boundary spike exposed three problems:

1. One version field conflates the wire schema, the rules and card data, and the
   random generator. A rules fix that does not change the JSON can replay to a
   different state with no error.
2. There is no state fingerprint, so a divergence is silent until someone notices a
   wrong result.
3. The record stores player commands, not derived events. Calling it event-sourced
   invites confusion, and sending the record to a client leaks the seed and private
   commands.

## Decision

### Command log

The record is a **command log**: it stores player commands, not the events they
produce. The state is obtained by replaying the commands. Observations for
animations, if needed, are derived and are not the source of truth.

### Format

```rust
struct GameRecord<C, A> {
    schema_version: u32,
    game_id: GameId,
    rules_version: u32,
    rng_version: u32,
    config: C,
    seed: u64,
    commands: Vec<RecordedCommand<A>>,
    outcome: Option<Outcome>,
}

struct RecordedCommand<A> {
    seq: u32,
    decision_id: DecisionId,
    actor: PlayerId,
    action: A,
    state_hash: u64,
}
```

- `schema_version` changes when the JSON layout or field types change.
- `rules_version` changes when the rules or the card data change.
- `rng_version` changes when the seed derivation or the shuffle change (ADR 0008).
- `state_hash` is computed after applying the command, over a **canonical** encoding
  of the state. The default is FNV-1a 64-bit, hence the `u64` field; if a
  cryptographic digest is preferred, use BLAKE3 and store the digest as a hex string
  instead. Never `std::collections::hash_map::DefaultHasher`, whose algorithm is not
  guaranteed stable. The hash is for divergence detection, not a substitute for
  server validation.
- `outcome` is stored for history and quick display; replay remains authoritative.
- Commands are stored in submission order with their `seq`. For simultaneous phases
  the engine resolves in canonical `PlayerId` order (ADR 0009), so the result does not
  depend on the recorded order.

`GameId` is a stable identifier (`splendor`, `six-nimmt`, …) owned by the platform
crate (ADR 0011); it is not a free-form string in the engine.

### Versions and compatibility

- All three versions must match the current engine to replay. A mismatch is rejected
  with a clear error, never silently reinterpreted.
- A migration is a new, tested code path; it is not an implicit rule change. Until a
  migration exists, old records are rejected.
- Bumping `rules_version` or `rng_version` is a breaking change of the record format.
  Bump `schema_version` only when the layout changes.
- Keep a corpus of records per supported version under `tests/records/` and replay it
  in CI; compare the final `state_hash` and `outcome`.

### Confidentiality boundary

- `AuthoritativeRecord` (server): seed, all commands and hashes. Never sent to a
  client during play.
- `PlayerUpdate`: `{ seq, status, view, decision }` for one seat. Never contains the
  seed, another seat's private command, or the full state.
- `PublicReplay`: optional, after the game, with an explicit policy. It may reveal the
  seed and commands only when the game is finished and the information is no longer
  sensitive; otherwise it is redacted or omitted.
- A local single-player save may include the seed; it is the player's own game. An
  online save is a server-side operation.

### Validation and checkpoints

The server checks, for every command: `seq` is the next one, the actor is awaited,
the `decision_id` matches, the action is legal, and `state_hash` matches its own
state after applying it. A mismatch aborts the session and flags the record.

Servers may keep snapshots every N commands as caches; the command log remains the
source of truth. Checkpoints also make the first divergence easy to locate.

### Recordable games

The hash and record helpers live behind a separate trait, so `Game` stays minimal and
games that are only used in tests do not pay for it:

```rust
pub trait Recordable: Game {
    fn state_hash(&self) -> u64;
}
```

All shipped games implement it; the CLI and the server bound on `G: Recordable`.

## Alternatives considered

- **One `format_version`** (ADR 0004). It conflates schema, rules and RNG, and
  silently changes replay results. Rejected.
- **Serialize the full state.** No replay or history, and the format breaks on every
  state change. Rejected.
- **Store the deck order instead of the seed.** Robust but larger and game-specific;
  ADR 0008 already chose the seed. Rejected.
- **Send the record to clients for synchronisation.** It leaks the seed and private
  commands. Rejected.
- **`DefaultHasher`.** Not stable across Rust versions. Rejected.
- **Keep old engines to replay old records.** Expensive and hard to test; reject
  instead, migrate only on demand. Rejected as a default.

## Consequences

- Save, resume, replay, server validation and reconnection share one format.
- A rules or RNG change is explicit and testable; old records are rejected or
  migrated deliberately.
- The record is slightly larger (one hash and one sequence number per command);
  acceptable.
- The docs and code use "command log", not "event sourcing".
- The server never sends the seed during play; clients receive per-seat updates.
- The hash algorithm is an implementation detail, but it must be stable and
  documented. `GameRecord` and the hash are per game; the multi-game dispatch is
  ADR 0011.
