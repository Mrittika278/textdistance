use std::time::Instant;
use axum::{
    extract::Json,
    http::StatusCode,
};

use crate::models::{DistanceRequest, DistanceResponse};

use textdistance_rs::{Distance, Similarity};

use textdistance_rs::simple::{Identity, Prefix, Postfix, Hamming};
use textdistance_rs::edit::{Levenshtein, DamerauLevenshtein, Jaro, JaroWinkler};
use textdistance_rs::token::{Jaccard, Sorensen, Overlap, Cosine, Tversky, Bag};
use textdistance_rs::sequence::{LcsSeq, LcsStr, RatcliffObershelp};
use textdistance_rs::phonetic::{Mra, Editex};

pub async fn calculate(
    Json(req): Json<DistanceRequest>,
) -> Result<Json<DistanceResponse>, StatusCode> {

    let start = Instant::now();
    let s1 = req.string1.as_str();
    let s2 = req.string2.as_str();

   let (distance, similarity) = match req.algorithm.as_str() {

    //---------------- SIMPLE ----------------

    "identity" => (
        None,
        Some(Identity.similarity_value(s1, s2) as f64)
    ),

    "prefix" => (
        None,
        Some(Prefix.similarity_value(s1, s2) as f64)
    ),

    "postfix" => (
        None,
        Some(Postfix.similarity_value(s1, s2) as f64)
    ),

    "hamming" => {

        if s1.chars().count() != s2.chars().count() {
            return Err(StatusCode::BAD_REQUEST);
        }

        (
            Some(Hamming.distance(s1, s2) as f64),
            None
        )

    },

    //---------------- EDIT ----------------

    "levenshtein" => (
        Some(Levenshtein.distance(s1, s2) as f64),
        Some(Levenshtein.normalized_similarity(s1, s2))
    ),

    "damerau" => (
        Some(DamerauLevenshtein.distance(s1, s2) as f64),
        Some(DamerauLevenshtein.normalized_similarity(s1, s2))
    ),

    "jaro" => (
        None,
        Some(Jaro::new().normalized_similarity(s1, s2))
    ),

    "jaro_winkler" => (
        None,
        Some(JaroWinkler::new().normalized_similarity(s1, s2))
    ),

    //---------------- TOKEN ----------------

    "jaccard" => (
        None,
        Some(Jaccard.normalized_similarity(s1, s2))
    ),

    "sorensen" => (
        None,
        Some(Sorensen.normalized_similarity(s1, s2))
    ),

    "overlap" => (
        None,
        Some(Overlap.normalized_similarity(s1, s2))
    ),

    "cosine" => (
        None,
        Some(Cosine.normalized_similarity(s1, s2))
    ),

    "tversky" => (
        None,
        Some(Tversky::new().normalized_similarity(s1, s2))
    ),

    "bag" => (
        Some(Bag.distance(s1, s2) as f64),
        None
    ),

    //---------------- SEQUENCE ----------------

    "lcsseq" => (
        None,
        Some(LcsSeq.normalized_similarity(s1, s2))
    ),

    "lcsstr" => (
        None,
        Some(LcsStr.normalized_similarity(s1, s2))
    ),

    "ratcliff" => (
        None,
        Some(RatcliffObershelp.normalized_similarity(s1, s2))
    ),

    //---------------- PHONETIC ----------------

    "mra" => (
        None,
        Some(Mra.normalized_similarity(s1, s2))
    ),

    "editex" => (
        Some(Editex::new().distance(s1, s2) as f64),
        Some(Editex::new().normalized_similarity(s1, s2))
    ),

    _ => return Err(StatusCode::BAD_REQUEST),
};

    let elapsed = start.elapsed().as_secs_f64() * 1000.0;

Ok(Json(DistanceResponse {

    algorithm: req.algorithm,

    distance,

    similarity,

    execution_ms: elapsed,

    status: String::from("Success"),

}))
}
