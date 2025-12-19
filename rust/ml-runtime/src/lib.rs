#![deny(missing_docs)]

//! TinyML/ONNX/TFLite inference shims.

use core_types::Sample;

/// Basic interface for running a model over a window of samples.
pub trait Model {
    /// Human-readable model id.
    fn id(&self) -> &str;
    /// Run inference over a slice of samples, returning a score/value.
    fn infer(&self, window: &[Sample]) -> f32;
}

/// No-op model for wiring.
pub struct NoopModel;

impl Model for NoopModel {
    fn id(&self) -> &str {
        "noop"
    }

    fn infer(&self, _window: &[Sample]) -> f32 {
        0.0
    }
}

