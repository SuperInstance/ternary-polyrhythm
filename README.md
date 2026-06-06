# ternary-polyrhythm

Multiple simultaneous rhythmic patterns with ternary support — Euclidean rhythm generation, cross-rhythms, polymeters, LCM synchronization, and layered density analysis.

## Background

Polyrhythm — the simultaneous use of contrasting rhythmic patterns — is one of the most sophisticated achievements of human musical culture. West African Ewe drumming layers bells, rattles, and drums in patterns of 3, 4, 6, 7, and 12 beats, creating a dense interlocking texture where the "groove" emerges from the interaction of parts rather than residing in any single pattern.

The key mathematical insight is synchronization: two rhythms of lengths N and M align every LCM(N, M) beats. A 3-against-4 pattern repeats every 12 beats. A 3-against-4-against-5 pattern repeats every 60 beats. The point where all patterns simultaneously land on beat 1 is the "sync point" — the rhythmic equivalent of a harmonic resolution.

`ternary-polyrhythm` formalizes this mathematics for ternary-valued rhythm patterns, combining it with Björklund's Euclidean rhythm algorithm for optimal pulse distribution.

## How It Works

### Rhythm (Hit Set Representation)

Unlike `ternary-rhythm` which uses a sequence of ternary values, this crate represents rhythms as a `HashSet` of hit positions within a cycle of given length. This allows efficient membership testing (`hit_at(i)`) and set-theoretic combination of patterns.

### Polyrhythm (N-against-M)

A `Polyrhythm` pairs two rhythms of lengths N and M:

- **`sync_length()`** — LCM(N, M): total beats before both cycles align
- **`to_rhythms()`** — decompose into two constituent rhythm patterns
- **`combined_hits()`** — union of both hit sets over the sync cycle

For example, a 3:2 polyrhythm in a 6-beat cycle:
- Pattern A: hits at {0, 2, 4} (every 2 beats)
- Pattern B: hits at {0, 3} (every 3 beats)
- Combined: {0, 2, 3, 4}

### Euclidean Rhythm (Björklund Algorithm)

The `euclidean_rhythm(k, n)` function distributes `k` pulses as evenly as possible among `n` slots. This produces musically important patterns:

| k | n | Pattern | Traditional Name |
|---|---|---------|-----------------|
| 2 | 3 | [1,0,1] | Triplet pulse   |
| 3 | 8 | [1,0,1,0,1,0,0,0] | Cuban tresillo |
| 4 | 9 | [1,0,1,0,1,0,1,0,0] | Aksak          |
| 5 | 8 | [1,1,0,1,1,0,1,0] | York santí     |

The algorithm uses a floor-based distribution to achieve maximal evenness.

### Polymeter

A `Polymeter` represents two time signatures running simultaneously (e.g., 3/4 against 4/4). The `sync_point()` is the LCM of the two signatures — the beat where both meters simultaneously reach their downbeat.

### Layered Patterns

`layered_patterns()` takes multiple rhythms and computes the combined hit set over the LCM of all their lengths. This generalizes polyrhythm to arbitrary numbers of simultaneous patterns.

### Rhythmic Density

`rhythmic_density(hits, cycle_length)` computes the fraction of beats with hits. A density of 0.5 means half the positions are struck.

## Experimental Results

- **Euclidean rhythms produce traditional patterns.** The 3-in-8 Euclidean rhythm matches the Cuban tresillo exactly. The 5-in-8 matches the York santí pattern. This confirms Toussaint's (2005) finding that many traditional rhythms are Euclidean.
- **Sync points grow rapidly with prime-length rhythms.** A 3:4 polyrhythm syncs at 12 beats. A 3:4:5 polyrhythm syncs at 60 beats. Adding a length-7 rhythm pushes the sync to 420 beats — at 120 BPM, that's 3.5 minutes before the full cycle repeats.
- **Combined hit density increases sub-linearly.** Layering two patterns of density 0.5 produces combined density of ~0.75 (not 1.0), because some hits overlap. This overlap is musically important: it creates naturally accented positions.
- **Cross-rhythms create phantom patterns.** A 3:2 cross-rhythm with combined hits {0, 2, 3, 4} has gaps at positions 1 and 5. The perceptual system "hears" these gaps as a hidden 2-beat pattern — the phantom downbeat effect well-known in West African drumming.

## Impact

`ternary-polyrhythm` demonstrates that the mathematics of polyrhythm — GCD, LCM, set intersection and union — are independent of the representational richness of the rhythm patterns. Binary (hit/rest) or ternary (strong/medium/weak), the synchronization points and density properties are the same.

The crate provides the computational foundation for understanding how simple periodic patterns combine to produce complex emergent textures — a principle that applies equally to music, distributed systems, and multi-agent coordination.

## Use Cases

1. **Traditional music analysis** — Decompose West African, Balinese, and Indian rhythmic patterns into their polyrhythmic constituents and analyze their sync structure.
2. **Generative percussion** — Layer Euclidean rhythms of different lengths to produce automatically evolving percussion patterns with mathematically guaranteed periodicity.
3. **Distributed system synchronization** — Model multi-process coordination as polyrhythmic synchronization, using LCM to predict alignment points.
4. **Interactive music systems** — Build real-time polyrhythmic engines where users can add/remove pattern layers and see the combined rhythm evolve.

## Open Questions

1. **Euclidean optimality in ternary.** The current Euclidean algorithm distributes binary pulses (hit/rest). Can it be extended to distribute ternary values (strong/medium/weak) while maintaining evenness?
2. **Non-integer polymeter.** What happens with irrational meter ratios (e.g., 3 against π)? This leads to quasi-periodic rhythms that never exactly repeat — a topic in spectral music.
3. **Weighted sync points.** Not all sync points are musically equal. Could a metric quantify which sync points are "strong" (many voices aligning) versus "weak" (few voices)?

## Connection to Oxide Stack

`ternary-polyrhythm` extends `ternary-rhythm`'s polyrhythmic framework with set-based operations and Euclidean generation. Its LCM synchronization model connects to `ternary-tidelight`'s `TideClock` for fleet-wide temporal coordination. The rhythmic density analysis feeds `ternary-tempo`'s BPM estimation. The cross-rhythm detection complements `ternary-ear`'s pattern recognition capabilities.
