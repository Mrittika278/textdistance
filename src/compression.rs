use std::collections::HashMap;
use std::f64;
use itertools::Itertools;
use bzip2::write::BzEncoder;
use bzip2::Compression as BzCompression;

use xz2::write::XzEncoder;

use flate2::{
    Compression,
    write::ZlibEncoder,
};

use std::io::Write;


/// Trait for all NCD algorithms.
pub trait NCD {
    /// Return the "compressed size".
    fn compressed_size(&self, data: &str) -> f64;

    /// NCD formula.
    fn distance(&self, strings: &[&str]) -> f64 {
        if strings.is_empty() {
            return 0.0;
        }

        let compressed: Vec<f64> = strings
            .iter()
            .map(|s| self.compressed_size(s))
            .collect();

        let max = compressed
            .iter()
            .copied()
            .fold(f64::NEG_INFINITY, f64::max);

        if max == 0.0 {
            return 0.0;
        }

        let min = compressed
            .iter()
            .copied()
            .fold(f64::INFINITY, f64::min);

        let mut concat = f64::INFINITY;

        for perm in strings.iter().permutations(strings.len()) {
            let merged = perm.into_iter().copied().collect::<String>();
            let size = self.compressed_size(&merged);

            if size < concat {
                concat = size;
            }
        }

        (concat - min * ((strings.len() - 1) as f64)) / max
    }
}









pub struct RLENCD;

impl RLENCD {
    fn encode(data: &str) -> String {
        if data.is_empty() {
            return String::new();
        }

        let chars: Vec<char> = data.chars().collect();

        let mut out = String::new();

        let mut i = 0;

        while i < chars.len() {
            let ch = chars[i];

            let mut count = 1;

            while i + count < chars.len()
                && chars[i + count] == ch
            {
                count += 1;
            }

            match count {
                1 => out.push(ch),

                2 => {
                    out.push(ch);
                    out.push(ch);
                }

                _ => {
                    out.push_str(&count.to_string());
                    out.push(ch);
                }
            }

            i += count;
        }

        out
    }
}

impl NCD for RLENCD {
    fn compressed_size(&self, data: &str) -> f64 {
        Self::encode(data).len() as f64
    }
}









pub struct BWTRLENCD {
    pub terminator: char,
}

impl Default for BWTRLENCD {
    fn default() -> Self {
        Self {
            terminator: '\0',
        }
    }
}

impl BWTRLENCD {
    fn bwt(&self, text: &str) -> String {
        if text.is_empty() {
            return self.terminator.to_string();
        }

        if text.contains(self.terminator) {
            return text.to_string();
        }

        let mut s = text.to_string();

        s.push(self.terminator);

        let chars: Vec<char> = s.chars().collect();

        let n = chars.len();

        let mut rotations = Vec::with_capacity(n);

        for i in 0..n {
            let mut r = String::new();

            for j in 0..n {
                r.push(chars[(i + j) % n]);
            }

            rotations.push(r);
        }

        rotations.sort();

        let mut last = String::new();

        for r in rotations {
            last.push(r.chars().last().unwrap());
        }

        last
    }
}

impl NCD for BWTRLENCD {
    fn compressed_size(&self, data: &str) -> f64 {
        let transformed = self.bwt(data);

        RLENCD::encode(&transformed).len() as f64
    }
}









#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_rle() {

        let alg = RLENCD;

        assert_eq!(RLENCD::encode(""), "");

        assert_eq!(RLENCD::encode("aa"), "aa");

        assert_eq!(RLENCD::encode("aaa"), "3a");

        assert_eq!(RLENCD::encode("aaabbb"), "3a3b");

        assert!(alg.distance(&["test", "test"]) <=
                alg.distance(&["test", "text"]));
    }

    #[test]
    fn test_bwt() {

        let alg = BWTRLENCD::default();

        let out = alg.bwt("banana");

        assert!(!out.is_empty());

        assert!(alg.distance(&["banana", "banana"])
            <= alg.distance(&["banana", "orange"]));
    }
}
// ============================================================
// SQRT NCD
// ============================================================

pub struct SqrtNCD;

impl SqrtNCD {
    fn compressed_size(data: &str) -> f64 {
        let mut counts: HashMap<char, usize> = HashMap::new();

        for c in data.chars() {
            *counts.entry(c).or_insert(0) += 1;
        }

        counts
            .values()
            .map(|v| (*v as f64).sqrt())
            .sum()
    }

    pub fn distance(&self, a: &str, b: &str) -> f64 {
        if a.is_empty() && b.is_empty() {
            return 0.0;
        }

        let ca = Self::compressed_size(a);
        let cb = Self::compressed_size(b);

        let mut concat = String::new();
        concat.push_str(a);
        concat.push_str(b);

        let cab = Self::compressed_size(&concat);

        (cab - ca.min(cb)) / ca.max(cb)
    }
}

// ============================================================
// ENTROPY NCD
// ============================================================

pub struct EntropyNCD {
    pub coef: f64,
    pub base: f64,
}

impl Default for EntropyNCD {
    fn default() -> Self {
        Self {
            coef: 1.0,
            base: 2.0,
        }
    }
}

impl EntropyNCD {
    fn entropy(&self, data: &str) -> f64 {
        if data.is_empty() {
            return 0.0;
        }

        let mut counts: HashMap<char, usize> = HashMap::new();

        for c in data.chars() {
            *counts.entry(c).or_insert(0) += 1;
        }

        let total = data.chars().count() as f64;

        let mut entropy = 0.0;

        for count in counts.values() {
            let p = *count as f64 / total;
            entropy -= p * (p.ln() / self.base.ln());
        }

        entropy
    }

    fn compressed_size(&self, data: &str) -> f64 {
        self.coef + self.entropy(data)
    }

    pub fn distance(&self, a: &str, b: &str) -> f64 {
        if a.is_empty() && b.is_empty() {
            return 0.0;
        }

        let ca = self.compressed_size(a);
        let cb = self.compressed_size(b);

        let mut concat = String::new();
        concat.push_str(a);
        concat.push_str(b);

        let cab = self.compressed_size(&concat);

        (cab - ca.min(cb)) / ca.max(cb)
    }
}

// ============================================================
// TESTS
// ============================================================

#[cfg(test)]
mod compression_tests {
    use super::*;

    #[test]
    fn test_sqrt_ncd() {
        let alg = SqrtNCD;

        assert_eq!(alg.distance("", ""), 0.0);

        let d = alg.distance("abc", "abc");
        assert!(d >= 0.0);
    }

    #[test]
    fn test_entropy_ncd() {
        let alg = EntropyNCD::default();

        assert_eq!(alg.distance("", ""), 0.0);

        let d = alg.distance("aaaa", "bbbb");
        assert!(d >= 0.0);
    }
}
// ============================================================
// ZLIB NCD
// ============================================================

pub struct ZLIBNCD;

impl ZLIBNCD {
    fn compressed_size(data: &str) -> usize {
        let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());

        encoder.write_all(data.as_bytes()).unwrap();

        let compressed = encoder.finish().unwrap();

        // Python:
        // codecs.encode(data, "zlib_codec")[2:]
        compressed.len().saturating_sub(2)
    }

    pub fn distance(&self, a: &str, b: &str) -> f64 {
        if a.is_empty() && b.is_empty() {
            return 0.0;
        }

        let ca = Self::compressed_size(a) as f64;
        let cb = Self::compressed_size(b) as f64;

        let mut concat = String::new();
        concat.push_str(a);
        concat.push_str(b);

        let cab = Self::compressed_size(&concat) as f64;

        if ca.max(cb) == 0.0 {
            return 0.0;
        }

        (cab - ca.min(cb)) / ca.max(cb)
    }
}



// ============================================================
// BZ2 NCD
// ============================================================

pub struct BZ2NCD;

impl BZ2NCD {
    fn compressed_size(data: &str) -> usize {
        let mut encoder =
            BzEncoder::new(Vec::new(), BzCompression::default());

        encoder.write_all(data.as_bytes()).unwrap();

        let compressed = encoder.finish().unwrap();

        // Match Python slicing [15:]
        compressed.len().saturating_sub(15)
    }

    pub fn distance(&self, a: &str, b: &str) -> f64 {
        if a.is_empty() && b.is_empty() {
            return 0.0;
        }

        let ca = Self::compressed_size(a) as f64;
        let cb = Self::compressed_size(b) as f64;

        let mut concat = String::new();
        concat.push_str(a);
        concat.push_str(b);

        let cab = Self::compressed_size(&concat) as f64;

        if ca.max(cb) == 0.0 {
            return 0.0;
        }

        (cab - ca.min(cb)) / ca.max(cb)
    }
}
// ============================================================
// LZMA NCD
// ============================================================

pub struct LZMANCD;

impl LZMANCD {
    fn compressed_size(data: &str) -> usize {
        let mut encoder = XzEncoder::new(Vec::new(), 6);

        encoder.write_all(data.as_bytes()).unwrap();

        let compressed = encoder.finish().unwrap();

        // Match Python slicing [14:]
        compressed.len().saturating_sub(14)
    }

    pub fn distance(&self, a: &str, b: &str) -> f64 {
        if a.is_empty() && b.is_empty() {
            return 0.0;
        }

        let ca = Self::compressed_size(a) as f64;
        let cb = Self::compressed_size(b) as f64;

        let mut concat = String::new();
        concat.push_str(a);
        concat.push_str(b);

        let cab = Self::compressed_size(&concat) as f64;

        if ca.max(cb) == 0.0 {
            return 0.0;
        }

        (cab - ca.min(cb)) / ca.max(cb)
    }
}
// ============================================================
// ARITHMETIC CODING NCD
// ============================================================
//
// Faithful port of textdistance's `ArithNCD`
// (textdistance/algorithms/compression_based.py).
//
// Python's implementation uses `fractions.Fraction`, which is exact
// (arbitrary-precision) and always kept in reduced form. To preserve
// that exactly — rather than silently switching to lossy floats —
// this port uses `num_rational::BigRational` (a `Ratio<BigInt>`),
// which reduces on construction the same way `Fraction` does.

use num_bigint::BigInt;
use num_rational::BigRational;
use num_traits::{One, ToPrimitive, Zero};

pub struct ArithNCD {
    /// Logarithm base used by `_get_size` to turn the final numerator
    /// into a "compressed size". Python default: 2.
    pub base: f64,
    /// Optional terminator character. Python default: None.
    pub terminator: Option<char>,
    /// q-gram size. Only qval == 1 (character-level) is implemented,
    /// matching every other algorithm already in this port.
    pub qval: usize,
}

impl Default for ArithNCD {
    fn default() -> Self {
        Self {
            base: 2.0,
            terminator: None,
            qval: 1,
        }
    }
}

impl ArithNCD {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_base(base: f64) -> Self {
        Self { base, ..Self::default() }
    }

    pub fn with_terminator(terminator: char) -> Self {
        Self { terminator: Some(terminator), ..Self::default() }
    }

    /// Port of `_make_probs`.
    ///
    /// Ordering matters here: Python builds this from
    /// `Counter.most_common()`, which sorts by descending count and,
    /// for ties, falls back to insertion order (Python dicts/Counters
    /// preserve first-occurrence order, and `sorted()` is stable).
    /// We reproduce that by tracking first-appearance order explicitly
    /// and doing a stable sort by count.
    fn make_probs(&self, data: &str) -> HashMap<char, (BigRational, BigRational)> {
        let mut order: Vec<char> = Vec::new();
        let mut counts: HashMap<char, i64> = HashMap::new();

        for c in data.chars() {
            let counter = counts.entry(c).or_insert(0);
            if *counter == 0 {
                order.push(c);
            }
            *counter += 1;
        }

        if let Some(term) = self.terminator {
            // Python: `counts[self.terminator] = 1` — this OVERWRITES
            // any existing count for the terminator char, it does not
            // add to it. If the char is new, it's appended at the end
            // of iteration order, exactly like a fresh dict key.
            if !counts.contains_key(&term) {
                order.push(term);
            }
            counts.insert(term, 1);
        }

        let total_letters: i64 = counts.values().sum();

        let mut items: Vec<(char, i64)> = order.iter().map(|&c| (c, counts[&c])).collect();
        items.sort_by(|a, b| b.1.cmp(&a.1)); // stable: ties keep `order`

        let mut prob_pairs = HashMap::new();
        let mut cumulative: i64 = 0;
        for (ch, count) in items {
            let start = BigRational::new(BigInt::from(cumulative), BigInt::from(total_letters));
            let width = BigRational::new(BigInt::from(count), BigInt::from(total_letters));
            prob_pairs.insert(ch, (start, width));
            cumulative += count;
        }
        debug_assert_eq!(cumulative, total_letters);

        prob_pairs
    }

    /// Port of `_get_range`.
    fn get_range(
        &self,
        data: &str,
        probs: &HashMap<char, (BigRational, BigRational)>,
    ) -> (BigRational, BigRational) {
        let mut chars: Vec<char> = data.chars().collect();

        if let Some(term) = self.terminator {
            // Python: strip any existing terminator occurrences, then
            // append exactly one at the end.
            chars.retain(|&c| c != term);
            chars.push(term);
        }

        let mut start = BigRational::zero();
        let mut width = BigRational::one();

        for ch in chars {
            let (prob_start, prob_width) = &probs[&ch];
            start += prob_start * &width;
            width *= prob_width;
        }

        let end = &start + &width;
        (start, end)
    }

    /// Port of `_compress`. Returns the exact arithmetic-coded fraction.
    fn compress(&self, data: &str) -> BigRational {
        let probs = self.make_probs(data);
        let (start, end) = self.get_range(data, &probs);

        let mut output_fraction = BigRational::zero();
        let mut output_denominator = BigInt::one();

        while !(start <= output_fraction && output_fraction < end) {
            let output_numerator =
                BigInt::one() + (start.numer() * &output_denominator) / start.denom();
            output_fraction = BigRational::new(output_numerator, output_denominator.clone());
            output_denominator *= 2;
        }

        output_fraction
    }
}

impl NCD for ArithNCD {
    /// Port of `_get_size`.
    fn compressed_size(&self, data: &str) -> f64 {
        let fraction = self.compress(data);
        let numerator = fraction.numer();

        if numerator.is_zero() {
            return 0.0;
        }

        // Python's math.log() handles arbitrary-precision ints without
        // overflow. Converting to f64 here is exact for the string
        // lengths this port is validated against — see the note on
        // this trade-off in DECISIONS.md if inputs get very long.
        let numerator_f = numerator.to_f64().unwrap_or(f64::INFINITY);
        (numerator_f.ln() / self.base.ln()).ceil()
    }
}
