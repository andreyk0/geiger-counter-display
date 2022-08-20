use embedded_graphics::{
    mono_font::{ascii::FONT_10X20, MonoTextStyleBuilder},
    pixelcolor::BinaryColor,
    prelude::*,
    text::{Baseline, Text},
};

use core::fmt::Write;
use heapless::String;

pub fn render_output<D>(d: &mut D, v: f32) -> Result<(), D::Error>
where
    D: DrawTarget<Color = BinaryColor>,
{
    let mut sbuf: String<32> = String::new();

    let text_style = MonoTextStyleBuilder::new()
        .font(&FONT_10X20)
        .text_color(BinaryColor::On)
        .build();

    Text::with_baseline("X Duh!", Point::zero(), text_style, Baseline::Top).draw(d)?;

    write!(sbuf, "{:6.1}", v);

    Text::with_baseline(&sbuf, Point::new(0, 16), text_style, Baseline::Top).draw(d)?;

    Ok(())
}
