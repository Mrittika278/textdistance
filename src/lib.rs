pub mod simple;
pub mod edit;
pub mod token;
pub mod sequence;
pub mod phonetic;
pub mod compression;

pub trait Distance: Sized {
    fn distance(&self, s1: &str, s2: &str) -> usize;

    fn maximum(&self, s1: &str, s2: &str) -> usize {
        s1.len().max(s2.len())
    }

    fn similarity(&self, s1: &str, s2: &str) -> usize {
        self.maximum(s1, s2) - self.distance(s1, s2)
    }

    fn normalized_distance(&self, s1: &str, s2: &str) -> f64 {
        let max = self.maximum(s1, s2);
        if max == 0 {
            return 0.0;
        }
        self.distance(s1, s2) as f64 / max as f64
    }

    fn normalized_similarity(&self, s1: &str, s2: &str) -> f64 {
        1.0 - self.normalized_distance(s1, s2)
    }

    fn quick_answer(&self, s1: &str, s2: &str) -> Option<usize> {
        if s1 == s2 {
            return Some(0);
        }
        if s1.is_empty() || s2.is_empty() {
            return Some(self.maximum(s1, s2));
        }
        None
    }
}

pub trait Similarity: Sized {
    fn similarity_value(&self, s1: &str, s2: &str) -> usize;

    fn maximum(&self, s1: &str, s2: &str) -> usize {
        s1.len().max(s2.len())
    }

    fn distance(&self, s1: &str, s2: &str) -> usize {
        self.maximum(s1, s2) - self.similarity_value(s1, s2)
    }

    fn normalized_distance(&self, s1: &str, s2: &str) -> f64 {
        let max = self.maximum(s1, s2);
        if max == 0 {
            return 0.0;
        }
        self.distance(s1, s2) as f64 / max as f64
    }

    fn normalized_similarity(&self, s1: &str, s2: &str) -> f64 {
        let max = self.maximum(s1, s2);
        if max == 0 {
            return 1.0;
        }
        self.similarity_value(s1, s2) as f64 / max as f64
    }

    fn quick_answer(&self, s1: &str, s2: &str) -> Option<usize> {
        if s1 == s2 {
            return Some(self.maximum(s1, s2));
        }
        if s1.is_empty() || s2.is_empty() {
            return Some(0);
        }
        None
    }
}
