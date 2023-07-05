mod image_loading;
mod inference;
mod jobs;

use clap::Parser;
use inference::{inference_worker, setup_inference_session};
use std::path::PathBuf;
use tokio::task::JoinSet;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about=None)]
struct Cli {
    /// The path to the ONNX model file.
    model: PathBuf,
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let args = Cli::parse();

    let session = setup_inference_session(args.model).unwrap();
    let batch_size = session.inputs[0].dimensions[0].unwrap().try_into().unwrap();

    let (_image_tx, image_rx) = tokio::sync::mpsc::channel(batch_size);
    let (prediction_tx, _prediction_rx) = tokio::sync::mpsc::unbounded_channel();

    let mut tasks = JoinSet::new();

    tasks.spawn(inference_worker(
        session,
        batch_size,
        image_rx,
        prediction_tx,
    ));

    tasks.join_next().await;
}
