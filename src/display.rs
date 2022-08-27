use embedded_graphics::{
    mono_font::{ascii::FONT_10X20, MonoTextStyleBuilder},
    pixelcolor::BinaryColor,
    prelude::*,
    text::{Baseline, Text},
};

use core::fmt::Write;
use heapless::String;

// tube recharge time
const MIN_PERIOD: f32 = 0.0005;

pub fn render_output<D>(d: &mut D, period_secs: f32) -> Result<(), D::Error>
where
    D: DrawTarget<Color = BinaryColor>,
{
    let samples_per_sec = if period_secs < MIN_PERIOD {
        1.0 / MIN_PERIOD
    } else {
        1.0 / period_secs
    };

    let mut sbuf: String<32> = String::new();

    let text_style = MonoTextStyleBuilder::new()
        .font(&FONT_10X20)
        .text_color(BinaryColor::On)
        .build();

    Text::with_baseline(">>", Point::zero(), text_style, Baseline::Top).draw(d)?;

    write!(sbuf, "{:6.3}", samples_per_sec).unwrap();

    Text::with_baseline(&sbuf, Point::new(0, 16), text_style, Baseline::Top).draw(d)?;

    Ok(())
}
