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
    #[arg(short, long, default_value_t = 8)]
    pub keep: usize,

    /// How many positions to rotate the highlight colors
    #[arg(short, long, default_value_t = 0)]
    pub rotation: usize,

    /// The size (in degrees) of a color wheel segment that should be treated as a single hue
    #[arg(short, long, default_value_t = 15.0)]
    pub segment_size: f64,

    /// The chroma to use for base colors
    #[arg(short, long)]
    pub base_chroma: Option<f64>,

    /// The lightness to use for highlight colors
    #[arg(short, long)]
    pub hl_lightness: Option<f64>,

    /// The chroma to use for highlight colors
    #[arg(short, long)]
    pub hl_chroma: Option<f64>,
}

impl From<Args> for PaletteSettings {
    fn from(args: Args) -> Self {
        let style = if args.light {
            PaletteStyle::Light
        } else {
            PaletteStyle::Dark
        };
        let defaults = PaletteSettings::default_for(style);
        Self {
            style,
            keep: args.keep,
            rotation: args.rotation,
            base_chroma: args.base_chroma.unwrap_or(defaults.base_chroma),
            hl_chroma: args.hl_chroma.unwrap_or(defaults.hl_chroma),
            hl_lightness: args.hl_lightness.unwrap_or(defaults.hl_lightness),
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
    let settings = args.clone().into();
    let colors = generate_palette(parse_colors(&mut img, &args.clone().into()), &settings)?;
    let style = Base24Style {
        name: "Iro Theme".to_string(),
        author: "You".to_string(),
        variant: settings.style.to_string(),
        palette: colors,
    };
    println!("{}", serde_yaml::to_string(&style)?);
    Ok(())
}
