# 0004. Event-sourced game records

- Status: Accepted
- Date: 2026-10-15

## Context

The MVP needs save/resume and replay. Later, online play needs synchronization,
persistence, reconnection, asynchronous games and cheat-resistant validation.

## Decision

A game is represented by a `GameRecord`:

```rust
struct GameRecord<G: Game> {
    format_version: u32,
    game: String,                 // e.g. "splendor"
    config: G::Config,            // player count, options
    seed: u64,
    actions: Vec<(PlayerId, G::Action)>,
}
```

The current state is obtained by replaying the actions from the initial state. Records
are serialized as versioned JSON. Snapshots of the state may be cached for performance,
but the record is the source of truth.

## Alternatives considered

- **Serialize the full state**: simple save/resume, but no replay, no history, and
  the format breaks whenever the state struct changes.
- **State + diff events**: more flexible for rendering animations, but more formats to
  maintain; can be added later on top of action replay.

## Consequences

- Save, resume, replay, undo (in local games) and multiplayer sync share one mechanism.
- Changing the rules or the action format can invalidate old records: `format_version`
  must be bumped and old versions either migrated or rejected.
- The server can validate every action by applying it to its own state.
