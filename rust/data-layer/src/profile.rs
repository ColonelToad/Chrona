//! User profile types and preset archetypes for synthetic data generation.

/// Activity kind (used in daily schedule).
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ActivityKind {
    /// Sleep state.
    Sleep,
    /// Sitting (desk, car, couch).
    Sitting,
    /// Standing (standing desk, counter).
    Standing,
    /// Walking (slow pace, ~3 mph).
    WalkingSlow,
    /// Walking (brisk pace, ~4–5 mph).
    WalkingBrisk,
    /// Stairs (climbing or descending).
    Stairs,
    /// Cycling (easy pace).
    CyclingEasy,
    /// Cycling (hard pace).
    CyclingHard,
    /// Running (easy pace, ~5–6 mph).
    RunningEasy,
    /// Running (moderate pace, ~6–8 mph).
    RunningModerate,
    /// Running (hard pace, ~8+ mph).
    RunningHard,
    /// Gym / strength training.
    GymModerate,
    /// Gym / high intensity.
    GymHigh,
}

/// Exercise type used in exercise schedule.
#[derive(Debug, Clone, Copy)]
pub enum ExerciseType {
    /// Running (easy, moderate, hard variants handled via time blocks).
    Running,
    /// Cycling (easy, hard variants).
    Cycling,
    /// Gym / strength training.
    Gym,
    /// Brisk walking.
    BriskWalking,
}

/// Fitness level (coarse descriptor).
#[derive(Debug, Clone, Copy)]
pub enum FitnessLevel {
    /// Sedentary (<2000 steps/day).
    Sedentary,
    /// Low (2000–5000 steps/day).
    Low,
    /// Moderate (5000–10000 steps/day).
    Moderate,
    /// Active (10000–15000 steps/day).
    Active,
    /// Very active (>15000 steps/day).
    VeryActive,
}

/// User profile archetype.
#[derive(Debug, Clone, Copy)]
pub enum ProfileType {
    /// Business professional (sedentary, stressful).
    BusinessProfessional,
    /// College student (moderate activity, variable stress).
    CollegeStudent,
    /// Professional athlete (high activity, low stress).
    ProAthlete,
    /// Shift worker (disrupted sleep, high stress).
    ShiftWorker,
    /// Remote worker (consistent, less commute stress).
    RemoteWorker,
}

/// Distribution of exercise intensity preferences (low/moderate/high).
#[derive(Debug, Clone, Copy)]
pub struct IntensityDistribution {
    /// Fraction preferring low intensity.
    pub low: f32,
    /// Fraction preferring moderate intensity.
    pub moderate: f32,
    /// Fraction preferring high intensity.
    pub high: f32,
}

impl IntensityDistribution {
    /// Normalize so proportions sum to 1.0.
    pub fn normalized(mut self) -> Self {
        let sum = (self.low + self.moderate + self.high).max(1e-6);
        self.low /= sum;
        self.moderate /= sum;
        self.high /= sum;
        self
    }
}

/// A time block in a daily schedule.
#[derive(Debug, Clone, Copy)]
pub struct ScheduleBlock {
    /// Start minute of day (0–1439).
    pub start_minute: u16,
    /// Duration in minutes.
    pub duration_min: u16,
    /// Activity kind during this block.
    pub kind: ActivityKind,
}

/// Repeating daily schedule template.
#[derive(Debug, Clone)]
pub struct DailySchedule {
    /// Ordered blocks covering the day.
    pub blocks: Vec<ScheduleBlock>,
}

impl DailySchedule {
    /// Return the current activity kind at a given minute-of-day (0–1439).
    pub fn activity_at_minute(&self, minute: u16) -> ActivityKind {
        for block in &self.blocks {
            let start = block.start_minute as u32;
            let end = start + block.duration_min as u32;
            let m = minute as u32;
            if m >= start && m < end {
                return block.kind;
            }
        }
        ActivityKind::Sitting // fallback
    }
}

/// Complete user profile for synthetic data generation.
#[derive(Debug, Clone)]
pub struct UserProfile {
    /// Profile archetype.
    pub profile_type: ProfileType,
    /// Age in years.
    pub age: u8,
    /// Fitness level.
    pub fitness_level: FitnessLevel,

    // Physiological baselines
    /// Resting heart rate in bpm.
    pub resting_hr: f32,
    /// Max heart rate in bpm.
    pub max_hr: f32,
    /// HRV baseline (RMSSD) in ms at rest.
    pub hrv_baseline: f32,
    /// Baseline skin temperature in °C.
    pub baseline_temp: f32,
    /// Baseline EDA in μS (electrodermal activity).
    pub baseline_eda: f32,

    // Behavior/schedule
    /// Typical sleep start time (minute-of-day, 0–1439).
    pub typical_sleep_start_min: u16,
    /// Typical sleep duration in minutes.
    pub typical_sleep_duration_min: u16,
    /// Daily activity schedule.
    pub activity_schedule: DailySchedule,
    /// Stress sensitivity (0.0–1.0); higher → more responsive to stressors.
    pub stress_sensitivity: f32,

    // Exercise
    /// Types of exercise the user does.
    pub exercise_types: Vec<ExerciseType>,
    /// Target exercise frequency (times per week).
    pub exercise_frequency_per_week: usize,
    /// Preferred intensity distribution.
    pub exercise_intensity_pref: IntensityDistribution,
}

/// Map an activity kind to a nominal intensity [0.0, 1.0].
pub fn activity_intensity(kind: ActivityKind) -> f32 {
    match kind {
        ActivityKind::Sleep => 0.0,
        ActivityKind::Sitting => 0.05,
        ActivityKind::Standing => 0.10,
        ActivityKind::WalkingSlow => 0.25,
        ActivityKind::WalkingBrisk => 0.40,
        ActivityKind::Stairs => 0.50,
        ActivityKind::CyclingEasy => 0.35,
        ActivityKind::CyclingHard => 0.60,
        ActivityKind::RunningEasy => 0.55,
        ActivityKind::RunningModerate => 0.70,
        ActivityKind::RunningHard => 0.85,
        ActivityKind::GymModerate => 0.45,
        ActivityKind::GymHigh => 0.65,
    }
}

/// Map an activity kind to a nominal HR boost (above resting).
pub fn activity_hr_boost(kind: ActivityKind, resting_hr: f32, max_hr: f32) -> f32 {
    let intensity = activity_intensity(kind);
    (max_hr - resting_hr) * intensity
}

/// Map a stress level [0.0, 1.0] to an HR boost in bpm.
pub fn stress_hr_boost(stress_level: f32) -> f32 {
    stress_level * 8.0
}

/// Map an activity kind to a nominal accel magnitude (g).
pub fn activity_accel_mean(kind: ActivityKind) -> f32 {
    match kind {
        ActivityKind::Sleep => 0.15,
        ActivityKind::Sitting => 1.02,
        ActivityKind::Standing => 1.08,
        ActivityKind::WalkingSlow => 1.85,
        ActivityKind::WalkingBrisk => 2.35,
        ActivityKind::Stairs => 2.80,
        ActivityKind::CyclingEasy => 1.65,
        ActivityKind::CyclingHard => 1.80,
        ActivityKind::RunningEasy => 3.50,
        ActivityKind::RunningModerate => 4.20,
        ActivityKind::RunningHard => 4.80,
        ActivityKind::GymModerate => 2.00,
        ActivityKind::GymHigh => 2.50,
    }
}

/// Map an activity kind to accel noise std dev (g).
pub fn activity_accel_noise(kind: ActivityKind) -> f32 {
    match kind {
        ActivityKind::Sleep => 0.08,
        ActivityKind::Sitting => 0.12,
        ActivityKind::Standing => 0.18,
        ActivityKind::WalkingSlow => 0.45,
        ActivityKind::WalkingBrisk => 0.65,
        ActivityKind::Stairs => 0.85,
        ActivityKind::CyclingEasy => 0.55,
        ActivityKind::CyclingHard => 0.65,
        ActivityKind::RunningEasy => 1.00,
        ActivityKind::RunningModerate => 1.20,
        ActivityKind::RunningHard => 1.40,
        ActivityKind::GymModerate => 0.60,
        ActivityKind::GymHigh => 0.75,
    }
}

/// Preset profiles.
pub mod presets {
    use super::*;

    /// Create a BusinessProfessional profile.
    pub fn business_professional() -> UserProfile {
        UserProfile {
            profile_type: ProfileType::BusinessProfessional,
            age: 35,
            fitness_level: FitnessLevel::Low,
            resting_hr: 68.0,
            max_hr: 185.0,
            hrv_baseline: 45.0,
            baseline_temp: 36.2,
            baseline_eda: 2.5,
            typical_sleep_start_min: 22 * 60 + 30, // 22:30
            typical_sleep_duration_min: 7 * 60 + 30, // 7h 30m
            activity_schedule: DailySchedule {
                blocks: vec![
                    ScheduleBlock { start_minute: 0, duration_min: 360, kind: ActivityKind::Sleep },
                    ScheduleBlock { start_minute: 360, duration_min: 60, kind: ActivityKind::Standing },
                    ScheduleBlock { start_minute: 420, duration_min: 60, kind: ActivityKind::Sitting },
                    ScheduleBlock { start_minute: 480, duration_min: 240, kind: ActivityKind::Sitting },
                    ScheduleBlock { start_minute: 720, duration_min: 25, kind: ActivityKind::Sitting },
                    ScheduleBlock { start_minute: 745, duration_min: 35, kind: ActivityKind::WalkingSlow },
                    ScheduleBlock { start_minute: 780, duration_min: 240, kind: ActivityKind::Sitting },
                    ScheduleBlock { start_minute: 1020, duration_min: 60, kind: ActivityKind::WalkingBrisk },
                    ScheduleBlock { start_minute: 1080, duration_min: 60, kind: ActivityKind::Standing },
                    ScheduleBlock { start_minute: 1140, duration_min: 120, kind: ActivityKind::Sitting },
                    ScheduleBlock { start_minute: 1260, duration_min: 60, kind: ActivityKind::Sitting },
                    ScheduleBlock { start_minute: 1320, duration_min: 180, kind: ActivityKind::Sleep },
                ],
            },
            stress_sensitivity: 0.7,
            exercise_types: vec![ExerciseType::Running, ExerciseType::Gym, ExerciseType::BriskWalking],
            exercise_frequency_per_week: 3,
            exercise_intensity_pref: IntensityDistribution {
                low: 0.3,
                moderate: 0.5,
                high: 0.2,
            }
            .normalized(),
        }
    }
}
