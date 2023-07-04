use image::{imageops::FilterType, ImageFormat};
use ndarray::{ArrayBase, Axis, Dim, IxDynImpl, OwnedRepr};
use nshare::ToNdarray3;
use std::{fs::File, io::BufReader, path::Path};

pub fn load_image(
    path: impl AsRef<Path>,
    width: u32,
    height: u32,
) -> ArrayBase<OwnedRepr<f32>, Dim<IxDynImpl>> {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    image::load(reader, ImageFormat::Jpeg)
        .unwrap()
        .resize_exact(width, height, FilterType::Triangle)
        .into_rgb32f()
        .into_ndarray3()
        .insert_axis(Axis(0))
        .into_dyn()
}
