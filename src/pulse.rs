#[derive(Debug, Copy, PartialEq, Clone)]
pub struct PulseSample {
    pub duration_seconds: f32,
}

impl PulseSample {
    pub fn new(duration_seconds: f32) -> Self {
        PulseSample { duration_seconds }
    }
}
