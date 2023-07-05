#![forbid(unsafe_code)]
#![warn(missing_docs)]
#![doc=include_str!("../README.md")]

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// A CHiMP job definition.
#[derive(Debug, Serialize, Deserialize)]
pub struct Job {
    /// The path of a file containing the image to perform inference on.
    pub file: PathBuf,
    /// The channel to send predictions to.
    pub predictions_channel: String,
}

impl Job {
    /// Deserialize an instance [`Job`] from bytes of JSON text.
    pub fn from_slice(v: &[u8]) -> Result<Self, serde_json::Error> {
        serde_json::from_slice(v)
    }

    /// Serialize the [`Job`] as a JSON byte vector
    pub fn to_vec(&self) -> Result<Vec<u8>, serde_json::Error> {
        serde_json::to_vec(&self)
    }
}

/// A set of predictions which apply to a single image.
#[derive(Debug, Serialize, Deserialize)]
pub struct Predictions(pub Vec<Prediction>);

impl Predictions {
    /// Deserialize an instance [`Predictions`] from bytes of JSON text.
    pub fn from_slice(v: &[u8]) -> Result<Self, serde_json::Error> {
        serde_json::from_slice(v)
    }

    /// Serialize the [`Predictions`] as a JSON byte vector
    pub fn to_vec(&self) -> Result<Vec<u8>, serde_json::Error> {
        serde_json::to_vec(&self)
    }
}

/// A singular predicted region.
#[derive(Debug, Serialize, Deserialize)]
pub struct Prediction {
    /// The bounding box which encompases the region.
    pub bbox: [f32; 4],
    /// The class label predicted to exist within the region.
    pub label: i64,
    /// The confidence of the prediction.
    pub score: f32,
}

impl Prediction {
    /// Deserialize an instance [`Prediction`] from bytes of JSON text.
    pub fn from_slice(v: &[u8]) -> Result<Self, serde_json::Error> {
        serde_json::from_slice(v)
    }

    /// Serialize the [`Prediction`] as a JSON byte vector
    pub fn to_vec(&self) -> Result<Vec<u8>, serde_json::Error> {
        serde_json::to_vec(&self)
    }
}
