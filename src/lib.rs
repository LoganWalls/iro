pub mod base24;

use std::ops::Div;
use std::path::PathBuf;

use clap::Parser;
use image::RgbImage;
use itertools::Itertools;
use palette::Oklch;
use palette::{cast::FromComponents, IntoColor, Srgb};

/// Generate color schemes from images
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Path to the image
    pub path: PathBuf,

    /// Generates light color schemes when true
    #[arg(short, long, default_value_t = false)]
    pub light_mode: bool,
}

pub fn parse_colors(image: &mut RgbImage) -> Vec<Oklch<f64>> {
    let segment_size = 16.0;
    let k = 5;

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
                .div(360.0 / segment_size)
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
        .take(k)
        .map(|(count, l, c, h)| {
            let count = *count as f64;
            Oklch::new(l / count, c / count, h / count)
        })
        .collect::<Vec<_>>()
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn it_works() {
//         assert_eq!(1, 1);
//     }
// }
