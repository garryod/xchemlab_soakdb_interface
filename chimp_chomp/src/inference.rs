use chimp_protocol::Job;
use itertools::{izip, Itertools};
use ndarray::{Array1, Array2, Array3, Axis, Ix1, Ix2, Ix4};
use ort::{
    tensor::{FromArray, InputTensor},
    Environment, ExecutionProvider, GraphOptimizationLevel, OrtError, Session, SessionBuilder,
};
use std::{ops::Deref, path::Path, sync::Arc};
use tokio::sync::mpsc::{error::TryRecvError, Receiver, UnboundedSender};

use crate::image_loading::ChimpImage;

pub type BBoxes = Array2<f32>;
pub type Labels = Array1<i64>;
pub type Scores = Array1<f32>;
pub type Masks = Array3<f32>;

pub fn setup_inference_session(model_path: impl AsRef<Path>) -> Result<Session, OrtError> {
    let environment = Arc::new(
        Environment::builder()
            .with_name("CHiMP")
            .with_execution_providers([ExecutionProvider::cpu()])
            .build()?,
    );
    SessionBuilder::new(&environment)?
        .with_optimization_level(GraphOptimizationLevel::Level3)?
        .with_model_from_file(model_path)
}

fn do_inference(
    session: &Session,
    images: &[ChimpImage],
    batch_size: usize,
) -> Vec<(BBoxes, Labels, Scores, Masks)> {
    let batch_images = images
        .iter()
        .map(|image| image.deref().view())
        .cycle()
        .take(batch_size)
        .collect::<Vec<_>>();
    let input = InputTensor::from_array(ndarray::stack(Axis(0), &batch_images).unwrap().into_dyn());
    let outputs = session.run(vec![input]).unwrap();
    outputs
        .into_iter()
        .take(images.len() * 4)
        .tuples()
        .map(|(bboxes, labels, scores, masks)| {
            let bboxes = bboxes
                .try_extract::<f32>()
                .unwrap()
                .view()
                .to_owned()
                .into_dimensionality::<Ix2>()
                .unwrap();
            let labels = labels
                .try_extract::<i64>()
                .unwrap()
                .view()
                .to_owned()
                .into_dimensionality::<Ix1>()
                .unwrap();
            let scores = scores
                .try_extract::<f32>()
                .unwrap()
                .view()
                .to_owned()
                .into_dimensionality::<Ix1>()
                .unwrap();
            let masks = masks
                .try_extract::<f32>()
                .unwrap()
                .view()
                .to_owned()
                .into_dimensionality::<Ix4>()
                .unwrap()
                .remove_axis(Axis(1));

            (bboxes, labels, scores, masks)
        })
        .collect()
}

pub async fn inference_worker(
    session: Session,
    batch_size: usize,
    mut image_rx: Receiver<(ChimpImage, Job)>,
    prediction_tx: UnboundedSender<(BBoxes, Labels, Scores, Masks, Job)>,
) {
    let mut images = Vec::new();
    let mut jobs = Vec::<Job>::new();
    loop {
        let (image, job) = image_rx.recv().await.unwrap();
        println!("Got image for job: {job:?}");
        images.push(image);
        jobs.push(job);
        while images.len() < batch_size {
            match image_rx.try_recv() {
                Ok((image, job)) => {
                    images.push(image);
                    jobs.push(job);
                    Ok(())
                }
                Err(TryRecvError::Empty) => break,
                Err(TryRecvError::Disconnected) => Err(TryRecvError::Disconnected),
            }
            .unwrap();
        }
        println!("Performing inference on {} images", images.len());
        let predictions = do_inference(&session, &images, batch_size);
        izip!(predictions.into_iter(), jobs.iter()).for_each(
            |((bboxes, labels, scores, masks), job)| {
                prediction_tx
                    .send((bboxes, labels, scores, masks, job.clone()))
                    .unwrap();
            },
        );
        images.clear();
        jobs.clear();
    }
}
