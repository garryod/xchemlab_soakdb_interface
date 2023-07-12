use std::ops::Deref;

use crate::image_loading::WellImage;
use chimp_protocol::{Circle, Job, Point};
use opencv::{
    core::{Vec4f, Vector},
    imgproc::{hough_circles, HOUGH_GRADIENT},
    prelude::MatTraitConst,
};
use tokio::sync::mpsc::UnboundedSender;

pub async fn find_well_center(
    image: WellImage,
    job: Job,
    well_location_tx: UnboundedSender<(Circle, Job)>,
) {
    println!("Finding Well Center for {job:?}");
    let max_side = *image.deref().mat_size().iter().max().unwrap();
    let mut circles = Vector::<Vec4f>::new();
    hough_circles(
        &*image,
        &mut circles,
        HOUGH_GRADIENT,
        1.0,
        1.0,
        10.0,
        10.0,
        max_side / 2,
        max_side,
    )
    .unwrap();
    let well_location = circles
        .into_iter()
        .max_by(|&a, &b| a[3].total_cmp(&b[3]))
        .unwrap();
    let well_location = Circle {
        center: Point {
            x: well_location[0] as usize,
            y: well_location[1] as usize,
        },
        radius: well_location[2],
    };
    well_location_tx.send((well_location, job)).unwrap()
}
