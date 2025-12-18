#![deny(missing_docs)]

//! TinyML/ONNX/TFLite inference shims.

use core_types::Sample;
use data_layer::MhealthRecord;

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

/// Activity classifier using statistical features from sensor data.
/// Works with MHEALTH dataset (L1-L12 activities).
pub struct ActivityClassifier;

impl ActivityClassifier {
    /// Classify activity from a window of sensor readings.
    /// Returns the predicted activity label (1-12) and confidence (0.0-1.0).
    pub fn classify(records: &[MhealthRecord]) -> (u8, f32) {
        if records.is_empty() {
            return (1, 0.0); // Default to L1 (standing) with low confidence
        }

        // Extract statistical features from sensor data
        let (accel_rms, gyro_rms) = Self::compute_features(records);

        // Simple threshold-based classification
        // In 16GB tier, this becomes a trained model
        let activity = Self::threshold_classify(accel_rms, gyro_rms);
        let confidence = Self::compute_confidence(accel_rms, gyro_rms);

        (activity, confidence)
    }

    fn compute_features(records: &[MhealthRecord]) -> (f32, f32) {
        let mut accel_sum = 0.0;
        let mut gyro_sum = 0.0;

        for r in records {
            // Acceleration magnitude
            let accel_mag = (r.al_x.powi(2)
                + r.al_y.powi(2)
                + r.al_z.powi(2)
                + r.ar_x.powi(2)
                + r.ar_y.powi(2)
                + r.ar_z.powi(2))
            .sqrt();
            accel_sum += accel_mag;

            // Gyro magnitude
            let gyro_mag = (r.gl_x.powi(2)
                + r.gl_y.powi(2)
                + r.gl_z.powi(2)
                + r.gr_x.powi(2)
                + r.gr_y.powi(2)
                + r.gr_z.powi(2))
            .sqrt();
            gyro_sum += gyro_mag;
        }

        let accel_rms = (accel_sum / records.len() as f32).sqrt();
        let gyro_rms = (gyro_sum / records.len() as f32).sqrt();

        (accel_rms, gyro_rms)
    }

    fn threshold_classify(accel_rms: f32, gyro_rms: f32) -> u8 {
        match (accel_rms, gyro_rms) {
            // Very low motion: standing/sitting/lying
            (a, _) if a < 0.5 => match gyro_rms {
                g if g < 0.1 => 2, // L2: Sitting
                _ => 1,            // L1: Standing
            },
            // Low-moderate motion: walking, arm elevation
            (a, g) if a < 1.5 && g < 1.0 => match gyro_rms {
                g if g > 0.8 => 7, // L7: Arm elevation
                _ => 4,            // L4: Walking
            },
            // Moderate motion: climbing, cycling
            (a, g) if a < 2.5 && g < 1.5 => match gyro_rms {
                g if g > 1.2 => 5, // L5: Climbing stairs
                _ => 9,            // L9: Cycling
            },
            // Moderate-high motion: bending, crouching
            (a, g) if a < 3.5 && g > 1.0 => match a {
                a if a > 3.0 => 8, // L8: Knees bending
                _ => 6,            // L6: Waist bends
            },
            // High motion: jogging, running, jumping
            (a, _) if a >= 3.5 => match a {
                a if a > 4.5 => 11, // L11: Running
                a if a > 4.0 => 12, // L12: Jumping
                _ => 10,            // L10: Jogging
            },
            _ => 1, // Default to standing
        }
    }

    fn compute_confidence(accel_rms: f32, _gyro_rms: f32) -> f32 {
        // Higher confidence when motion is clear (not ambiguous middle values)
        if accel_rms < 0.3 || accel_rms > 4.5 {
            0.85
        } else if accel_rms < 0.8 || accel_rms > 3.5 {
            0.75
        } else {
            0.65
        }
    }
}
