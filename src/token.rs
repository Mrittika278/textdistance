use std::collections::HashMap;
use crate::Similarity;
use crate::Distance;

/// Build a character frequency map from a string
fn get_counter(s: &str) -> HashMap<char, usize> {
    let mut map = HashMap::new();
    for c in s.chars() {
        *map.entry(c).or_insert(0) += 1;
    }
    map
}

/// Count total elements in a counter
fn count_counter(counter: &HashMap<char, usize>) -> usize {
    counter.values().sum()
}

/// Intersect two counters (min of each key)
fn intersect_counters(
    c1: &HashMap<char, usize>,
    c2: &HashMap<char, usize>,
) -> HashMap<char, usize> {
    let mut result = HashMap::new();
    for (k, &v1) in c1 {
        if let Some(&v2) = c2.get(k) {
            result.insert(*k, v1.min(v2));
        }
    }
    result
}

/// Union two counters (max of each key)
fn union_counters(
    c1: &HashMap<char, usize>,
    c2: &HashMap<char, usize>,
) -> HashMap<char, usize> {
    let mut result = c1.clone();
    for (k, &v2) in c2 {
        let current = *result.get(k).unwrap_or(&0);
        result.insert(*k, current.max(v2));
    }
    result
}

// ─── Jaccard ───

pub struct Jaccard;

impl Similarity for Jaccard {
    fn similarity_value(&self, s1: &str, s2: &str) -> usize {
        if let Some(ans) = self.quick_answer(s1, s2) {
            return ans;
        }
        let c1 = get_counter(s1);
        let c2 = get_counter(s2);
        let intersection = count_counter(&intersect_counters(&c1, &c2));
        let union = count_counter(&union_counters(&c1, &c2));
        if union == 0 {
            return 1000;
        }
        (intersection as f64 / union as f64 * 1000.0).round() as usize
    }

    fn maximum(&self, _s1: &str, _s2: &str) -> usize {
        1000
    }
}

// ─── Sorensen (Dice) ───

pub struct Sorensen;

impl Similarity for Sorensen {
    fn similarity_value(&self, s1: &str, s2: &str) -> usize {
        if let Some(ans) = self.quick_answer(s1, s2) {
            return ans;
        }
        let c1 = get_counter(s1);
        let c2 = get_counter(s2);
        let count = count_counter(&c1) + count_counter(&c2);
        if count == 0 {
            return 1000;
        }
        let intersection = count_counter(&intersect_counters(&c1, &c2));
        (2.0 * intersection as f64 / count as f64 * 1000.0).round() as usize
    }

    fn maximum(&self, _s1: &str, _s2: &str) -> usize {
        1000
    }
}

// ─── Overlap ───

pub struct Overlap;

impl Similarity for Overlap {
    fn similarity_value(&self, s1: &str, s2: &str) -> usize {
        if let Some(ans) = self.quick_answer(s1, s2) {
            return ans;
        }
        let c1 = get_counter(s1);
        let c2 = get_counter(s2);
        let intersection = count_counter(&intersect_counters(&c1, &c2));
        let min_count = count_counter(&c1).min(count_counter(&c2));
        if min_count == 0 {
            return 1000;
        }
        (intersection as f64 / min_count as f64 * 1000.0).round() as usize
    }

    fn maximum(&self, _s1: &str, _s2: &str) -> usize {
        1000
    }
}

// ─── Cosine ───

pub struct Cosine;

impl Similarity for Cosine {
    fn similarity_value(&self, s1: &str, s2: &str) -> usize {
        if let Some(ans) = self.quick_answer(s1, s2) {
            return ans;
        }
        let c1 = get_counter(s1);
        let c2 = get_counter(s2);
        let intersection = count_counter(&intersect_counters(&c1, &c2));
        let count1 = count_counter(&c1);
        let count2 = count_counter(&c2);
        if count1 == 0 && count2 == 0 {
            return 1000;
        }
        let prod = (count1 * count2) as f64;
        let denom = prod.sqrt();
        if denom == 0.0 {
            return 0;
        }
        (intersection as f64 / denom * 1000.0).round() as usize
    }

    fn maximum(&self, _s1: &str, _s2: &str) -> usize {
        1000
    }
}

// ─── Tversky ───

pub struct Tversky {
    pub alpha: f64,
    pub beta: f64,
}

impl Tversky {
    pub fn new() -> Self {
        Self { alpha: 1.0, beta: 1.0 }
    }
}

impl Default for Tversky {
    fn default() -> Self {
        Self::new()
    }
}

impl Similarity for Tversky {
    fn similarity_value(&self, s1: &str, s2: &str) -> usize {
        if let Some(ans) = self.quick_answer(s1, s2) {
            return ans;
        }
        let c1 = get_counter(s1);
        let c2 = get_counter(s2);
        let intersection = count_counter(&intersect_counters(&c1, &c2));
        let count1 = count_counter(&c1);
        let count2 = count_counter(&c2);
        let diff1 = count1 - intersection;
        let diff2 = count2 - intersection;
        let denom = intersection as f64 + self.alpha * diff1 as f64 + self.beta * diff2 as f64;
        if denom == 0.0 {
            return 1000;
        }
        (intersection as f64 / denom * 1000.0).round() as usize
    }

    fn maximum(&self, _s1: &str, _s2: &str) -> usize {
        1000
    }
}

// ─── Bag distance ───

pub struct Bag;

impl Distance for Bag {
    fn distance(&self, s1: &str, s2: &str) -> usize {
        if let Some(ans) = self.quick_answer(s1, s2) {
            return ans;
        }
        let c1 = get_counter(s1);
        let c2 = get_counter(s2);
        let intersection = intersect_counters(&c1, &c2);

        let mut diff1 = 0usize;
        for (k, &v) in &c1 {
            diff1 += v - intersection.get(k).unwrap_or(&0);
        }
        let mut diff2 = 0usize;
        for (k, &v) in &c2 {
            diff2 += v - intersection.get(k).unwrap_or(&0);
        }
        diff1.max(diff2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_jaccard() {
        let alg = Jaccard;
        let val = alg.normalized_similarity("abc", "abd");
        assert!((val - 0.5).abs() < 0.01);
        assert!((alg.normalized_similarity("hello", "hello") - 1.0).abs() < 0.001);
        assert!((alg.normalized_similarity("abc", "xyz") - 0.0).abs() < 0.001);
    }

    #[test]
    fn test_sorensen() {
        let alg = Sorensen;
        let val = alg.normalized_similarity("abc", "abd");
        assert!((val - 0.6666666).abs() < 0.01);
        assert!((alg.normalized_similarity("hello", "hello") - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_overlap() {
        let alg = Overlap;
        let val = alg.normalized_similarity("abc", "abd");
        assert!((val - 0.6666666).abs() < 0.01);
        assert!((alg.normalized_similarity("hello", "hello") - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_cosine() {
        let alg = Cosine;
        let val = alg.normalized_similarity("abc", "abd");
        assert!((val - 0.6666666).abs() < 0.01);
        assert!((alg.normalized_similarity("hello", "hello") - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_bag() {
        let alg = Bag;
        assert_eq!(alg.distance("abc", "abd"), 1);
        assert_eq!(alg.distance("hello", "hello"), 0);
        assert_eq!(alg.distance("abc", "xyz"), 3);
        assert_eq!(alg.distance("", ""), 0);
        assert_eq!(alg.distance("", "abc"), 3);
    }
}
