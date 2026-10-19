# 0008. Reproducible randomness

- Status: Accepted
- Date: 2026-10-15

## Context

Game records are `config + seed + actions` (ADR 0004): replaying a saved game relies on
the seed producing exactly the same deck order, forever and on every platform.

ChaCha (from `rand_chacha`) is a well-defined algorithm whose output stream is stable.
But the helpers built on top of RNGs are not guaranteed to be:

- `rand`'s `SliceRandom::shuffle` and `Rng::gen_range` have changed their algorithms
  between `rand` versions, which changes their output for the same RNG state.
- `SeedableRng::seed_from_u64` expands a `u64` into a full seed with an algorithm that is
  an implementation detail of `rand_core`.

A dependency upgrade could therefore silently make every saved game replay differently,
with no error.

## Decision

In game crates (`game-core`, `splendor`, later others):

1. Use `ChaCha8Rng` from `rand_chacha`, with default features disabled (no `getrandom`,
   no OS entropy).
2. Build the 32-byte ChaCha seed from the game's `u64` seed with our own documented
   function (e.g. the seed's little-endian bytes followed by zeros), then call
   `from_seed`. Never use `seed_from_u64`.
3. Implement our own helpers in `game-core`:
   - a bounded integer function (`below(n)`), using rejection sampling on `next_u64` to
     avoid bias;
   - a Fisher–Yates `shuffle` built on it.
   Never use `rand`'s `shuffle`, `gen_range` or distributions in game crates.
4. **Golden tests**: for a few fixed seeds, the first raw outputs, a shuffle of `0..90`
   and the initial Splendor setup are checked against hard-coded expected values.
   If a golden test fails, replay compatibility is broken.
5. AI crates are not bound by this ADR: AI randomness (ISMCTS rollouts, random agent)
   only needs to be seedable for reproducible tests, not stable across versions.

## Alternatives considered

- **Pin `rand` to an exact version**: works until the day we need to upgrade; the
  breakage is then silent.
- **Store the initial deck order in the record instead of the seed**: robust, but makes
  records larger and game-specific, and does not cover games that draw randomness during
  play. It remains an option if a future game needs it.

## Consequences

- About 30 lines of code to write and test; a good small exercise on integer arithmetic
  and traits (`RngCore`).
- `rand_chacha` can be upgraded safely: golden tests catch any change.
- Changing the shuffle or seed derivation is a breaking change of the record format
  (`format_version` must be bumped, see ADR 0004).
