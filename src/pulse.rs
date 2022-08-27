use crate::types::*;

#[derive(Debug, Copy, PartialEq, Clone)]
pub struct PulseSample {
    pub duration_seconds: f32,
    pub ts: SystemInstant, // when the sample was taken, to filter out older ones
}

impl PulseSample {
    pub fn new(duration_seconds: f32, ts: SystemInstant) -> Self {
        PulseSample {
            duration_seconds,
            ts,
        }
    }
}

const BUF_SIZE: usize = 8;

pub struct SampleBuffer {
    samples: [Option<PulseSample>; BUF_SIZE],
    next_idx: usize,
}

impl SampleBuffer {
    pub fn new() -> Self {
        SampleBuffer {
            samples: [None; BUF_SIZE],
            next_idx: 0,
        }
    }

    pub fn add(&mut self, s: PulseSample) {
        self.samples[self.next_idx] = Some(s);
        self.next_idx += 1;
        if self.next_idx >= BUF_SIZE {
            self.next_idx = 0;
        }
    }

    // (last duration secs, avrg duration secs)
    pub fn get(&self, ts_from: SystemInstant) -> (Option<f32>, Option<f32>) {
        let i = if self.next_idx > 0 {
            self.next_idx - 1
        } else {
            BUF_SIZE - 1
        };

        (
            self.samples[i]
                .iter()
                .filter_map(|s| {
                    if s.ts >= ts_from {
                        Some(s.duration_seconds)
                    } else {
                        None
                    }
                })
                .next(),
            self.average_duration_secs_newer_than(ts_from),
        )
    }

    fn average_duration_secs_newer_than(&self, ts_from: SystemInstant) -> Option<f32> {
        let (n, sum) = self
            .samples
            .iter()
            .filter_map(|x| x.filter(|s| s.ts >= ts_from))
            .fold((0usize, 0f32), |(n, s), x| (n + 1, s + x.duration_seconds));

        if n > 0 {
            Some(sum / n as f32)
        } else {
            None
        }
    }
}
