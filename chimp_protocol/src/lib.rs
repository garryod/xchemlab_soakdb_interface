use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub struct Job {
    pub file: PathBuf,
    pub predictions_channel: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Predictions(pub Vec<Prediction>);

#[derive(Debug, Serialize, Deserialize)]
pub struct Prediction {
    pub bbox: [f32; 4],
    pub label: i64,
    pub score: f32,
}
