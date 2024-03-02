pub mod base24;
pub use base24::{generate_palette, Base24Style};

use image::RgbImage;
use itertools::Itertools;
pub use palette::Oklch;
use palette::{cast::FromComponents, IntoColor, Srgb};
use std::ops::Div;

pub struct ParseColorsSettings {
    pub segment_size: f64,
}

impl Default for ParseColorsSettings {
    fn default() -> Self {
        Self { segment_size: 16.0 }
    }
}

pub fn lch_to_hex(color: &Oklch<f64>) -> String {
    let rgb: Srgb<u8> = Srgb::from_linear((*color).into_color());
    format!("{0:02x}{1:02x}{2:02x}", rgb.red, rgb.green, rgb.blue)
}

pub fn parse_colors(image: &mut RgbImage, settings: &ParseColorsSettings) -> Vec<Oklch<f64>> {
    // Put image into OkLab color space
    let oklab_image = <&[Srgb<u8>]>::from_components(&**image)
        .iter()
        .map(|pixel| pixel.into_linear::<f64>().into_color())
        .collect::<Vec<Oklch<f64>>>();

    // Drop pixels with low chroma
    let avg_chroma =
        oklab_image.iter().map(|pixel| pixel.chroma).sum::<f64>() / oklab_image.len() as f64;

    oklab_image
        .iter()
        .filter(|pixel| pixel.chroma >= avg_chroma)
        .into_grouping_map_by(|pixel| {
            // Map each pixel to a segment of the color wheel base on hue
            pixel
                .hue
                .into_positive_degrees()
                .div(360.0 / settings.segment_size)
                .floor() as u16
        })
        .fold((0, 0.0, 0.0, 0.0), |(count, l, c, h), _, pixel| {
            (
                count + 1,
                l + pixel.l,
                c + pixel.chroma,
                h + pixel.hue.into_positive_degrees(),
            )
        })
        .values()
        .sorted_unstable_by_key(|(count, _, _, _)| count)
        .rev()
        .map(|(count, l, c, h)| {
            let count = *count as f64;
            Oklch::new(l / count, c / count, h / count)
        })
        .collect::<Vec<_>>()
}
