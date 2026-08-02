use crate::{Distance, Similarity};

/// Identity similarity: 1 if equal, 0 otherwise.
pub struct Identity;

impl Similarity for Identity {
    fn similarity_value(&self, s1: &str, s2: &str) -> usize {
        if s1 == s2 { 1 } else { 0 }
    }

    fn maximum(&self, _s1: &str, _s2: &str) -> usize {
        1
    }
}

/// Prefix similarity: length of common prefix.
pub struct Prefix;

impl Similarity for Prefix {
    fn similarity_value(&self, s1: &str, s2: &str) -> usize {
        if s1.is_empty() || s2.is_empty() {
            return 0;
        }
        s1.chars()
            .zip(s2.chars())
            .take_while(|(c1, c2)| c1 == c2)
            .count()
    }
}

/// Postfix similarity: length of common suffix.
pub struct Postfix;

impl Similarity for Postfix {
    fn similarity_value(&self, s1: &str, s2: &str) -> usize {
        if s1.is_empty() || s2.is_empty() {
            return 0;
        }
        s1.chars()
            .rev()
            .zip(s2.chars().rev())
            .take_while(|(c1, c2)| c1 == c2)
            .count()
    }
}

/// Hamming distance: number of differing items.
/// For unequal lengths, counts all positions (like zip_longest).
pub struct Hamming;

impl Distance for Hamming {
    fn distance(&self, s1: &str, s2: &str) -> usize {
        if let Some(ans) = self.quick_answer(s1, s2) {
            return ans;
        }
        let c1: Vec<char> = s1.chars().collect();
        let c2: Vec<char> = s2.chars().collect();
        let max_len = c1.len().max(c2.len());
        let mut diff = 0;
        for i in 0..max_len {
            let ch1 = c1.get(i);
            let ch2 = c2.get(i);
            if ch1 != ch2 {
                diff += 1;
            }
        }
        diff
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_identity() {
        let alg = Identity;
        assert_eq!(alg.similarity_value("hello", "hello"), 1);
        assert_eq!(alg.similarity_value("hello", "world"), 0);
        assert_eq!(alg.distance("hello", "hello"), 0);
        assert_eq!(alg.distance("hello", "world"), 1);
        assert_eq!(alg.normalized_similarity("hello", "hello"), 1.0);
        assert_eq!(alg.normalized_similarity("hello", "world"), 0.0);
        assert_eq!(alg.normalized_distance("hello", "hello"), 0.0);
        assert_eq!(alg.normalized_distance("hello", "world"), 1.0);
    }

    #[test]
    fn test_prefix() {
        let alg = Prefix;
        assert_eq!(alg.similarity_value("hello", "help"), 3);
        assert_eq!(alg.similarity_value("abc", "xyz"), 0);
        assert_eq!(alg.similarity_value("", "abc"), 0);
    }

    #[test]
    fn test_postfix() {
        let alg = Postfix;
        assert_eq!(alg.similarity_value("hello", "jello"), 4);
        assert_eq!(alg.similarity_value("abc", "xyz"), 0);
        assert_eq!(alg.similarity_value("abc", "bc"), 2);
    }

    #[test]
    fn test_hamming() {
        let alg = Hamming;
        assert_eq!(alg.distance("karolin", "kathrin"), 3);
        assert_eq!(alg.distance("1011101", "1001001"), 2);
        assert_eq!(alg.distance("", ""), 0);
        // Unequal lengths: zip_longest behavior
        assert_eq!(alg.distance("kitten", "sitting"), 3);
    }
}
