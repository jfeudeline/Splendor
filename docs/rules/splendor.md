# Splendor rules notes

Clarifications and deliberate deviations from the official rules, as implemented by the
engine. The official rulebook remains the reference for everything not listed here.

Each entry is marked **Decided** or **Open**. Open entries are settled during phase 2
(rules), at the latest before the engine accepts the corresponding action.

## Taking tokens

- **Open — taking fewer than 3 different gems.** When fewer than 3 colors are available
  in the bank, may a player take 1 or 2 different gems? The rulebook does not say;
  implementations differ. Proposal: allowed only when fewer than 3 colors are available
  (never as a free choice). To check against the official FAQ.
- **Decided — taking 2 identical gems** requires at least 4 tokens of that color in the
  bank before taking (official rule).
- **Decided — discarding above 10 tokens** happens after the main action, as a separate
  decision; the player may return any tokens, including ones just taken (official rule).

## Paying for cards

- **Decided (deviation) — automatic payment.** The cost is paid with card bonuses first,
  then colored tokens, then gold for the remainder. The player does not choose to spend
  gold while keeping a colored token.
  - *Why*: keeping gold instead of a colored token is almost always at least as good,
    since gold is wild and both count towards the 10-token limit. The only exception is
    the effect on the bank (returning a colored token can enable or deny an opponent's
    "take 2"), which is marginal. Explicit payment would multiply the actions an AI must
    consider.
  - Making payment explicit later is a breaking change of the action format
    (`format_version` bump, ADR 0004).

## Reserving

- **Decided — reserving with no gold left** is allowed; the player just gets no gold
  (official rule).
- **Decided — at most 3 reserved cards** per player (official rule).
- Face-down reserved cards (from the top of a deck) are visible only to their owner
  (ADR 0006).

## Nobles

- **Decided** — checked at the end of the player's turn, after any discard; at most one
  noble per turn. If several are eligible, the player chooses (separate decision). Not an
  action (official rules).

## No legal action

- **Decided (clarification) — pass.** If a player can take no tokens, reserve nothing
  and buy nothing, their only legal action is `Pass`. `Pass` is never legal otherwise.
  This guarantees that `legal_actions` is never empty (ADR 0005).
  - *Open*: if every player passes in a row, the game can no longer progress. Proposal:
    the game ends immediately and is scored as usual. Rare enough to settle in phase 2.

## End of game

- **Decided** — when a player reaches 15 prestige points, the round is completed so that
  all players have played the same number of turns. Highest prestige wins; ties are
  broken by the fewest purchased development cards; a remaining tie is a shared victory
  (official rules).
