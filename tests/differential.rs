use serde::Deserialize;
use textdistance_rs::{
    Distance, Similarity,
    simple::{Hamming, Identity, Prefix, Postfix},
    edit::{Levenshtein, DamerauLevenshtein, Jaro, JaroWinkler},
    phonetic::Editex,
    sequence::{LcsSeq, LcsStr, RatcliffObershelp},
    token::{Jaccard, Sorensen, Overlap, Cosine, Bag},
};

#[derive(Deserialize)]
struct Expected {
    pairs: Vec<(String, String)>,
    results: std::collections::HashMap<String, Vec<Option<serde_json::Value>>>,
}

fn load_expected() -> Expected {
    let json = include_str!("expected.json");
    serde_json::from_str(json).unwrap()
}

fn get_int(v: &Option<serde_json::Value>) -> Option<usize> {
    v.as_ref().and_then(|v| v.as_i64().map(|n| n as usize))
}

fn get_float(v: &Option<serde_json::Value>) -> Option<f64> {
    v.as_ref().and_then(|v| v.as_f64())
}

macro_rules! diff_test_distance {
    ($name:ident, $alg:expr, $key:literal) => {
        #[test]
        fn $name() {
            let data = load_expected();
            let vals = &data.results[$key];
            for (i, (s1, s2)) in data.pairs.iter().enumerate() {
                let rust_val = $alg.distance(s1, s2);
                if let Some(py_val) = get_int(&vals[i]) {
                    assert_eq!(
                        rust_val, py_val,
                        "{}: distance({}, {:?}) = {}, expected {}",
                        $key, i, (s1, s2), rust_val, py_val
                    );
                }
            }
        }
    };
}

macro_rules! diff_test_similarity_float {
    ($name:ident, $alg:expr, $key:literal) => {
        #[test]
        fn $name() {
            let data = load_expected();
            let vals = &data.results[$key];
            for (i, (s1, s2)) in data.pairs.iter().enumerate() {
                let rust_val = $alg.normalized_similarity(s1, s2);
                if let Some(py_val) = get_float(&vals[i]) {
                    let diff = (rust_val - py_val).abs();
                    assert!(
                        diff < 0.001,
                        "{}: normalized_similarity({}, {:?}) = {}, expected {}",
                        $key, i, (s1, s2), rust_val, py_val
                    );
                }
            }
        }
    };
}

macro_rules! diff_test_similarity_int {
    ($name:ident, $alg:expr, $key:literal) => {
        #[test]
        fn $name() {
            let data = load_expected();
            let vals = &data.results[$key];
            for (i, (s1, s2)) in data.pairs.iter().enumerate() {
                let rust_val = $alg.similarity_value(s1, s2);
                if let Some(py_val) = get_int(&vals[i]) {
                    assert_eq!(
                        rust_val, py_val,
                        "{}: similarity({}, {:?}) = {}, expected {}",
                        $key, i, (s1, s2), rust_val, py_val
                    );
                }
            }
        }
    };
}

// Distance algorithms
diff_test_distance!(diff_hamming, Hamming, "hamming");
diff_test_distance!(diff_levenshtein, Levenshtein, "levenshtein");
diff_test_distance!(diff_damerau_levenshtein, DamerauLevenshtein, "damerau_levenshtein");
diff_test_distance!(diff_bag, Bag, "bag");
diff_test_distance!(diff_editex, Editex::new(), "editex");

// Similarity algorithms (float)
diff_test_similarity_float!(diff_jaro, Jaro::new(), "jaro");
diff_test_similarity_float!(diff_jaro_winkler, JaroWinkler::new(), "jaro_winkler");
diff_test_similarity_float!(diff_ratcliff, RatcliffObershelp, "ratcliff_obershelp");
diff_test_similarity_float!(diff_jaccard, Jaccard, "jaccard");
diff_test_similarity_float!(diff_sorensen, Sorensen, "sorensen");
diff_test_similarity_float!(diff_overlap, Overlap, "overlap");
diff_test_similarity_float!(diff_cosine, Cosine, "cosine");
diff_test_similarity_float!(diff_identity, Identity, "identity");
diff_test_similarity_float!(diff_prefix, Prefix, "prefix");
diff_test_similarity_float!(diff_postfix, Postfix, "postfix");

// Similarity algorithms (int)
diff_test_similarity_int!(diff_lcsseq, LcsSeq, "lcsseq");
diff_test_similarity_int!(diff_lcsstr, LcsStr, "lcsstr");