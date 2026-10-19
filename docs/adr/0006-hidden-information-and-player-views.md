# 0006. Hidden information and player views

- Status: Accepted
- Date: 2026-10-15

## Context

Splendor has hidden information: the order of the three development decks, and cards
reserved face down from the top of a deck (visible only to their owner). Online
multiplayer must not leak hidden information to clients, and AI players should be fair.

## Decision

- Each game defines a `View` type: what a given player is allowed to know.
- `Game::view(state, player)` is the only way state is exposed to players, whether human
  (CLI, web, server clients) or AI.
- Agents take decisions from a `View`, never from the full `State`.
- Search AIs (ISMCTS) build *determinizations*: full states sampled to be consistent
  with the view (e.g. shuffling the unknown cards), and search over them.

For Splendor the view hides deck contents/order (only deck sizes are shown) and other
players' face-down reserved cards (only their count and level).

## Alternatives considered

- **Omniscient AIs**: simpler at first, but unfair, and it would force a refactor
  before multiplayer, since the server needs views anyway.

## Consequences

- The same view type feeds the CLI, the web front, the server protocol and AIs.
- Games must provide a way to sample a full state consistent with a view
  (for ISMCTS); this may become a trait method in phase 9.
- Spectator views (no hidden information revealed) can be added in the same way.
