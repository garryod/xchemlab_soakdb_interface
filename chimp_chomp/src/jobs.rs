use crate::image_loading::{load_image, Image};
use chimp_protocol::{Job, Predictions};
use futures_lite::StreamExt;
use lapin::{
    options::{BasicAckOptions, BasicConsumeOptions, BasicPublishOptions},
    types::FieldTable,
    BasicProperties, Channel, Connection, Consumer,
};
use tokio::sync::mpsc::{Sender, UnboundedReceiver};
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

pub async fn job_consumption_worker(
    mut job_consumer: Consumer,
    input_width: u32,
    input_height: u32,
    image_tx: Sender<(Image, String)>,
) {
    while let Some(delivery) = job_consumer.next().await {
        let delievry = delivery.unwrap();
        delievry.ack(BasicAckOptions::default()).await.unwrap();

        let job = Job::from_slice(&delievry.data).unwrap();
        let image = load_image(job.file, input_width, input_height);

        image_tx
            .send((image, job.predictions_channel))
            .await
            .unwrap();
    }
}

pub async fn predictions_producer_worker(
    mut prediction_rx: UnboundedReceiver<(Predictions, String)>,
    rabbitmq_channel: Channel,
) {
    while let Some((predictions, predictions_channel)) = prediction_rx.recv().await {
        rabbitmq_channel
            .basic_publish(
                "",
                &predictions_channel,
                BasicPublishOptions::default(),
                &predictions.to_vec().unwrap(),
                BasicProperties::default(),
            )
            .await
            .unwrap()
            .await
            .unwrap();
    }
}
