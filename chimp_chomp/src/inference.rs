use chimp_protocol::{Prediction, Predictions};
use itertools::{izip, Itertools};
use ndarray::{ArrayBase, Axis, Dim, Ix1, Ix2, IxDynImpl, ViewRepr};
use ort::{
    tensor::{FromArray, InputTensor},
    Environment, ExecutionProvider, GraphOptimizationLevel, OrtError, Session, SessionBuilder,
};
use std::{path::Path, sync::Arc};
use tokio::sync::mpsc::{Receiver, UnboundedSender};

use crate::image_loading::Image;

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
    images: &[ArrayBase<ViewRepr<&f32>, Dim<IxDynImpl>>],
    batch_size: usize,
) -> Vec<Predictions> {
    let images = images
        .iter()
        .cloned()
        .chain(std::iter::repeat(images[0].clone()).take(batch_size - images.len()))
        .collect::<Vec<_>>();
    let input = InputTensor::from_array(ndarray::concatenate(Axis(0), &images).unwrap());
    let outputs = session.run(vec![input]).unwrap();
    outputs
        .into_iter()
        .tuples()
        .map(|(bboxes, labels, scores, _)| {
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

            Predictions(
                izip!(
                    bboxes.outer_iter(),
                    labels.to_vec().iter(),
                    scores.to_vec().iter()
                )
                .map(|(bbox, &label, &score)| Prediction {
                    bbox: bbox.to_vec().try_into().unwrap(),
                    label,
                    score,
                })
                .collect(),
            )
        })
        .collect()
}

pub async fn inference_worker(
    session: Session,
    batch_size: usize,
    mut image_rx: Receiver<(Image, String)>,
    prediction_tx: UnboundedSender<(Predictions, String)>,
) {
    image_rx
        .recv()
        .await
        .iter()
        .map(|(image, predictions_channel)| (image.view(), predictions_channel))
        .chunks(batch_size)
        .into_iter()
        .for_each(|jobs| {
            let (images, prediction_channels) = jobs.into_iter().unzip::<_, _, Vec<_>, Vec<_>>();
            let predictions = do_inference(&session, &images, batch_size);
            izip!(predictions.into_iter(), prediction_channels.into_iter()).for_each(
                |(predictions, prediction_channel)| {
                    prediction_tx
                        .send((predictions, prediction_channel.clone()))
                        .unwrap()
                },
            )
        });
}
