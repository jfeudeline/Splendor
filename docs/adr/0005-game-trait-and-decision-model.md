# 0005. `Game` trait and decision model

- Status: Proposed (signatures to be confirmed during phases 1–2)
- Date: 2026-10-15

## Context

Generic components (AI agents, game runner, simulator, server) need a common interface
to games. Splendor is almost strictly alternating, but the next planned game, Agricola,
has multi-step turns, card effects that ask other players for decisions, and possibly
simultaneous decisions (drafting). Other platform games (7 Wonders, Dominion…) have
similar needs.

Possible models for "who plays when":

1. **Single current player**: `current_player() -> PlayerId`. Simplest; breaks on
   interrupts, simultaneous play and multi-step turns.
2. **Atomic compound actions**: one action per turn containing every sub-choice.
   Leads to combinatorial explosion (e.g. Agricola fencing) and complex UIs.
3. **Pending decisions / state machine**: the engine asks one player for one decision
   at a time; a turn can span several decisions; interrupts push decisions for other
   players.
4. **Simultaneous decisions**: the engine waits for a set of players and resolves once
   all have chosen, hiding choices until then.
5. **Chance as a player** (OpenSpiel style): every random draw is an explicit decision
   of a "chance" player. Orthogonal to the above.

Card effects and triggers (Agricola, Dominion) are not a turn model; they are a layer
built on top of model 3.

## Decision

Combine models **3 and 4**, without model 5 for now.

The trait is implemented **by the game state type itself**, with methods taking `self`,
which is the idiomatic Rust shape (`state.apply(…)`, `state.status()`):

```rust
pub enum Status {
    AwaitingDecision(Vec<PlayerId>), // players who must act now
    Finished(Outcome),
}

pub trait Game: Clone {
    type Config;
    type Action: Clone;
    type View;

    fn new(config: &Self::Config, seed: u64) -> Self;
    fn status(&self) -> Status;
    fn legal_actions(&self, player: PlayerId) -> Vec<Self::Action>;
    fn apply(&mut self, player: PlayerId, action: Self::Action)
        -> Result<(), IllegalAction>;
    fn view(&self, player: PlayerId) -> Self::View;
}
```

Generic code is written as `fn play<G: Game>(game: &mut G, …)`, and `GameRecord<G>`
(ADR 0004) uses `G::Config` and `G::Action`.

- The granularity of `Action` is game-specific; the trait does not impose it.
- **Invariant**: while the status is `AwaitingDecision(players)`, `legal_actions` is
  non-empty for each of those players. A game where a player can be stuck must offer an
  explicit action for it (in Splendor, `Pass`, see `docs/rules/splendor.md`). This is
  checked by property tests in phase 4.
- **Splendor**: `AwaitingDecision` always contains exactly one player. The main action
  (take 3 different gems, take 2 identical gems, reserve, buy, or pass when nothing else
  is possible) is atomic; discarding tokens above 10 and choosing among several eligible
  nobles are separate follow-up decisions, tracked by an internal
  `Phase { Main, Discard, ChooseNoble }`.
- **Agricola** will add a decision stack and an effect system inside its own crate,
  without changing the trait.
- **Randomness** stays an internal seeded RNG (ADRs 0003 and 0008). For Splendor, all
  randomness happens at setup (deck shuffles); ISMCTS re-samples unknown information from
  the player view. Explicit chance nodes may be introduced later through a new ADR.

### Known performance trade-off

`status()` and `legal_actions()` allocate a `Vec` on every call, and search AIs call them
millions of times. This is accepted for clarity. If the criterion benchmarks of phases 6–7
show it matters, candidates are: `SmallVec`, a player bitset in `Status`, or
`legal_actions_into(&self, player, &mut Vec<Action>)` to reuse a buffer. Such a change
is recorded in a new ADR.

## Alternatives considered

- **Model 1 only**: would need a breaking change for Agricola.
- **Model 2**: unworkable action spaces for Agricola; heavier UI for Splendor too.
- **Model 5 now**: more explicit and AI-friendly, but more verbose and unnecessary for
  Splendor.
- **Associated functions on a separate marker type** (`Game::apply(&mut state, …)` with
  `type State`): allows a game type distinct from its state, but is less idiomatic and
  noisier to call; nothing in the project needs that separation yet.

## Consequences

- One `status()` method replaces `current_player()` for a much wider coverage of games.
- UIs show only the options available at the current step.
- Generic AIs must handle several pending players; with ISMCTS, other players'
  simultaneous choices are treated as hidden information.
- Splendor has a slightly richer internal state (phase) than a naive implementation.
- A game can never deadlock silently: an empty action list is a bug, caught by tests.
