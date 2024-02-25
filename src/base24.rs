use anyhow::{Context, Result};
use itertools::{Either, Itertools};
use palette::convert::FromColorUnclamped;
use palette::{Clamp, IntoColor, LinSrgb, Okhsv, Oklab, Oklch, Srgb};
use serde::ser::SerializeMap;
use serde::{Serialize, Serializer};

use std::mem::MaybeUninit;
use std::{array, iter};

#[derive(Serialize, Debug)]
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
        // let okhsv: Okhsv<f64> = Okhsv::from_color_unclamped(*color).clamp();
        // dbg!(okhsv);
        // let oklab: Oklab<f64> = Oklab::from_color_unclamped(okhsv);
        // dbg!(oklab);
        // let rgb: Srgb<u8> = Srgb::from_linear(LinSrgb::from_color_unclamped(oklab));
        let rgb: Srgb<u8> = Srgb::from_linear((*color).into_color());
        let hex = format!("{0:02x}{1:02x}{2:02x}", rgb.red, rgb.green, rgb.blue);

        serializer.serialize_entry(&format_args!("base{idx:02X}"), &hex)?;
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

pub fn generate_palette(mut colors: Vec<Oklch<f64>>, light_mode: bool) -> Result<[Oklch<f64>; 24]> {
    let base_hue = colors.first().expect("at least one color").hue;
    let (base_chroma, highlight_lightness, highlight_chroma) = if light_mode {
        (0.04, 0.4, 0.13)
    } else {
        (0.03, 0.5, 0.12)
    };

    let base_colors = (if light_mode {
        Either::Left((1..=8).rev())
    } else {
        Either::Right(1..=8)
    })
    .map(|l| Oklch::new(l as f64 * 0.1, base_chroma, base_hue));

    let mut i = 0;
    while colors.len() < 8 {
        colors.push(colors[i]);
        i += 1;
    }
    let (highlights, highlights_tee) = colors
        .iter()
        .sorted_unstable_by(|a, b| a.chroma.partial_cmp(&b.chroma).expect("comparable chromas"))
        .rev()
        .take(8)
        .map(|color| color.hue)
        .sorted_unstable_by_key(|hue| hue.into_positive_degrees() as u16)
        .map(|hue| Oklch::new(highlight_lightness, highlight_chroma, hue))
        .tee();

    let base24_bg: [Oklch<f64>; 2] = if light_mode {
        [
            Oklch::new(0.85, base_chroma, base_hue),
            Oklch::new(0.90, base_chroma, base_hue),
        ]
    } else {
        [
            Oklch::new(0.05, base_chroma, base_hue),
            Oklch::new(0.0, base_chroma, base_hue),
        ]
    };

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
