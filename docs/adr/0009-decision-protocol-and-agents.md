# 0009. Decision protocol, agents and information sets

- Status: Proposed (signatures confirmed by the second game in phase 7)
- Date: 2026-11-03
- Supersedes: [0005](0005-game-trait-and-decision-model.md)
- Amends: [0006](0006-hidden-information-and-player-views.md)

## Context

ADR 0005 puts `legal_actions` on the full state and returns a `Vec<PlayerId>` for
pending players; ADR 0006 says an agent decides from a `View`, never from the full
state. The two cannot both hold: with only a `View`, an agent or a network client
cannot enumerate legal actions, and a view that does not carry them cannot be used
to play. The same gap appears in multiplayer: the server must send the view and the
legal actions from the same instant, or a client can submit an action computed on a
stale view.

The boundary spike ([0001](../spikes/0001-wasm-boundary.md)) validated a
`Decision { id, view, legal }` bundle, a `DecisionId`, a `PlayerSet` bitset and a
`Copy` status. It also showed that simultaneous choices must be stored privately and
resolved in a canonical order, and that the view must be a sufficient statistic for
determinization.

## Decision

### Trait shape

```rust
pub trait Game: Clone + Sized {
    type Config: Clone + Serialize + DeserializeOwned;
    type Action: Copy + Eq + Hash + Debug + Serialize + DeserializeOwned;
    type View: Clone + Serialize;
    type Outcome: Copy + Serialize;
    type Error: Error;

    fn new(config: &Self::Config, seed: u64) -> Result<Self, Self::Error>;
    fn status(&self) -> Status<Self::Outcome>;
    fn decision_for(&self, player: PlayerId) -> Option<Decision<Self::View, Self::Action>>;
    fn legal_actions_into(&self, player: PlayerId, out: &mut Vec<Self::Action>);
    fn apply(
        &mut self,
        player: PlayerId,
        decision: DecisionId,
        action: Self::Action,
    ) -> Result<(), ApplyError>;
    fn view(&self, player: PlayerId) -> Self::View;
}
```

- `decision_for` is the UI and simple-agent entry point; `legal_actions_into` is the
  allocation-free primitive used by search. The game implements both; `decision_for`
  may call the latter.
- `Decision` is a wire type: `Serialize` only. `View` is `Serialize` only; `Action`
  is `Serialize + Deserialize` (clients send it); `Config` is both (records).
- `DecisionId(u32)` is monotone per game. Ids for a simultaneous batch are assigned
  when the phase starts, not when a player submits, so the state does not depend on
  arrival order.
- `apply` is atomic: on error, the state is unchanged. It checks that the player is
  awaited, that the id matches and that the action is legal before mutating.
- `status()` lists only players who still have to act. `Status` is `Copy`;
  `PlayerSet` is a `u16` bitset; `Outcome` is a fixed array serialized as a slice.

### Simultaneous decisions

- A submission is definitive; a player cannot change it or submit twice.
- The choice is stored privately. Other players' views show only that the player
  submitted (`submitted: true`), never the value.
- Resolution happens when the last awaited player submits, in ascending `PlayerId`
  order, never in arrival order.
- A test applies the same choices in several orders and compares the resolved state.

### Views, agents and determinization

The view is the player's full information set: public state, own private state, and
any public constraint needed to reason (for example Gang of Four's "the winner holds
my best card"). It must be a sufficient statistic for determinization:

```
view(state, p) == view(determinize(state, p), p)
legal(state, p) == legal(determinize(state, p), p)
```

`Determinize` is a separate trait, so games that do not support search do not pay for
it:

```rust
pub trait Determinize: Game {
    fn determinize<R: Rng>(view: &Self::View, player: PlayerId, rng: &mut R) -> Self;
}
```

Taking the view rather than `&self` is deliberate: it forces the view to be a
sufficient statistic and makes it impossible for the search to peek at the true state.
The spike implemented `determinize(&self, ...)` for convenience; the final trait must
not.

`Agent` takes the decision, never the state:

```rust
pub trait Agent<G: Game> {
    fn decide(
        &mut self,
        player: PlayerId,
        decision: &Decision<G::View, G::Action>,
        budget: &mut dyn Budget,
    ) -> G::Action;
}

pub trait Budget {
    fn keep_going(&mut self, iterations_done: u64) -> bool;
}
```

`Budget` is a number of iterations or a caller-provided clock; there is no
`Instant::now()` in the AI crate (ADR 0003). ISMCTS uses the ranks exposed by
`Outcome`; a game-specific utility hook is added only if benchmarks require it.

### Configuration and player count

`new` is fallible. `Config` is validated before construction; `PlayerId` is a `u8`
in `0..MAX_PLAYERS`, with `MAX_PLAYERS = 10` (6 nimmt!). `legal_actions_into` for a
player who is not awaited leaves the buffer empty; `decision_for` returns `None`.

## Alternatives considered

- **A `PlayerView` trait carrying `legal_actions`.** It duplicates the source of
  truth (the state must also know), has no `DecisionId`, and has no answer for a
  spectator or a player who already submitted. Rejected.
- **Give agents the full state and rely on discipline.** Unenforceable across the
  WASM and server boundaries, and unfair. Rejected.
- **`Vec<PlayerId>` in `Status`.** Allocates on every call and is not `Copy`.
  Rejected; `PlayerSet` is cheaper.
- **`u64` decision ids.** They become `BigInt` at the WASM boundary and `JSON.parse`
  cannot produce one. Rejected; `u32`.
- **Resolve simultaneous choices on submission.** It leaks the first choice and makes
  the result order-dependent. Rejected.
- **`current_player()`** (ADR 0005 model 1). It breaks on simultaneous decisions and
  interrupts. Rejected.
- **An object-safe `dyn Game`.** Not possible with associated types and `Self`; the
  multi-game boundary is ADR 0011. Rejected.

## Consequences

- Clients, agents and the CLI use one `Decision` shape; the view and the legal
  actions cannot come from different instants.
- `DecisionId` handles reconnection, duplicate submissions and stale views.
- The engine must store pending simultaneous choices privately and assign batch ids.
- The view must be a sufficient statistic. Games with constraints must include them,
  or provide a game-specific determinization; 6 nimmt! is the test in phase 7.
- `decision_for` allocates a view and a `Vec<Action>`; search uses
  `legal_actions_into`. Phase 4 benchmarks both.
- `Game` remains non-object-safe; the application boundary is ADR 0011.
- ADR 0009 stays Proposed until phase 7. ADR 0005 is superseded; ADR 0006's view
  principle stands, with the sufficient-statistic addition.
