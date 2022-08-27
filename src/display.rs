use embedded_graphics::{
    mono_font::{ascii::FONT_10X20, MonoTextStyleBuilder},
    pixelcolor::BinaryColor,
    prelude::*,
    text::{Baseline, Text},
};

use core::fmt::Write;
use heapless::String;

use micromath::F32Ext;

pub fn render_output<D>(
    d: &mut D,
    last_sample_duration_seconds: Option<f32>,
    avg_sample_duration_seconds: Option<f32>,
) -> Result<(), D::Error>
where
    D: DrawTarget<Color = BinaryColor>,
{
    let avg_samples_per_sec = avg_sample_duration_seconds
        .map(|s| 1f32 / s)
        .unwrap_or(0f32);

    let mut sbuf: String<32> = String::new();

    let text_style = MonoTextStyleBuilder::new()
        .font(&FONT_10X20)
        .text_color(BinaryColor::On)
        .build();

    // 13 '>' chars fit on the screen
    // 1000Hz is about the max for this tube
    let last_samples_per_sec = last_sample_duration_seconds
        .map(|s| 1f32 / s)
        .unwrap_or(0f32);
    let num_arrows = ((last_samples_per_sec.log10() * 4.5).max(0.0).ceil() as u8).min(13);

    for _ in 1..num_arrows {
        sbuf.push('>').unwrap();
    }

    Text::with_baseline(sbuf.as_str(), Point::zero(), text_style, Baseline::Top).draw(d)?;
    sbuf.clear();

    write!(sbuf, "{:6.3}/s", avg_samples_per_sec).unwrap();
    Text::with_baseline(&sbuf, Point::new(0, 16), text_style, Baseline::Top).draw(d)?;
    sbuf.clear();

    Ok(())
}
