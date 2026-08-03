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

    let s1 = req.string1.as_str();
    let s2 = req.string2.as_str();

    let result = match req.algorithm.as_str() {

        //---------------- SIMPLE ----------------

        "identity" => Identity.similarity_value(s1, s2) as f64,

        "prefix" => Prefix.similarity_value(s1, s2) as f64,

        "postfix" => Postfix.similarity_value(s1, s2) as f64,

        "hamming" => {
            if s1.chars().count() != s2.chars().count() {
                return Err(StatusCode::BAD_REQUEST);
            }
            Hamming.distance(s1, s2) as f64
        }

        //---------------- EDIT ----------------

        "levenshtein" => Levenshtein.distance(s1, s2) as f64,

        "damerau" => DamerauLevenshtein.distance(s1, s2) as f64,

        "jaro" => Jaro::new().normalized_similarity(s1, s2),

        "jaro_winkler" => JaroWinkler::new().normalized_similarity(s1, s2),

        //---------------- TOKEN ----------------

        "jaccard" => Jaccard.normalized_similarity(s1, s2),

        "sorensen" => Sorensen.normalized_similarity(s1, s2),

        "overlap" => Overlap.normalized_similarity(s1, s2),

        "cosine" => Cosine.normalized_similarity(s1, s2),

        "tversky" => Tversky::new().normalized_similarity(s1, s2),

        "bag" => Bag.distance(s1, s2) as f64,

        //---------------- SEQUENCE ----------------

        "lcsseq" => LcsSeq.similarity_value(s1, s2) as f64,

        "lcsstr" => LcsStr.similarity_value(s1, s2) as f64,

        "ratcliff" => RatcliffObershelp.normalized_similarity(s1, s2),

        //---------------- PHONETIC ----------------

        "mra" => Mra.similarity_value(s1, s2) as f64,

        "editex" => Editex::new().distance(s1, s2) as f64,

        //---------------- Unknown ----------------

        _ => return Err(StatusCode::BAD_REQUEST),
    };

    Ok(Json(DistanceResponse {

        algorithm: req.algorithm,

        result,

        status: String::from("Success"),

    }))
}
