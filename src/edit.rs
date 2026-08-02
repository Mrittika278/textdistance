use crate::Distance;

pub struct Levenshtein;

impl Distance for Levenshtein {
    fn distance(&self, s1: &str, s2: &str) -> usize {
        if let Some(ans) = self.quick_answer(s1, s2) {
            return ans;
        }
        let s1: Vec<char> = s1.chars().collect();
        let s2: Vec<char> = s2.chars().collect();
        let len1 = s1.len();
        let len2 = s2.len();

        let mut prev: Vec<usize> = (0..=len2).collect();
        let mut cur = vec![0usize; len2 + 1];

        for r in 1..=len1 {
            cur[0] = r;
            for c in 1..=len2 {
                let deletion = prev[c] + 1;
                let insertion = cur[c - 1] + 1;
                let edit = prev[c - 1] + if s1[r - 1] == s2[c - 1] { 0 } else { 1 };
                cur[c] = edit.min(deletion).min(insertion);
            }
            std::mem::swap(&mut prev, &mut cur);
        }
        prev[len2]
    }
}

// ─── Damerau-Levenshtein (restricted) ───

pub struct DamerauLevenshtein;

impl Distance for DamerauLevenshtein {
    fn distance(&self, s1: &str, s2: &str) -> usize {
        if let Some(ans) = self.quick_answer(s1, s2) {
            return ans;
        }
        let s1: Vec<char> = s1.chars().collect();
        let s2: Vec<char> = s2.chars().collect();
        let len1 = s1.len();
        let len2 = s2.len();

        // Standard 0-based DP matrix
        let mut d = vec![vec![0usize; len2 + 1]; len1 + 1];

        // Initialize borders
        for i in 0..=len1 {
            d[i][0] = i;
        }
        for j in 0..=len2 {
            d[0][j] = j;
        }

        for i in 1..=len1 {
            for j in 1..=len2 {
                let cost = if s1[i - 1] == s2[j - 1] { 0 } else { 1 };

                let deletion = d[i - 1][j] + 1;
                let insertion = d[i][j - 1] + 1;
                let substitution = d[i - 1][j - 1] + cost;

                let mut result = deletion.min(insertion).min(substitution);

                // Transposition: check if current char matches previous in s2,
                // and previous char in s1 matches current in s2
                if i > 1 && j > 1 && s1[i - 1] == s2[j - 2] && s1[i - 2] == s2[j - 1] {
                    let transposition = d[i - 2][j - 2] + cost;
                    result = result.min(transposition);
                }

                d[i][j] = result;
            }
        }

        d[len1][len2]
    }
}

// ─── Jaro ───

pub struct Jaro {
    pub long_tolerance: bool,
}

impl Jaro {
    pub fn new() -> Self {
        Self { long_tolerance: false }
    }
}

impl Default for Jaro {
    fn default() -> Self {
        Self::new()
    }
}

fn jaro_core(s1: &[char], s2: &[char], long_tolerance: bool, prefix_weight: f64) -> f64 {
    let len1 = s1.len();
    let len2 = s2.len();

    if len1 == 0 || len2 == 0 {
        return 0.0;
    }

    let search_range = (len1.max(len2) / 2).saturating_sub(1);

    let mut s1_flags = vec![false; len1];
    let mut s2_flags = vec![false; len2];

    // Count matching characters
    let mut common_chars = 0usize;
    for i in 0..len1 {
        let low = i.saturating_sub(search_range);
        let hi = (i + search_range + 1).min(len2);
        for j in low..hi {
            if !s2_flags[j] && s1[i] == s2[j] {
                s1_flags[i] = true;
                s2_flags[j] = true;
                common_chars += 1;
                break;
            }
        }
    }

    if common_chars == 0 {
        return 0.0;
    }

    // Count transpositions
    let mut k = 0;
    let mut trans_count = 0usize;
    for i in 0..len1 {
        if !s1_flags[i] {
            continue;
        }
        while k < len2 && !s2_flags[k] {
            k += 1;
        }
        if s1[i] != s2[k] {
            trans_count += 1;
        }
        k += 1;
    }
    let trans_count = trans_count / 2;

    // Jaro weight
    let mut weight = common_chars as f64 / len1 as f64
        + common_chars as f64 / len2 as f64
        + (common_chars - trans_count) as f64 / common_chars as f64;
    weight /= 3.0;

    // No Winkler boost if prefix_weight is 0
    if prefix_weight == 0.0 {
        return weight;
    }

    if weight <= 0.7 {
        return weight;
    }

    // Winkler prefix boost (up to 4 chars)
    let min_len = len1.min(len2);
    let max_prefix = min_len.min(4);
    let mut prefix_len = 0usize;
    for i in 0..max_prefix {
        if s1[i] == s2[i] {
            prefix_len += 1;
        } else {
            break;
        }
    }
    if prefix_len > 0 {
        weight += prefix_len as f64 * prefix_weight * (1.0 - weight);
    }

    // Long string tolerance
    if !long_tolerance || min_len <= 4 {
        return weight;
    }
    if common_chars <= prefix_len + 1 || 2 * common_chars < min_len + prefix_len {
        return weight;
    }
    let tmp = (common_chars - prefix_len - 1) as f64 / (len1 + len2 - prefix_len * 2 + 2) as f64;
    weight += (1.0 - weight) * tmp;
    weight
}

pub struct JaroWinkler {
    pub long_tolerance: bool,
}

impl JaroWinkler {
    pub fn new() -> Self {
        Self { long_tolerance: false }
    }
}

impl Default for JaroWinkler {
    fn default() -> Self {
        Self::new()
    }
}

impl crate::Similarity for Jaro {
    fn similarity_value(&self, s1: &str, s2: &str) -> usize {
        if let Some(ans) = self.quick_answer(s1, s2) {
            return ans;
        }
        let s1: Vec<char> = s1.chars().collect();
        let s2: Vec<char> = s2.chars().collect();
        let val = jaro_core(&s1, &s2, self.long_tolerance, 0.0);
        (val * 1000.0).round() as usize
    }

    fn maximum(&self, _s1: &str, _s2: &str) -> usize {
        1000
    }
}

impl crate::Similarity for JaroWinkler {
    fn similarity_value(&self, s1: &str, s2: &str) -> usize {
        if let Some(ans) = self.quick_answer(s1, s2) {
            return ans;
        }
        let s1: Vec<char> = s1.chars().collect();
        let s2: Vec<char> = s2.chars().collect();
        let val = jaro_core(&s1, &s2, self.long_tolerance, 0.1);
        (val * 1000.0).round() as usize
    }

    fn maximum(&self, _s1: &str, _s2: &str) -> usize {
        1000
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_levenshtein_basic() {
        let alg = Levenshtein;
        assert_eq!(alg.distance("kitten", "sitting"), 3);
        assert_eq!(alg.distance("saturday", "sunday"), 3);
        assert_eq!(alg.distance("", ""), 0);
        assert_eq!(alg.distance("", "abc"), 3);
        assert_eq!(alg.distance("abc", ""), 3);
        assert_eq!(alg.distance("abc", "abc"), 0);
    }

    #[test]
    fn test_levenshtein_similarity() {
        let alg = Levenshtein;
        assert_eq!(alg.similarity("kitten", "sitting"), 4);
        let nd = alg.normalized_distance("kitten", "sitting");
        assert!((nd - 0.42857142857142855).abs() < 1e-10);
        let ns = alg.normalized_similarity("kitten", "sitting");
        assert!((ns - 0.5714285714285714).abs() < 1e-10);
    }

    #[test]
    fn test_levenshtein_quick_answer() {
        let alg = Levenshtein;
        assert_eq!(alg.distance("hello", "hello"), 0);
        assert_eq!(alg.distance("", "hello"), 5);
        assert_eq!(alg.distance("hello", ""), 5);
    }

    #[test]
    fn test_damerau_levenshtein() {
        let alg = DamerauLevenshtein;
        // CA -> AC: transposition
        assert_eq!(alg.distance("ca", "ac"), 1);
        // Same as levenshtein when no transposition
        assert_eq!(alg.distance("kitten", "sitting"), 3);
        assert_eq!(alg.distance("", ""), 0);
        assert_eq!(alg.distance("abc", "abc"), 0);
        assert_eq!(alg.distance("", "abc"), 3);
    }

    #[test]
    fn test_jaro() {
        let _alg = Jaro::new();
        // "MARTHA" vs "MARHTA" should be high (transposition)
        let val = jaro_core(
            &"MARTHA".chars().collect::<Vec<_>>(),
            &"MARHTA".chars().collect::<Vec<_>>(),
            false,
            0.0,
        );
        assert!((val - 0.9444444444444445).abs() < 1e-10);

        // Exact match
        let val = jaro_core(
            &"hello".chars().collect::<Vec<_>>(),
            &"hello".chars().collect::<Vec<_>>(),
            false,
            0.0,
        );
        assert!((val - 1.0).abs() < 1e-10);

        // No match
        let val = jaro_core(
            &"abc".chars().collect::<Vec<_>>(),
            &"xyz".chars().collect::<Vec<_>>(),
            false,
            0.0,
        );
        assert!((val - 0.0).abs() < 1e-10);
    }

    #[test]
    fn test_jaro_winkler() {
        let val = jaro_core(
            &"MARTHA".chars().collect::<Vec<_>>(),
            &"MARHTA".chars().collect::<Vec<_>>(),
            false,
            0.1,
        );
        // Jaro-Winkler boosts Jaro because of "MART" prefix
        assert!(val > 0.96);
    }
}
