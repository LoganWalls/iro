use anyhow::Result;
use clap::Parser;
use iro::base24::{generate_palette, Base24Style, PaletteSettings, PaletteStyle};
use iro::{parse_colors, ParseColorsSettings};

use std::path::PathBuf;

/// Generate color schemes from images
#[derive(Parser, Debug, Clone)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Path to the image
    pub path: PathBuf,

    /// Generates light color schemes when true
    #[arg(short, long, default_value_t = false)]
    pub light: bool,

    /// The number of colors to keep from the image
    #[arg(short, long, default_value_t = 5)]
    pub keep_image_colors: usize,

    /// The size (in degrees) of a color wheel segment that should be treated as a single hue
    #[arg(short, long, default_value_t = 15.0)]
    pub segment_size: f64,
}

impl From<Args> for PaletteSettings {
    fn from(args: Args) -> Self {
        Self {
            style: if args.light {
                PaletteStyle::Light
            } else {
                PaletteStyle::Dark
            },
            keep_image_colors: Some(args.keep_image_colors),
        }
    }
}

impl From<Args> for ParseColorsSettings {
    fn from(args: Args) -> Self {
        Self {
            segment_size: args.segment_size,
        }
    }
}

pub fn main() -> Result<()> {
    let args = Args::try_parse()?;
    let mut img = image::open(&args.path)?.into_rgb8();
    let colors = generate_palette(
        parse_colors(&mut img, &args.clone().into()),
        &args.clone().into(),
    )?;
    let style = Base24Style {
        name: "Bebop".to_string(),
        author: "".to_string(),
        variant: "dark".to_string(),
        palette: colors,
    };
    println!("{}", serde_yaml::to_string(&style)?);
    Ok(())
}
