# 0001. Record architecture decisions

- Status: Accepted
- Date: 2026-10-15

## Context

This is a long-running learning project that will grow from a CLI game into a multi-game
platform. Decisions made early (engine shape, data formats) will constrain later work,
and the reasons behind them are easy to forget.

## Decision

Record significant architecture decisions as Architecture Decision Records (ADRs) in
`docs/adr/`, using [template.md](template.md). Files are numbered sequentially
(`NNNN-short-title.md`).

An ADR is **Proposed** while its details are still expected to change (for example,
signatures to be refined while implementing, or a decision that only takes effect in a
later phase); it may be edited freely. It becomes **Accepted** once the decision is
implemented or firmly settled. An accepted ADR is not rewritten; it is superseded by a
new one.

## Consequences

- The history of design choices lives next to the code and is versioned with it.
- Writing an ADR is a small extra step for each significant decision.
