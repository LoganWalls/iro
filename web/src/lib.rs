use anyhow::Result;
use base64::{engine::general_purpose::STANDARD as BASE64_STANDARD, Engine as _};
use image::imageops::FilterType::Nearest;
use image::io::Reader as ImageReader;
use iro::{generate_palette, lch_to_hex, parse_colors, Base24Style};
use leptos::*;
use std::io::Cursor;

fn style_from_bytes(image_bytes: &[u8]) -> Result<Base24Style> {
    let full_img = ImageReader::new(Cursor::new(image_bytes))
        .with_guessed_format()
        .unwrap()
        .decode()
        .unwrap();
    let mut img = full_img.resize(full_img.width() / 4, full_img.height() / 4, Nearest).to_rgb8();
    let colors = generate_palette(parse_colors(&mut img), false)?;
    Ok(Base24Style {
        name: "Bebop".to_string(),
        author: "".to_string(),
        variant: "dark".to_string(),
        palette: colors,
    })
}

#[component]
pub fn ImagePreview() -> impl IntoView {
    let image_bytes = include_bytes!("../shirasuka-shiomi-slope.png");
    let base64_data = BASE64_STANDARD.encode(image_bytes);

    let b24_style = style_from_bytes(image_bytes).unwrap();
    let palette_color_chips = b24_style
        .palette
        .into_iter()
        .map(|color| {
            let hex = lch_to_hex(&color);
            let style = format!("width: 3em; height: 3em; border-radius: 3em; background-color: #{hex};");
            view! { <div style={style}></div>  }
        })
        .collect::<Vec<_>>();

    view! {
        <div>
            <img style="width: 80em;" src={format!("data:image/png;base64,{base64_data}")} />
            <div style="display: flex; width: 30em; flex-wrap: wrap; gap: 0.5em;">{palette_color_chips}</div>
        </div>
    }
}
