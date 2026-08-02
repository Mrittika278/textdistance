use std::collections::HashSet;
use crate::{Distance, Similarity};

// ─── MRA (Match Rating Approach) ───

pub struct Mra;

impl Mra {
    fn calc_mra(word: &str) -> String {
        if word.is_empty() {
            return String::new();
        }
        let upper: String = word.chars().map(|c| c.to_ascii_uppercase()).collect();
        let chars: Vec<char> = upper.chars().collect();
        // Keep first char, remove vowels from the rest
        let mut result = vec![chars[0]];
        for c in &chars[1..] {
            match *c {
                'A' | 'E' | 'I' | 'O' | 'U' => {}
                _ => result.push(*c),
            }
        }
        // Remove consecutive duplicates
        let mut deduped = Vec::new();
        for c in result {
            if deduped.last() != Some(&c) {
                deduped.push(c);
            }
        }
        // Truncate to 6 (first 3 + last 3)
        if deduped.len() > 6 {
            let mut truncated = deduped[..3].to_vec();
            truncated.extend_from_slice(&deduped[deduped.len() - 3..]);
            truncated.into_iter().collect()
        } else {
            deduped.into_iter().collect()
        }
    }
}

impl Similarity for Mra {
    fn similarity_value(&self, s1: &str, s2: &str) -> usize {
        if s1.is_empty() || s2.is_empty() {
            return 0;
        }
        let mut seq1: Vec<char> = Self::calc_mra(s1).chars().collect();
        let mut seq2: Vec<char> = Self::calc_mra(s2).chars().collect();

        let max_length = seq1.len().max(seq2.len());
        let count = 2;

        if (max_length as isize - seq1.len().min(seq2.len()) as isize).unsigned_abs() > count {
            return 0;
        }

        for _ in 0..count {
            let minlen = seq1.len().min(seq2.len());
            let mut new_seq1 = Vec::new();
            let mut new_seq2 = Vec::new();
            for (c1, c2) in seq1.iter().zip(seq2.iter()) {
                if c1 != c2 {
                    new_seq1.push(*c1);
                    new_seq2.push(*c2);
                }
            }
            // Append the remainder of the longer sequence
            if seq1.len() > minlen {
                new_seq1.extend_from_slice(&seq1[minlen..]);
            }
            if seq2.len() > minlen {
                new_seq2.extend_from_slice(&seq2[minlen..]);
            }
            seq1 = new_seq1;
            seq2 = new_seq2;
        }

        if seq1.is_empty() && seq2.is_empty() {
            return max_length;
        }
        max_length - seq1.len().max(seq2.len())
    }

    fn maximum(&self, s1: &str, s2: &str) -> usize {
        Self::calc_mra(s1).len().max(Self::calc_mra(s2).len())
    }
}

// ─── Editex ───

pub struct Editex {
    match_cost: usize,
    group_cost: usize,
    mismatch_cost: usize,
    local: bool,
    groups: Vec<HashSet<char>>,
    grouped: HashSet<char>,
    ungrouped: HashSet<char>,
}

impl Editex {
    pub fn new() -> Self {
        let groups: Vec<HashSet<char>> = vec![
            ['A', 'E', 'I', 'O', 'U', 'Y'].iter().copied().collect(),
            ['B', 'P'].iter().copied().collect(),
            ['C', 'K', 'Q'].iter().copied().collect(),
            ['D', 'T'].iter().copied().collect(),
            ['L', 'R'].iter().copied().collect(),
            ['M', 'N'].iter().copied().collect(),
            ['G', 'J'].iter().copied().collect(),
            ['F', 'P', 'V'].iter().copied().collect(),
            ['S', 'X', 'Z'].iter().copied().collect(),
            ['C', 'S', 'Z'].iter().copied().collect(),
        ];
        let grouped: HashSet<char> = groups.iter().flat_map(|g| g.iter().copied()).collect();
        let ungrouped: HashSet<char> = ['H', 'W'].iter().copied().collect();
        Self {
            match_cost: 0,
            group_cost: 1,
            mismatch_cost: 2,
            local: false,
            groups,
            grouped,
            ungrouped,
        }
    }

    fn r_cost(&self, c1: char, c2: char) -> usize {
        if c1 == c2 {
            return self.match_cost;
        }
        if !self.grouped.contains(&c1) || !self.grouped.contains(&c2) {
            return self.mismatch_cost;
        }
        for group in &self.groups {
            if group.contains(&c1) && group.contains(&c2) {
                return self.group_cost;
            }
        }
        self.mismatch_cost
    }

    fn d_cost(&self, c1: char, c2: char) -> usize {
        if c1 != c2 && self.ungrouped.contains(&c1) {
            self.group_cost
        } else {
            self.r_cost(c1, c2)
        }
    }
}

impl Default for Editex {
    fn default() -> Self {
        Self::new()
    }
}

impl Distance for Editex {
    fn distance(&self, s1: &str, s2: &str) -> usize {
        if let Some(ans) = self.quick_answer(s1, s2) {
            return ans;
        }

        let max_length = self.maximum(s1, s2);

        // Prepend space and uppercase
        let s1_chars: Vec<char> = std::iter::once(' ')
            .chain(s1.chars().map(|c| c.to_ascii_uppercase()))
            .collect();
        let s2_chars: Vec<char> = std::iter::once(' ')
            .chain(s2.chars().map(|c| c.to_ascii_uppercase()))
            .collect();

        let len_s1 = s1_chars.len() - 1;
        let len_s2 = s2_chars.len() - 1;

        let mut d: Vec<Vec<usize>> = vec![vec![0; len_s2 + 1]; len_s1 + 1];

        if !self.local {
            for i in 1..=len_s1 {
                d[i][0] = d[i - 1][0] + self.d_cost(s1_chars[i - 1], s1_chars[i]);
            }
        }
        for j in 1..=len_s2 {
            d[0][j] = d[0][j - 1] + self.d_cost(s2_chars[j - 1], s2_chars[j]);
        }

        for i in 1..=len_s1 {
            for j in 1..=len_s2 {
                let del = d[i - 1][j] + self.d_cost(s1_chars[i - 1], s1_chars[i]);
                let ins = d[i][j - 1] + self.d_cost(s2_chars[j - 1], s2_chars[j]);
                let sub = d[i - 1][j - 1] + self.r_cost(s1_chars[i], s2_chars[j]);
                d[i][j] = del.min(ins).min(sub);
            }
        }

        let distance = d[len_s1][len_s2];
        distance.min(max_length)
    }

    fn maximum(&self, s1: &str, s2: &str) -> usize {
        s1.len().max(s2.len()) * self.mismatch_cost
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mra() {
        let alg = Mra;
        // Same name
        assert!(alg.similarity_value("Smith", "Smith") > 0);
        // Similar names
        assert!(alg.similarity_value("Smith", "Smyth") > 0);
        // Totally different
        assert_eq!(alg.similarity_value("abc", "xyz"), 0);
        // Empty
        assert_eq!(alg.similarity_value("", "abc"), 0);
    }

    #[test]
    fn test_editex() {
        let alg = Editex::new();
        // Same string
        assert_eq!(alg.distance("hello", "hello"), 0);
        // Empty
        assert_eq!(alg.distance("", ""), 0);
        // Different
        assert!(alg.distance("abc", "xyz") > 0);
        // Phonetically similar (same group)
        assert!(alg.distance("cat", "kat") < alg.distance("cat", "dog"));
    }
}
