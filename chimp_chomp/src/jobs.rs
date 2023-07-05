use crate::image_loading::load_image;
use chimp_protocol::Job;
use futures_lite::StreamExt;
use lapin::{
    options::{BasicAckOptions, BasicConsumeOptions},
    types::FieldTable,
    Connection, Consumer,
};
use ndarray::{ArrayBase, Dim, IxDynImpl, OwnedRepr};
use tokio::sync::mpsc::Sender;
use url::Url;
use uuid::Uuid;

pub async fn setup_rabbitmq_client(address: Url) -> Result<Connection, lapin::Error> {
    lapin::Connection::connect(address.as_str(), lapin::ConnectionProperties::default()).await
}

pub async fn setup_job_consumer(
    rabbitmq_client: Connection,
    channel: impl AsRef<str>,
) -> Result<Consumer, lapin::Error> {
    let worker_id = Uuid::new_v4();
    let worker_tag = format!("chimp_chomp_{worker_id}");
    let job_channel = rabbitmq_client.create_channel().await?;
    job_channel
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
    image_tx: Sender<ArrayBase<OwnedRepr<f32>, Dim<IxDynImpl>>>,
) {
    while let Some(delivery) = job_consumer.next().await {
        let delievry = delivery.unwrap();
        delievry.ack(BasicAckOptions::default()).await.unwrap();

        let job = Job::from_slice(&delievry.data).unwrap();
        let image = load_image(job.file, input_width, input_height);

        image_tx.send(image).await.unwrap();
    }
}
