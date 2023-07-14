#![forbid(unsafe_code)]
#![warn(missing_docs)]
#![doc=include_str!("../README.md")]

use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use uuid::Uuid;

/// A CHiMP job definition.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Job {
    /// A unique identifier for the job, to be returned in the [`Response`].
    pub id: Uuid,
    /// The path of a file containing the image to perform inference on.
    pub file: PathBuf,
    /// The channel to send predictions to.
    pub predictions_channel: String,
}

impl Job {
    /// Deserialize an instance [`Request`] from bytes of JSON text.
    pub fn from_slice(v: &[u8]) -> Result<Self, serde_json::Error> {
        serde_json::from_slice(v)
    }

    /// Serialize the [`Request`] as a JSON byte vector
    pub fn to_vec(&self) -> Result<Vec<u8>, serde_json::Error> {
        serde_json::to_vec(&self)
    }
}

/// A set of predictions which apply to a single image.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Response {
    /// The unique identifier of the requesting [`Request`].
    pub job_id: Uuid,
    /// The proposed point for solvent insertion.
    pub insertion_point: Point,
    /// The location of the well centroid and radius.
    pub well_location: Circle,
    /// A bounding box emcompasing the solvent.
    pub drop: BBox,
    /// A set of bounding boxes, each emcompasing a crystal.
    pub crystals: Vec<BBox>,
}

impl Response {
    /// Deserialize an instance [`Predictions`] from bytes of JSON text.
    pub fn from_slice(v: &[u8]) -> Result<Self, serde_json::Error> {
        serde_json::from_slice(v)
    }

    /// Serialize the [`Predictions`] as a JSON byte vector
    pub fn to_vec(&self) -> Result<Vec<u8>, serde_json::Error> {
        serde_json::to_vec(&self)
    }
}

/// A point in 2D space.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Point {
    /// The position of the point in the X axis.
    pub x: usize,
    /// The position of the point in the Y axis.
    pub y: usize,
}

/// A circle, defined by the center point and radius.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Circle {
    /// The position of the circles center.
    pub center: Point,
    /// The radius of the circle.
    pub radius: f32,
}

/// A bounding box which encompasing a region.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BBox {
    /// The position of the upper bound in the Y axis.
    pub top: f32,
    /// The position of the lower bound in the Y axis.
    pub bottom: f32,
    /// The position of the upper bound in the X axis.
    pub right: f32,
    /// The position of the lower bound in the X axis.
    pub left: f32,
}
