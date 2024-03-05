use anyhow::{Context, Result};
use itertools::{Either, Itertools};
use palette::Oklch;
use serde::ser::SerializeMap;
use serde::{Serialize, Serializer};

use std::mem::MaybeUninit;
use std::{array, iter};

use crate::lch_to_hex;

#[derive(Serialize, Clone, Debug)]
pub struct Base24Style {
    pub name: String,
    pub author: String,
    pub variant: String,
    #[serde(serialize_with = "serialize_colors")]
    pub palette: [Oklch<f64>; 24],
}

fn serialize_colors<S>(colors: &[Oklch<f64>; 24], serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let mut serializer = serializer.serialize_map(Some(24))?;

    for (idx, color) in colors.iter().enumerate() {
        serializer.serialize_entry(&format_args!("base{idx:02X}"), &lch_to_hex(color))?;
    }

    serializer.end()
}

pub fn color_array<const N: usize>(
    iter: impl Iterator<Item = Oklch<f64>>,
) -> Option<[Oklch<f64>; N]> {
    let mut colors = array::from_fn::<_, N, _>(|_| MaybeUninit::uninit());
    let mut count = 0;

    for (src, dst) in iter::zip(iter, &mut colors) {
        dst.write(src);
        count += 1;
    }

    if count < N {
        return None;
    }

    // SAFETY: Because of the length check above we know that every element
    // in `colors` must have been written to by now, so they are all valid.
    Some(colors.map(|color| unsafe { color.assume_init() }))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PaletteStyle {
    Dark,
    Light,
}

impl Default for PaletteStyle {
    fn default() -> Self {
        Self::Dark
    }
}

pub struct PaletteSettings {
    pub style: PaletteStyle,
    pub keep: Option<usize>,
    pub base_chroma: f64,
    pub hl_lightness: f64,
    pub hl_chroma: f64,
}

impl PaletteSettings {
    pub fn default_for(style: PaletteStyle) -> Self {
        match style {
            PaletteStyle::Dark => Self {
                style: PaletteStyle::Dark,
                keep: None,
                base_chroma: 0.03,
                hl_chroma: 0.12,
                hl_lightness: 0.6,
            },
            PaletteStyle::Light => Self {
                style: PaletteStyle::Light,
                keep: None,
                base_chroma: 0.04,
                hl_chroma: 0.14,
                hl_lightness: 0.5,
            },
        }
    }
}

impl Default for PaletteSettings {
    fn default() -> Self {
        Self::default_for(PaletteStyle::Dark)
    }
}

pub fn generate_palette(
    mut colors: Vec<Oklch<f64>>,
    settings: &PaletteSettings,
) -> Result<[Oklch<f64>; 24]> {
    let base_hue = colors.first().expect("at least one color").hue;
    let base_colors_it;
    let base24_bg: [Oklch<f64>; 2];
    match settings.style {
        PaletteStyle::Dark => {
            base_colors_it = Either::Right(1..=8);
            base24_bg = [
                Oklch::new(0.05, settings.base_chroma, base_hue),
                Oklch::new(0.0, settings.base_chroma, base_hue),
            ];
        }
        PaletteStyle::Light => {
            base_colors_it = Either::Left((1..=8).rev());
            base24_bg = [
                Oklch::new(0.85, settings.base_chroma, base_hue),
                Oklch::new(0.90, settings.base_chroma, base_hue),
            ];
        }
    };
    let base_colors =
        base_colors_it.map(|l| Oklch::new(l as f64 * 0.125, settings.base_chroma, base_hue));

    let mut i = 0;
    while colors.len() < 8 {
        colors.push(colors[i]);
        i += 1;
    }
    let (highlights, highlights_tee) = colors
        .iter()
        .take(settings.keep.unwrap_or(colors.len()))
        .sorted_unstable_by(|a, b| a.chroma.partial_cmp(&b.chroma).expect("comparable chromas"))
        .rev()
        .take(8)
        .map(|color| color.hue)
        .sorted_unstable_by_key(|hue| hue.into_positive_degrees() as u16)
        .map(|hue| Oklch::new(settings.hl_lightness, settings.hl_chroma, hue))
        .tee();

    let bright_highlights = highlights_tee.enumerate().filter_map(|(i, color)| {
        match i {
            // Base24 has 2 fewer bright highlight colors compared to highlight colors
            // We exclude base0a (index 2) since it doesn't correspond to a terminal color
            // and base0f (index 7) which is deprecated.
            // See: https://github.com/tinted-theming/base24/blob/18af13d81e31a37be3617891c0a9e7a87da0ade9/styling.md
            2 | 7 => None,
            _ => Some(Oklch::new(color.l * 1.2, color.chroma * 1.2, color.hue)),
        }
    });

    color_array::<24>(
        base_colors
            .chain(highlights)
            .chain(base24_bg)
            .chain(bright_highlights),
    )
    .with_context(|| "Not enough colors")
}
