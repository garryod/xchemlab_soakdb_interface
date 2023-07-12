use crate::{
    image_loading::{load_image, ChimpImage, WellImage},
    postprocessing::Contents,
};
use chimp_protocol::{Circle, Job, Response};
use lapin::{
    message::Delivery,
    options::{BasicAckOptions, BasicConsumeOptions, BasicPublishOptions},
    types::FieldTable,
    BasicProperties, Channel, Connection, Consumer,
};
use tokio::sync::mpsc::Sender;
use url::Url;
use uuid::Uuid;

pub async fn setup_rabbitmq_client(address: Url) -> Result<Connection, lapin::Error> {
    lapin::Connection::connect(address.as_str(), lapin::ConnectionProperties::default()).await
}

pub async fn setup_job_consumer(
    rabbitmq_channel: Channel,
    channel: impl AsRef<str>,
) -> Result<Consumer, lapin::Error> {
    let worker_id = Uuid::new_v4();
    let worker_tag = format!("chimp_chomp_{worker_id}");
    rabbitmq_channel
        .basic_consume(
            channel.as_ref(),
            &worker_tag,
            BasicConsumeOptions::default(),
            FieldTable::default(),
        )
        .await
}

pub async fn consume_job(
    delivery: Result<Delivery, lapin::Error>,
    input_width: u32,
    input_height: u32,
    chimp_image_tx: Sender<(ChimpImage, Job)>,
    well_image_tx: Sender<(WellImage, Job)>,
) {
    let delievry = delivery.unwrap();
    delievry.ack(BasicAckOptions::default()).await.unwrap();

    let job = Job::from_slice(&delievry.data).unwrap();
    println!("Consumed Job: {job:?}");
    let (chimp_image, well_image) = load_image(job.file.clone(), input_width, input_height);

    chimp_image_tx
        .send((chimp_image, job.clone()))
        .await
        .unwrap();
    well_image_tx.send((well_image, job)).await.unwrap();
}

pub async fn produce_response(
    contents: Contents,
    well_location: Circle,
    job: Job,
    rabbitmq_channel: Channel,
) {
    println!("Producing response for: {job:?}");
    rabbitmq_channel
        .basic_publish(
            "",
            &job.predictions_channel,
            BasicPublishOptions::default(),
            &Response {
                job_id: job.id,
                insertion_point: contents.insertion_point,
                well_location,
                drop: contents.drop,
                crystals: contents.crystals,
            }
            .to_vec()
            .unwrap(),
            BasicProperties::default(),
        )
        .await
        .unwrap()
        .await
        .unwrap();
}
