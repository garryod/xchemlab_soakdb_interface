#![forbid(unsafe_code)]
#![warn(missing_docs)]
#![doc=include_str!("../README.md")]

mod image_loading;
mod inference;
mod jobs;
mod postprocessing;
mod well_centering;

use crate::{
    inference::{inference_worker, setup_inference_session},
    jobs::{consume_job, produce_response, setup_job_consumer, setup_rabbitmq_client},
    postprocessing::postprocess_inference,
    well_centering::find_well_center,
};
use clap::Parser;
use futures_lite::StreamExt;
use std::{collections::HashMap, path::PathBuf};
use tokio::{select, task::JoinSet};
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
    opencv::core::set_num_threads(0).unwrap();

    let session = setup_inference_session(args.model).unwrap();
    let input_width = session.inputs[0].dimensions[3].unwrap();
    let input_height = session.inputs[0].dimensions[2].unwrap();
    let batch_size = session.inputs[0].dimensions[0].unwrap().try_into().unwrap();

    let rabbitmq_client = setup_rabbitmq_client(args.rabbitmq_url).await.unwrap();
    let job_channel = rabbitmq_client.create_channel().await.unwrap();
    let response_channel = rabbitmq_client.create_channel().await.unwrap();
    let mut job_consumer = setup_job_consumer(job_channel, args.rabbitmq_channel)
        .await
        .unwrap();

    let (chimp_image_tx, chimp_image_rx) = tokio::sync::mpsc::channel(batch_size);
    let (well_image_tx, mut well_image_rx) = tokio::sync::mpsc::channel(batch_size);
    let (well_location_tx, mut well_location_rx) = tokio::sync::mpsc::unbounded_channel();
    let (prediction_tx, mut prediction_rx) = tokio::sync::mpsc::unbounded_channel();
    let (contents_tx, mut contents_rx) = tokio::sync::mpsc::unbounded_channel();

    let mut tasks = JoinSet::new();

    tasks.spawn(inference_worker(
        session,
        batch_size,
        chimp_image_rx,
        prediction_tx,
    ));

    let mut well_locations = HashMap::new();
    let mut well_contents = HashMap::new();

    loop {
        select! {
            biased;

            Some(delivery) = job_consumer.next() =>  {
                tasks.spawn(consume_job(delivery, input_width, input_height, chimp_image_tx.clone(), well_image_tx.clone()));
            }

            Some((well_image, job)) = well_image_rx.recv() =>  {
                tasks.spawn(find_well_center(well_image, job, well_location_tx.clone()));
            }

            Some((bboxes, labels, _, masks, job)) = prediction_rx.recv() => {
                tasks.spawn(postprocess_inference(bboxes, labels, masks, job, contents_tx.clone()));
            }

            Some((well_location, job)) = well_location_rx.recv() => {
                if let Some(contents) = well_contents.remove(&job.id) {
                    tasks.spawn(produce_response(contents, well_location, job, response_channel.clone()));
                } else {
                    well_locations.insert(job.id, well_location);
                }
            }

            Some((contents, job)) = contents_rx.recv() => {
                if let Some(well_location) = well_locations.remove(&job.id) {
                    tasks.spawn(produce_response(contents, well_location, job, response_channel.clone()));
                } else {
                    well_contents.insert(job.id, contents);
                }
            }

            else => break
        }
    }
}
