use crate::inference::{BBoxes, Labels, Masks};
use chimp_protocol::{BBox, Job, Point};
use itertools::izip;
use ndarray::{Array2, ArrayView, ArrayView2, Ix1};
use opencv::{
    core::CV_8U,
    imgproc::{distance_transform, DIST_L1, DIST_MASK_3},
    prelude::Mat,
};
use tokio::sync::mpsc::UnboundedSender;

#[derive(Debug)]
pub struct Contents {
    pub insertion_point: Point,
    pub drop: BBox,
    pub crystals: Vec<BBox>,
}

const PREDICTION_THRESHOLD: f32 = 0.5;

fn insertion_mask(
    drop_mask: ArrayView2<f32>,
    crystal_masks: Vec<ArrayView2<'_, f32>>,
) -> Array2<bool> {
    let mut mask = drop_mask.mapv(|prediction| prediction > PREDICTION_THRESHOLD);
    crystal_masks.into_iter().for_each(|crystal_mask| {
        mask.zip_mut_with(&crystal_mask, |valid, prediction| {
            *valid &= *prediction < PREDICTION_THRESHOLD
        })
    });
    mask
}

fn optimal_insert_position(insertion_mask: Array2<bool>) -> Point {
    let mask = Mat::from_exact_iter(insertion_mask.mapv(|pixel| pixel as u8).into_iter()).unwrap();
    let mut distances = Mat::default();
    distance_transform(&mask, &mut distances, DIST_L1, DIST_MASK_3, CV_8U).unwrap();
    let (furthest_point, _) = distances
        .iter::<u8>()
        .unwrap()
        .max_by(|(_, a), (_, b)| a.cmp(b))
        .unwrap();
    Point {
        x: furthest_point.x as usize,
        y: furthest_point.y as usize,
    }
}

fn bbox_from_array(bbox: ArrayView<f32, Ix1>) -> BBox {
    BBox {
        left: bbox[0],
        top: bbox[1],
        right: bbox[2],
        bottom: bbox[3],
    }
}

fn find_drop_instance<'a>(
    labels: &Labels,
    bboxes: &BBoxes,
    masks: &'a Masks,
) -> Option<(BBox, ArrayView2<'a, f32>)> {
    izip!(labels, bboxes.outer_iter(), masks.outer_iter())
        .find_map(|(label, bbox, mask)| (*label == 1).then_some((bbox_from_array(bbox), mask)))
}

fn find_crystal_instances<'a>(
    labels: &Labels,
    bboxes: &BBoxes,
    masks: &'a Masks,
) -> Vec<(BBox, ArrayView2<'a, f32>)> {
    izip!(labels, bboxes.outer_iter(), masks.outer_iter())
        .filter_map(|(label, bbox, mask)| (*label == 2).then_some((bbox_from_array(bbox), mask)))
        .collect()
}

pub async fn postprocess_inference(
    bboxes: BBoxes,
    labels: Labels,
    masks: Masks,
    job: Job,
    contents_tx: UnboundedSender<(Contents, Job)>,
) {
    println!("Postprocessing: {job:?}");
    let (drop, drop_mask) = find_drop_instance(&labels, &bboxes, &masks).unwrap();
    let (crystals, crystal_masks) = find_crystal_instances(&labels, &bboxes, &masks)
        .into_iter()
        .unzip();
    let insertion_mask = insertion_mask(drop_mask, crystal_masks);
    let insertion_point = optimal_insert_position(insertion_mask);
    let contents = Contents {
        drop,
        crystals,
        insertion_point,
    };
    contents_tx.send((contents, job)).unwrap();
}
