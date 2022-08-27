use embedded_graphics::{
    mono_font::{ascii::FONT_10X20, MonoTextStyleBuilder},
    pixelcolor::BinaryColor,
    prelude::*,
    text::{Baseline, Text},
};

use core::fmt::Write;
use heapless::String;

pub fn render_output<D>(d: &mut D, sample_duration_seconds: Option<f32>) -> Result<(), D::Error>
where
    D: DrawTarget<Color = BinaryColor>,
{
    let samples_per_sec = sample_duration_seconds.map(|s| 1f32 / s).unwrap_or(0f32);

    let mut sbuf: String<32> = String::new();

    let text_style = MonoTextStyleBuilder::new()
        .font(&FONT_10X20)
        .text_color(BinaryColor::On)
        .build();

    Text::with_baseline("0123456789abc", Point::zero(), text_style, Baseline::Top).draw(d)?;

    write!(sbuf, "{:6.3}", samples_per_sec).unwrap();

    Text::with_baseline(&sbuf, Point::new(0, 16), text_style, Baseline::Top).draw(d)?;

    Ok(())
}
