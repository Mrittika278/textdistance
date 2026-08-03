use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct DistanceRequest {
    pub algorithm: String,
    pub string1: String,
    pub string2: String,
}

#[derive(Serialize)]
pub struct DistanceResponse {
    pub algorithm: String,
    pub distance: Option<f64>,
    pub similarity: Option<f64>,
    pub execution_ms: f64,
    pub status: String,
}
