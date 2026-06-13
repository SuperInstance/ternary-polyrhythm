# ternary-polyrhythm

Polyrhythmic pattern generation with ternary structure. Multiple simultaneous rhythmic cycles, LCM sync points, Euclidean (Björklund) rhythm generation, layered patterns, polymeters, and rhythmic density analysis.

## Why It Matters

A polyrhythm is the simultaneous presentation of two or more conflicting rhythmic patterns — e.g., 3 beats against 4. The tension between the patterns creates the musical feel of artists like Ligeti, Meshuggah, and West African drumming traditions. In ternary agent systems, polyrhythms model:

- **Multi-period agent scheduling**: agents with different cycle lengths running concurrently
- **Phase synchronization points**: when do N-period and M-period cycles align?
- **Diversity metrics**: rhythmic density measures how saturated a pattern is
- **Euclidean rhythm generation**: maximally even patterns (the "God's algorithm" of rhythm)

The mathematical structure is pure number theory: GCD, LCM, and modular arithmetic.

## How It Works

### LCM and GCD

For two cycle lengths $n$ and $m$:

$$\gcd(n, m) = \text{Euclidean algorithm}, \quad \text{lcm}(n, m) = \frac{n \cdot m}{\gcd(n, m)}$$

The **sync point** — when both rhythms return to beat 1 simultaneously — is the LCM.

**Example:** polyrhythm 3:4 syncs every $\text{lcm}(3,4) = 12$ beats.

**Complexity:** O($\log(\min(n,m))$) for GCD via Euclidean algorithm.

### Combined Hit Pattern

For a polyrhythm of $n$ against $m$ over one sync cycle of length $L = \text{lcm}(n,m)$:

$$\text{hits}_n = \left\{ i \cdot \frac{L}{n} : i = 0, 1, \ldots, n-1 \right\}$$
$$\text{hits}_m = \left\{ j \cdot \frac{L}{m} : j = 0, 1, \ldots, m-1 \right\}$$
$$\text{combined} = \text{hits}_n \cup \text{hits}_m$$

Total unique hits: $n + m - |\text{hits}_n \cap \text{hits}_m|$.

### Euclidean Rhythm (Björklund Algorithm)

Distribute $k$ pulses as evenly as possible in $n$ slots:

$$\text{hit}_i = \left\lfloor \frac{i \cdot n}{k} \right\rfloor \neq \left\lfloor \frac{(i+1) \cdot n}{k} \right\rfloor$$

Or equivalently, place pulses at positions $\lfloor i \cdot n/k \rfloor$ for $i = 0, \ldots, k-1$.

**Properties:**
- **Maximally even**: gaps between hits differ by at most 1
- **Rotation-equivalent**: all rotations produce the same rhythmic quality
- **Cultural universality**: generates virtually all traditional world music rhythms

**Example:** $E(3, 8) = [1, 0, 0, 1, 0, 0, 1, 0]$ — the Cuban tresillo.

### Layered Patterns

For $k$ rhythms with lengths $l_1, l_2, \ldots, l_k$, the combined cycle is:

$$L_{\text{total}} = \text{lcm}(l_1, l_2, \ldots, l_k)$$

A hit occurs at beat $b$ if **any** rhythm has a hit at position $b \bmod l_i$.

### Polymeter

Two time signatures running simultaneously. The sync point is $\text{lcm}(\text{sig}_A, \text{sig}_B)$.

### Rhythmic Density

$$\rho = \frac{|\text{hits}|}{\text{cycle length}}$$

- $\rho = 0$: silence
- $\rho = 1$: every beat is a hit (rolls)
- $\rho = 0.5$: half-time feel

## Quick Start

```rust
use ternary_polyrhythm::*;

// Polyrhythm 3:4 — syncs every 12 beats
let pr = Polyrhythm::new(3, 4);
assert_eq!(pr.sync_length(), 12);
let hits = pr.combined_hits(); // unique beat positions with hits

// Layered rhythms
let r1 = Rhythm::new(3, vec![0]);       // every 3rd beat
let r2 = Rhythm::new(4, vec![0]);       // every 4th beat
let combined = layered_patterns(&[r1, r2]); // over lcm(3,4)=12 beats

// Euclidean rhythm: 3 pulses in 8 slots
let er = euclidean_rhythm(3, 8);
assert_eq!(er.iter().filter(|&&b| b).count(), 3);
// [true, false, false, true, false, false, true, false] — tresillo!

// Cross-rhythm
let cr = cross_rhythm(3, 2);
assert_eq!(cr.sync_length(), 6);

// Polymeter 3/4 × 4/4
let pm = Polymeter::new(3, 4);
assert_eq!(pm.sync_point(), 12);

// Sync point for multiple patterns
let sp = sync_point(&[3, 4, 5]);
assert_eq!(sp, 60); // lcm(3,4,5)

// Rhythmic density
let hits: HashSet<usize> = vec![0, 2, 4, 6].into_iter().collect();
assert!((rhythmic_density(&hits, 8) - 0.5).abs() < 1e-10);
```

## API

| Type / Function | Description |
|---|---|
| `Polyrhythm::new(n, m)` | N-against-M polyrhythm |
| `.sync_length()` | LCM sync point |
| `.combined_hits()` | All unique hit positions over one cycle |
| `Rhythm::new(length, hits)` | Single rhythm pattern |
| `.hit_at(i) → bool` | Does beat $i$ have a hit? (wraps modulo length) |
| `layered_patterns(rhythms)` | Union of hits over LCM of all lengths |
| `euclidean_rhythm(k, n)` | Björklund algorithm: $k$ pulses in $n$ slots |
| `cross_rhythm(a, b)` | Classical cross-rhythm (alias for Polyrhythm) |
| `Polymeter::new(sig_a, sig_b)` | Conflicting time signatures |
| `.sync_point()` | Beat where both meters align on beat 1 |
| `sync_point(lengths)` | LCM of multiple cycle lengths |
| `rhythmic_density(hits, length)` | Fill ratio ρ ∈ [0, 1] |
| `polyrhythm_lcm(n, m)` | Convenience: LCM of two cycle lengths |

## Architecture Notes

Polyrhythms instantiate the **γ + η = C** identity in the time domain. Each rhythm cycle is a periodic allocation of "active" beats (γ — constructive impulses) and "rest" beats (η — inhibitory gaps). The total conserved quantity $C$ is the cycle length — fixed for each rhythm, regardless of how many hits it contains.

When two rhythms layer, their combined pattern represents the **interference** of two conserved cycles. The LCM sync point is the temporal analogue of the conservation boundary $C$: it's the shortest time at which both cycles have completed an integer number of revolutions, returning the system to its initial state. Denser rhythms (higher $\rho$) spend more of their $C$-budget on γ-beats; sparser rhythms conserve more η-rests. The Euclidean algorithm finds the unique distribution that minimizes the variance of inter-onset intervals — the most "balanced" allocation of γ and η within a fixed $C$.

## References

- Toussaint, G. T. (2005). *The Geometry of Musical Rhythm.* Springer.
- Björklund, E. (2003). *The Theory of Rep-Rate Pattern Generation in the Spallation Neutron Source.* (Original algorithm)
- London, J. (2012). *Hearing in Time.* 2nd ed. Oxford University Press.
- Wright, O. & Toussaint, G. T. (2006). *The Rhythmic Tiling Problem.* CIRM.

## License

MIT
