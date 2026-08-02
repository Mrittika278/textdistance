use crate::Similarity;


pub struct LcsSeq;

impl LcsSeq {

    fn lcs_length(s1: &[char], s2: &[char]) -> usize {
        let len1 = s1.len();
        let len2 = s2.len();

        let mut prev = vec![0usize; len2 + 1];
        let mut cur = vec![0usize; len2 + 1];

        for i in 1..=len1 {
            for j in 1..=len2 {
                if s1[i - 1] == s2[j - 1] {
                    cur[j] = prev[j - 1] + 1;
                } else {
                    cur[j] = prev[j].max(cur[j - 1]);
                }
            }
            std::mem::swap(&mut prev, &mut cur);
        }
        prev[len2]
    }
}

impl Similarity for LcsSeq {
    fn similarity_value(&self, s1: &str, s2: &str) -> usize {
        if let Some(ans) = self.quick_answer(s1, s2) {
            return ans;
        }
        let c1: Vec<char> = s1.chars().collect();
        let c2: Vec<char> = s2.chars().collect();
        Self::lcs_length(&c1, &c2)
    }
}


pub struct LcsStr;

impl LcsStr {

    fn lcsstr_length(s1: &[char], s2: &[char]) -> usize {
        let len1 = s1.len();
        let len2 = s2.len();
        let mut max_len = 0;

        let mut prev = vec![0usize; len2 + 1];
        let mut cur = vec![0usize; len2 + 1];

        for i in 1..=len1 {
            for j in 1..=len2 {
                if s1[i - 1] == s2[j - 1] {
                    cur[j] = prev[j - 1] + 1;
                    if cur[j] > max_len {
                        max_len = cur[j];
                    }
                } else {
                    cur[j] = 0;
                }
            }
            std::mem::swap(&mut prev, &mut cur);
      
            for v in cur.iter_mut() {
                *v = 0;
            }
        }
        max_len
    }
}

impl Similarity for LcsStr {
    fn similarity_value(&self, s1: &str, s2: &str) -> usize {
        if let Some(ans) = self.quick_answer(s1, s2) {
            return ans;
        }
        let c1: Vec<char> = s1.chars().collect();
        let c2: Vec<char> = s2.chars().collect();
        Self::lcsstr_length(&c1, &c2)
    }
}



pub struct RatcliffObershelp;

impl RatcliffObershelp {

    fn find_lcsstr_pos(s1: &[char], s2: &[char]) -> (usize, usize) {
        let len1 = s1.len();
        let len2 = s2.len();
        let mut max_len = 0;
        let mut pos1 = 0;

        let mut prev = vec![0usize; len2 + 1];
        let mut cur = vec![0usize; len2 + 1];

        for i in 1..=len1 {
            for j in 1..=len2 {
                if s1[i - 1] == s2[j - 1] {
                    cur[j] = prev[j - 1] + 1;
                    if cur[j] > max_len {
                        max_len = cur[j];
                        pos1 = i - max_len;
                    }
                } else {
                    cur[j] = 0;
                }
            }
            std::mem::swap(&mut prev, &mut cur);
            for v in cur.iter_mut() {
                *v = 0;
            }
        }
        (max_len, pos1)
    }

    fn find_helper(s1: &[char], s2: &[char]) -> usize {
        let (length, pos1) = Self::find_lcsstr_pos(s1, s2);
        if length == 0 {
            return 0;
        }
        let before1 = &s1[..pos1];
        let after1 = &s1[pos1 + length..];


        let sub = &s1[pos1..pos1 + length];
        let pos2 = s2.windows(length)
            .position(|w| w == sub)
            .unwrap_or(0);
        let before2 = &s2[..pos2];
        let after2 = &s2[pos2 + length..];

        Self::find_helper(before1, before2) + length + Self::find_helper(after1, after2)
    }
}

impl Similarity for RatcliffObershelp {
    fn similarity_value(&self, s1: &str, s2: &str) -> usize {
        if let Some(ans) = self.quick_answer(s1, s2) {
            return ans;
        }
        let c1: Vec<char> = s1.chars().collect();
        let c2: Vec<char> = s2.chars().collect();
        let total = c1.len() + c2.len();
        if total == 0 {
            return 1000;
        }
        let matched = Self::find_helper(&c1, &c2);
        ((2.0 * matched as f64) / (total as f64) * 1000.0).round() as usize
    }

    fn maximum(&self, _s1: &str, _s2: &str) -> usize {
        1000
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lcsseq() {
        let alg = LcsSeq;
        // "abcde" vs "ace" -> LCS is "ace" length 3
        assert_eq!(alg.similarity_value("abcde", "ace"), 3);
        // "abc" vs "abc" -> 3
        assert_eq!(alg.similarity_value("abc", "abc"), 3);
        // "abc" vs "xyz" -> 0
        assert_eq!(alg.similarity_value("abc", "xyz"), 0);
        // Empty
        assert_eq!(alg.similarity_value("", "abc"), 0);
        assert_eq!(alg.similarity_value("", ""), 0);
    }

    #[test]
    fn test_lcsstr() {
        let alg = LcsStr;
        // "abcdef" vs "zabxycdef" -> longest common substring "cdef" length 4
        assert_eq!(alg.similarity_value("abcdef", "zabxycdef"), 4);
        // "abc" vs "abc" -> 3
        assert_eq!(alg.similarity_value("abc", "abc"), 3);
        // "abc" vs "xyz" -> 0
        assert_eq!(alg.similarity_value("abc", "xyz"), 0);
        // Empty
        assert_eq!(alg.similarity_value("", "abc"), 0);
    }

    #[test]
    fn test_ratcliff_obershelp() {
        let alg = RatcliffObershelp;
        // Exact match
        let val = alg.normalized_similarity("hello", "hello");
        assert!((val - 1.0).abs() < 0.001);
        // No match
        let val = alg.normalized_similarity("abc", "xyz");
        assert!((val - 0.0).abs() < 0.001);
        // Partial match
        let val = alg.normalized_similarity("abcde", "abfde");
        assert!(val > 0.5);
    }
}
