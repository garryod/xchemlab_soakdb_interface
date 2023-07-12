use derive_more::Deref;
use ndarray::{Array, Ix3};
use opencv::{
    core::{Size_, Vec3f, CV_32FC3},
    imgcodecs::{imread, IMREAD_COLOR},
    imgproc::{cvt_color, resize, COLOR_BGR2GRAY, COLOR_BGR2RGB, INTER_LINEAR},
    prelude::{Mat, MatTraitConst},
};
use std::path::Path;

#[derive(Debug, Deref)]
pub struct WellImage(Mat);

#[derive(Debug, Deref)]
pub struct ChimpImage(Array<f32, Ix3>);

pub fn load_image(path: impl AsRef<Path>, width: u32, height: u32) -> (ChimpImage, WellImage) {
    let image = imread(path.as_ref().to_str().unwrap(), IMREAD_COLOR).unwrap();

    let mut resized_image = Mat::default();
    resize(
        &image,
        &mut resized_image,
        Size_ {
            width: width as i32,
            height: height as i32,
        },
        0.0,
        0.0,
        INTER_LINEAR,
    )
    .unwrap();

    let mut well_image = Mat::default();
    cvt_color(&resized_image, &mut well_image, COLOR_BGR2GRAY, 0).unwrap();

    let mut resized_rgb_image = Mat::default();
    cvt_color(&resized_image, &mut resized_rgb_image, COLOR_BGR2RGB, 0).unwrap();
    let mut resized_rgb_f32_image = Mat::default();

    resized_rgb_image
        .convert_to(
            &mut resized_rgb_f32_image,
            CV_32FC3,
            f64::from(std::u8::MAX).recip(),
            0.0,
        )
        .unwrap();
    let chimp_image = Array::from_iter(
        resized_rgb_f32_image
            .iter::<Vec3f>()
            .unwrap()
            .flat_map(|(_, pixel)| pixel),
    )
    .into_shape((
        resized_rgb_f32_image.mat_size()[0] as usize,
        resized_rgb_f32_image.mat_size()[1] as usize,
        resized_rgb_f32_image.channels() as usize,
    ))
    .unwrap()
    .permuted_axes((2, 0, 1))
    .as_standard_layout()
    .to_owned();

    (ChimpImage(chimp_image), WellImage(well_image))
}
