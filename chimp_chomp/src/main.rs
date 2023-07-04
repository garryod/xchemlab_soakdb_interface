mod model;

use crate::model::setup_inference_session;
use clap::Parser;
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about=None)]
struct Cli {
    /// The path to the ONNX model file.
    model: PathBuf,
}

fn main() {
    dotenvy::dotenv().ok();
    let args = Cli::parse();

    let _session = setup_inference_session(args.model).unwrap();
}
