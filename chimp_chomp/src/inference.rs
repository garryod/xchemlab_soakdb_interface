use chimp_protocol::{Prediction, Predictions};
use itertools::{izip, Itertools};
use ndarray::{ArrayBase, Axis, Dim, Ix2, Ix3, IxDynImpl, OwnedRepr, ViewRepr};
use ort::{
    tensor::{FromArray, InputTensor},
    Environment, ExecutionProvider, GraphOptimizationLevel, OrtError, Session, SessionBuilder,
};
use std::{path::Path, sync::Arc};
use tokio::sync::mpsc::{Receiver, UnboundedSender};

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
) -> Vec<Predictions> {
    let input = InputTensor::from_array(ndarray::concatenate(Axis(0), images).unwrap());
    let outputs = session.run(vec![input]).unwrap();
    let bboxes = outputs[0]
        .try_extract::<f32>()
        .unwrap()
        .view()
        .to_owned()
        .into_dimensionality::<Ix3>()
        .unwrap();
    let labels = outputs[1]
        .try_extract::<i64>()
        .unwrap()
        .view()
        .to_owned()
        .into_dimensionality::<Ix2>()
        .unwrap();
    let scores = outputs[2]
        .try_extract::<f32>()
        .unwrap()
        .view()
        .to_owned()
        .into_dimensionality::<Ix2>()
        .unwrap();

    izip!(
        bboxes.outer_iter(),
        labels.outer_iter(),
        scores.outer_iter()
    )
    .map(|(bboxes, labels, scores)| {
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
    mut image_rx: Receiver<ArrayBase<OwnedRepr<f32>, Dim<IxDynImpl>>>,
    prediction_tx: UnboundedSender<Predictions>,
) {
    image_rx
        .recv()
        .await
        .iter()
        .map(ArrayBase::view)
        .chunks(batch_size)
        .into_iter()
        .for_each(|images| {
            let images = images.collect::<Vec<_>>();
            let predictions = do_inference(&session, &images);
            predictions
                .into_iter()
                .for_each(|predictions| prediction_tx.send(predictions).unwrap())
        });
}
