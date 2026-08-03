use std::collections::HashMap;
use std::f64;
use itertools::Itertools;

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