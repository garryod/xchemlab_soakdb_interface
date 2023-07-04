use ort::{
    Environment, ExecutionProvider, GraphOptimizationLevel, OrtError, Session, SessionBuilder,
};
use std::{path::Path, sync::Arc};

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
