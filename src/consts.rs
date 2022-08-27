use stm32f1xx_hal::time::*;

pub const SYS_FREQ_HZ: u32 = 72_000_000;
pub const SYS_FREQ: Hertz = Hz(SYS_FREQ_HZ);

// tube recharge time, seconds
pub const PULSE_MIN_PERIOD_SEC: f32 = 0.0005;

// timer ticks per second, depends on the clock configs
pub const TIM_TICKS_PER_SEC: f32 = 9_000_000f32;
