//! TFLite model inference for Chrona

use core_types::Sample;
use crate::Model;
use tract_tflite::prelude::*;
use tract_tflite::prelude::{tvec, Tensor, TypedFact, TypedOp, Graph, SimplePlan};

/// TFLite model wrapper (stub; real implementation requires tflite crate or FFI)
/// TFLite model wrapper for running inference on sensor data.
pub struct TFLiteModel {
    /// Path to the TFLite model file.
    pub model_path: String,
    model: SimplePlan<TypedFact, Box<dyn TypedOp>, Graph<TypedFact, Box<dyn TypedOp>>>,
}

impl TFLiteModel {
    /// Create a new TFLiteModel from a file path and expected window size.
    pub fn new(model_path: &str, window_size: usize) -> Self {
        let model = tract_tflite::tflite()
            .model_for_path(model_path)
            .expect("Failed to load TFLite model file")
            .with_input_fact(0, TypedFact::dt_shape(f32::datum_type(), tvec![1, window_size]))
            .expect("Failed to set input fact")
            .into_optimized()
            .expect("Failed to optimize model")
            .into_runnable()
            .expect("Failed to make model runnable");
        TFLiteModel {
            model_path: model_path.to_string(),
            model,
        }
    }

    /// Run inference on a window of samples (flattened to f32 values).
    pub fn infer_samples(&self, window: &[Sample]) -> f32 {
        let input: Tensor = tract_ndarray::Array2::from_shape_vec((1, window.len()), window.iter().map(|s| s.value).collect()).unwrap().into();
        let result = self.model.run(tvec!(input.into())).expect("TFLite inference failed");
        result[0].to_array_view::<f32>().unwrap().iter().next().copied().unwrap_or(0.0)
    }
}

impl Model for TFLiteModel {
    fn id(&self) -> &str {
        &self.model_path
    }

    fn infer(&self, window: &[Sample]) -> f32 {
        self.infer_samples(window)
    }
}
