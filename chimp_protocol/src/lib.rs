use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub struct Job {
    pub file: PathBuf,
    pub predictions_channel: String,
}

impl Job {
    pub fn from_slice(v: &[u8]) -> Result<Self, serde_json::Error> {
        serde_json::from_slice(v)
    }

    pub fn to_vec(&self) -> Result<Vec<u8>, serde_json::Error> {
        serde_json::to_vec(&self)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Predictions(pub Vec<Prediction>);

impl Predictions {
    pub fn from_slice(v: &[u8]) -> Result<Self, serde_json::Error> {
        serde_json::from_slice(v)
    }

    pub fn to_vec(&self) -> Result<Vec<u8>, serde_json::Error> {
        serde_json::to_vec(&self)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Prediction {
    pub bbox: [f32; 4],
    pub label: i64,
    pub score: f32,
}

impl Prediction {
    pub fn from_slice(v: &[u8]) -> Result<Self, serde_json::Error> {
        serde_json::from_slice(v)
    }

    pub fn to_vec(&self) -> Result<Vec<u8>, serde_json::Error> {
        serde_json::to_vec(&self)
    }
}
