# ternary-polyrhythm

**Polyrhythms: the mathematics of "N against M" in ternary time.**

A polyrhythm is two or more rhythmic patterns playing simultaneously with different period lengths. 3 against 4. 5 against 7. The tension between the different pulse rates is the *entire point* — it's what makes West African drumming, gamelan, and math rock feel alive.

This crate computes polyrhythms for ternary hit patterns. Given N and M, it generates the two rhythm patterns, finds where they coincide (the "downbeat"), computes the full cycle length (LCM of N and M), and gives you tools to layer, shift, and analyze polyrhythmic structures.

## What's Inside

- **`Rhythm`** — a hit pattern: cycle length + set of hit positions. `hit_at(i)` checks any beat
- **`Polyrhythm`** — N against M. Generates both patterns, computes cycle length (LCM), coincidence points
- **`coincidence(n, m)`** — where do the two patterns hit simultaneously? The "one"
- **`shift(rhythm, offset)`** — shift a rhythm forward in time. Creates phase offset
- **`layer(rhythms)`** — combine multiple rhythms into one composite pattern (OR of all hits)
- **`gcd(a, b)`** / **`lcm(a, b)`** — the arithmetic that makes polyrhythms work
- **`euclidean(k, n)`** — Björklund's algorithm: distribute k hits as evenly as possible in n steps

## Quick Example

```rust
use ternary_polyrhythm::*;

// Classic 3:2 polyrhythm
let poly = Polyrhythm { n: 3, m: 2 };
// Pattern A: hits at 0, 4, 8 in a 12-step cycle
// Pattern B: hits at 0, 6 in a 12-step cycle
// Coincidence: beat 0 (the "one") — both patterns hit together

// Full cycle length
assert_eq!(poly.cycle_length(), 6); // LCM(3, 2)

// Where do they coincide?
let coincidences = coincidence(3, 2);
// [0] — only at the downbeat

// Layer into composite rhythm
let r1 = Rhythm::new(3, vec![0, 1]);
let r2 = Rhythm::new(4, vec![0, 2]);
let composite = layer(&[r1, r2]);

// Euclidean rhythm: 5 hits in 8 steps (as evenly as possible)
let euclid = euclidean(5, 8);
// [0, 1, 3, 5, 6] — the classic "Bossa Nova" pattern
```

## The Deeper Truth

**Polyrhythms are number theory made audible.** The cycle length is the LCM. The coincidence points follow the GCD structure. The tension between the two patterns is precisely the ratio N/M, which is irrational when N and M are coprime — meaning the patterns never fully resolve, they just keep cycling. This is why coprime polyrhythms (3:2, 5:3, 7:4) sound more interesting than simple ratios (4:2 = 2:1 = octave).

In ternary, the hits are {-1, 0, +1} — each hit carries a ternary value, making the composite pattern a *ternary sum* of the individual rhythms. This creates interference patterns that are richer than binary (hit/no-hit) but more structured than continuous amplitude.

**Use cases:**
- **Algorithmic music** — generate complex rhythmic structures from simple rules
- **Music education** — visualize and explore polyrhythms interactively
- **Game design** — rhythm-based mechanics with polyrhythmic patterns
- **Mathematics education** — LCM, GCD, and Euclidean algorithms made tangible
- **Dance choreography** — coordinate multiple dancers with different step patterns

## See Also

- **ternary-rhythm** — single rhythm pattern analysis
- **ternary-phase** — phase relationships between overlapping rhythms
- **ternary-fib** — period-8 as a natural polyrhythmic foundation
- **ternary-tempo** — speed relationships between polyrhythmic layers
- **ternary-fib** — Fibonacci rhythms create natural polyrhythms with period ratios

## Install

```bash
cargo add ternary-polyrhythm
```

## License

MIT
