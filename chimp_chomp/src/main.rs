#![forbid(unsafe_code)]
#![warn(missing_docs)]
#![doc=include_str!("../README.md")]

mod image_loading;
mod inference;
mod jobs;

use clap::Parser;
use inference::{inference_worker, setup_inference_session};
use jobs::{
    job_consumption_worker, predictions_producer_worker, setup_job_consumer, setup_rabbitmq_client,
};
use std::path::PathBuf;
use tokio::task::JoinSet;
use url::Url;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about=None)]
struct Cli {
    /// The path to the ONNX model file.
    model: PathBuf,
    /// The URL of the RabbitMQ server.
    rabbitmq_url: Url,
    /// The RabbitMQ channel on which jobs are assigned.
    rabbitmq_channel: String,
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let args = Cli::parse();

    let session = setup_inference_session(args.model).unwrap();
    let input_width = session.inputs[0].dimensions[3].unwrap();
    let input_height = session.inputs[0].dimensions[2].unwrap();
    let batch_size = session.inputs[0].dimensions[0].unwrap().try_into().unwrap();

    let rabbitmq_client = setup_rabbitmq_client(args.rabbitmq_url).await.unwrap();
    let job_channel = rabbitmq_client.create_channel().await.unwrap();
    let predictions_channel = rabbitmq_client.create_channel().await.unwrap();
    let job_consumer = setup_job_consumer(job_channel, args.rabbitmq_channel)
        .await
        .unwrap();

    let (image_tx, image_rx) = tokio::sync::mpsc::channel(batch_size);
    let (prediction_tx, prediction_rx) = tokio::sync::mpsc::unbounded_channel();

    let mut tasks = JoinSet::new();

    tasks.spawn(inference_worker(
        session,
        batch_size,
        image_rx,
        prediction_tx,
    ));

    tasks.spawn(job_consumption_worker(
        job_consumer,
        input_width,
        input_height,
        image_tx,
    ));

    tasks.spawn(predictions_producer_worker(
        prediction_rx,
        predictions_channel,
    ));

    tasks.join_next().await;
}
