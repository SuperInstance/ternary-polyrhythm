# ternary-polyrhythm

**Two rhythms at once. The tension between them IS the music.**

A polyrhythm is two (or more) rhythmic patterns playing simultaneously with different period lengths. 3 against 4. 5 against 7. The tension between the competing pulse rates is the *entire point* — it's what makes West African drumming, gamelan, and math rock feel alive. Your brain can't predict where both patterns will land, so it stays engaged, trying to resolve the irreconcilable.

This crate computes polyrhythms for ternary hit patterns. Given N and M, it generates both rhythms, finds where they coincide (the "downbeat"), computes the full cycle length (LCM), and gives you tools to layer, shift, and analyze polyrhythmic structures.

## What's Inside

- **`Rhythm`** — a hit pattern: cycle length + set of hit positions
- **`Polyrhythm`** — N against M. Generates both patterns, computes cycle length
- **`coincidence(n, m)`** — where do both patterns hit simultaneously? The "one"
- **`euclidean(k, n)`** — distribute k hits evenly in n steps (Björklund's algorithm)
- **`shift(rhythm, offset)`** — shift a rhythm forward, creating phase offset
- **`layer(rhythms)`** — combine multiple rhythms into one composite pattern
- **`gcd(a, b)`** / **`lcm(a, m)`** — the arithmetic underneath polyrhythms
- **`is_coprime(a, b)`** — coprime polyrhythms are the most interesting (irresolvable tension)

## Quick Example

```rust
use ternary_polyrhythm::*;

// Classic 3:2 polyrhythm
let poly = Polyrhythm { n: 3, m: 2 };
assert_eq!(poly.cycle_length(), 6); // LCM(3, 2)

// Where do they coincide?
let coincidences = coincidence(3, 2);
// [0] — only at the downbeat

// Layer two patterns
let r1 = Rhythm::new(3, vec![0, 1]);
let r2 = Rhythm::new(4, vec![0, 2]);
let composite = layer(&[r1, r2]);

// Euclidean: 5 in 8 (bossa nova)
let bossa = euclidean(5, 8);
// [0, 1, 3, 5, 6] — the classic pattern
```

## The Deeper Truth

**Polyrhythms are audible number theory.** The cycle length is the LCM. The coincidence points follow the GCD. The tension is the ratio N/M — which is irrational when N and M are coprime. Irrational ratios mean the patterns *never fully resolve*. They cycle, but they don't repeat in a way your brain can lock onto. This is why coprime polyrhythms (3:2, 5:3, 7:4) feel more alive than simple ratios (4:2 = 2:1 = octave).

In ternary, polyrhythmic hits carry {-1, 0, +1} values, and the composite is a ternary sum. This creates interference: where both patterns hit +1, the sum wraps to -1 (mod 3). The "collision" of two positive hits creates a negative value — which sounds like an accent that *inverts*. This ternary interference is a genuinely new musical phenomenon that doesn't exist in binary (hit/no-hit) rhythm systems.

**Use cases:**
- **Algorithmic music** — generate complex rhythms from simple number-theoretic rules
- **Music education** — visualize and explore polyrhythms interactively
- **Game design** — rhythm-based mechanics with polyrhythmic patterns
- **Math education** — LCM, GCD, and Euclidean algorithms made tangible
- **Dance** — coordinate multiple dancers with different step patterns

## See Also

- **ternary-rhythm** — single rhythm pattern analysis
- **ternary-phase** — phase relationships between overlapping rhythms
- **ternary-fib** — Fibonacci rhythms create natural polyrhythms
- **ternary-tempo** — speed relationships between polyrhythmic layers
- **ternary-harmonic** — the frequency-domain analog (polyrhythms in time = harmonics in frequency)
- **ternary-jam** — polyrhythmic improvisation

## Install

```bash
cargo add ternary-polyrhythm
```

## License

MIT
