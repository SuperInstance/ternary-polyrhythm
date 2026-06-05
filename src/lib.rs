//! # ternary-polyrhythm
//! Multiple simultaneous rhythmic patterns with ternary support.

#![forbid(unsafe_code)]

use std::collections::HashSet;

/// Compute GCD of two positive integers.
fn gcd(a: usize, b: usize) -> usize {
    let mut a = a;
    let mut b = b;
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

/// Compute LCM of two positive integers.
fn lcm(a: usize, b: usize) -> usize {
    if a == 0 || b == 0 { return 0; }
    a / gcd(a, b) * b
}

/// A rhythm pattern: which beats (0-indexed) have hits in a cycle of `length` beats.
#[derive(Debug, Clone)]
pub struct Rhythm {
    pub length: usize,
    pub hits: HashSet<usize>,
}

impl Rhythm {
    pub fn new(length: usize, hits: Vec<usize>) -> Self {
        Self { length, hits: hits.into_iter().collect() }
    }

    /// Does beat `i` have a hit?
    pub fn hit_at(&self, i: usize) -> bool {
        self.hits.contains(&(i % self.length))
    }
}

/// A polyrhythm: N against M.
#[derive(Debug, Clone)]
pub struct Polyrhythm {
    pub n: usize,
    pub m: usize,
}

impl Polyrhythm {
    pub fn new(n: usize, m: usize) -> Self {
        Self { n, m }
    }

    /// LCM sync point: total beats before both cycles align.
    pub fn sync_length(&self) -> usize {
        lcm(self.n, self.m)
    }

    /// Generate the two constituent rhythms.
    pub fn to_rhythms(&self) -> (Rhythm, Rhythm) {
        let r1 = Rhythm::new(self.n, (0..self.n).map(|i| i * lcm(self.n, self.m) / self.n).collect());
        let r2 = Rhythm::new(self.m, (0..self.m).map(|i| i * lcm(self.n, self.m) / self.m).collect());
        (r1, r2)
    }

    /// Combined pattern over one sync cycle: which beats have at least one hit.
    pub fn combined_hits(&self) -> HashSet<usize> {
        let total = self.sync_length();
        let mut hits = HashSet::new();
        for i in 0..self.n {
            hits.insert(i * total / self.n);
        }
        for i in 0..self.m {
            hits.insert(i * total / self.m);
        }
        hits
    }
}

/// Find the LCM sync point for N-against-M.
pub fn polyrhythm_lcm(n: usize, m: usize) -> usize {
    lcm(n, m)
}

/// Stack multiple rhythm patterns and return the combined hit set over the LCM of all lengths.
pub fn layered_patterns(rhythms: &[Rhythm]) -> HashSet<usize> {
    if rhythms.is_empty() { return HashSet::new(); }
    let total: usize = rhythms.iter().map(|r| r.length).fold(1, lcm);
    let mut hits = HashSet::new();
    for r in rhythms {
        for i in 0..total {
            if r.hit_at(i) {
                hits.insert(i);
            }
        }
    }
    hits
}

/// Generate a classic cross-rhythm (e.g., 3:2, 3:4).
pub fn cross_rhythm(a: usize, b: usize) -> Polyrhythm {
    Polyrhythm::new(a, b)
}

/// Björklund / Euclidean rhythm algorithm: distribute `k` pulses as evenly as possible in `n` slots.
/// Returns a vector of booleans.
pub fn euclidean_rhythm(k: usize, n: usize) -> Vec<bool> {
    if n == 0 { return vec![]; }
    if k == 0 { return vec![false; n]; }
    if k >= n { return vec![true; n]; }

    let mut pattern: Vec<Vec<bool>> = (0..n).map(|i| vec![i < k]).collect();
    let mut remainders: Vec<Vec<bool>> = vec![];

    // First pass: pair [1] and [0]
    let ones_count = k;
    let zeros_count = n - k;
    
    // Simple approach: distribute using remainder-based method
    let mut result = vec![false; n];
    let mut pos = 0;
    for _ in 0..k {
        result[pos % n] = true;
        pos += n / k;
        if pos >= n { pos = pos % n; }
    }
    
    // Use proper Björklund algorithm
    let mut bucket: Vec<f64> = vec![0.0; n];
    let mut out = vec![false; n];
    for i in 0..k {
        let idx = (i as f64 * n as f64 / k as f64).floor() as usize;
        if idx < n {
            out[idx] = true;
        }
    }
    out
}

/// A polymeter: two patterns with different time signatures running simultaneously.
#[derive(Debug, Clone)]
pub struct Polymeter {
    pub sig_a: usize,
    pub sig_b: usize,
}

impl Polymeter {
    pub fn new(sig_a: usize, sig_b: usize) -> Self {
        Self { sig_a, sig_b }
    }

    /// Beat at which both meters align on beat 1.
    pub fn sync_point(&self) -> usize {
        lcm(self.sig_a, self.sig_b)
    }
}

/// When all given patterns align (LCM of their lengths).
pub fn sync_point(lengths: &[usize]) -> usize {
    lengths.iter().fold(1, |acc, &l| lcm(acc, l))
}

/// Compute rhythmic density: hits per beat over a cycle.
pub fn rhythmic_density(hits: &HashSet<usize>, cycle_length: usize) -> f64 {
    if cycle_length == 0 { return 0.0; }
    hits.len() as f64 / cycle_length as f64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gcd() {
        assert_eq!(gcd(12, 8), 4);
    }

    #[test]
    fn test_lcm_basic() {
        assert_eq!(lcm(3, 4), 12);
    }

    #[test]
    fn test_polyrhythm_lcm() {
        assert_eq!(polyrhythm_lcm(3, 4), 12);
    }

    #[test]
    fn test_polyrhythm_lcm_same() {
        assert_eq!(polyrhythm_lcm(4, 4), 4);
    }

    #[test]
    fn test_polyrhythm_sync_length() {
        let pr = Polyrhythm::new(3, 4);
        assert_eq!(pr.sync_length(), 12);
    }

    #[test]
    fn test_polyrhythm_combined_hits() {
        let pr = Polyrhythm::new(2, 3);
        let hits = pr.combined_hits();
        assert_eq!(hits.len(), 4); // 0 + 2 from 2, 0 + 3 from 3, but 0 overlaps
        assert!(hits.contains(&0));
    }

    #[test]
    fn test_rhythm_hit_at() {
        let r = Rhythm::new(4, vec![0, 2]);
        assert!(r.hit_at(0));
        assert!(!r.hit_at(1));
        assert!(r.hit_at(2));
    }

    #[test]
    fn test_rhythm_hit_at_wraps() {
        let r = Rhythm::new(4, vec![0]);
        assert!(r.hit_at(4));
    }

    #[test]
    fn test_layered_patterns_empty() {
        let hits = layered_patterns(&[]);
        assert!(hits.is_empty());
    }

    #[test]
    fn test_layered_patterns_combined() {
        let r1 = Rhythm::new(3, vec![0]);
        let r2 = Rhythm::new(4, vec![0]);
        let hits = layered_patterns(&[r1, r2]);
        assert!(hits.contains(&0));
    }

    #[test]
    fn test_cross_rhythm() {
        let cr = cross_rhythm(3, 2);
        assert_eq!(cr.sync_length(), 6);
    }

    #[test]
    fn test_euclidean_rhythm_basic() {
        let er = euclidean_rhythm(3, 8);
        assert_eq!(er.len(), 8);
        assert_eq!(er.iter().filter(|&&b| b).count(), 3);
    }

    #[test]
    fn test_euclidean_rhythm_zero() {
        let er = euclidean_rhythm(0, 4);
        assert!(er.iter().all(|&b| !b));
    }

    #[test]
    fn test_polymeter_sync() {
        let pm = Polymeter::new(3, 4);
        assert_eq!(pm.sync_point(), 12);
    }

    #[test]
    fn test_sync_point_multiple() {
        let sp = sync_point(&[3, 4, 5]);
        assert_eq!(sp, 60);
    }

    #[test]
    fn test_rhythmic_density() {
        let hits: HashSet<usize> = vec![0, 2, 4, 6].into_iter().collect();
        let d = rhythmic_density(&hits, 8);
        assert!((d - 0.5).abs() < 1e-10);
    }

    #[test]
    fn test_rhythmic_density_zero() {
        let hits: HashSet<usize> = [0].into_iter().collect();
        assert_eq!(rhythmic_density(&hits, 0), 0.0);
    }
}
