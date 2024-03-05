mod code_preview;
mod copy_button;
mod image_upload;
mod toggle;
mod value_slider;

use crate::code_preview::CodePreview;
use crate::copy_button::CopyButton;
use crate::image_upload::ImageUpload;
use crate::toggle::Toggle;
use crate::value_slider::ValueSlider;
use iro::base24::PaletteStyle;

use anyhow::Result;
use base64::{engine::general_purpose::STANDARD as BASE64_STANDARD, Engine as _};
use image::imageops::FilterType::Nearest;
use image::io::Reader as ImageReader;
use iro::base24::PaletteSettings;
use iro::{generate_palette, lch_to_hex, parse_colors, Base24Style, Oklch, ParseColorsSettings};
use leptos::*;

use std::io::Cursor;

fn style_from_bytes(
    image_bytes: &[u8],
    parse_colors_settings: &ParseColorsSettings,
    palette_settings: &PaletteSettings,
) -> Result<Base24Style> {
    let full_img = ImageReader::new(Cursor::new(image_bytes))
        .with_guessed_format()
        .unwrap()
        .decode()
        .unwrap();
    let mut img = full_img
        .resize(full_img.width() / 4, full_img.height() / 4, Nearest)
        .to_rgb8();
    let colors = generate_palette(
        parse_colors(&mut img, parse_colors_settings),
        palette_settings,
    )?;
    Ok(Base24Style {
        name: "Test Style".to_string(),
        author: "".to_string(),
        variant: "dark".to_string(),
        palette: colors,
    })
}

#[component]
pub fn ColorChip(color: Oklch<f64>) -> impl IntoView {
    let hex = lch_to_hex(&color);
    let style = format!("background-color: #{hex};");
    view! { <div class="rounded-full size-12" style=style></div> }
}

static DEFAULT_IMAGE: &[u8] = include_bytes!("../static/shirasuka-shiomi-slope.png");

#[cfg(web_sys_unstable_apis)]
#[component]
pub fn ImagePreview() -> impl IntoView {
    let (image_bytes, set_image_bytes) = create_signal::<Box<[u8]>>(DEFAULT_IMAGE.into());
    let base64_data = move || BASE64_STANDARD.encode(image_bytes());

    let default_parse_settings = ParseColorsSettings::default();
    let default_settings = PaletteSettings::default();

    let segment_size = create_rw_signal(default_parse_settings.segment_size);
    let dark_mode = create_rw_signal(default_settings.style == PaletteStyle::Dark);
    let base_chroma = create_rw_signal(default_settings.base_chroma);
    let hl_chroma = create_rw_signal(default_settings.hl_chroma);
    let hl_lightness = create_rw_signal(default_settings.hl_lightness);


    let parse_colors_settings = move || ParseColorsSettings {
        segment_size: segment_size.get(),
    };

    let palette_settings = move || PaletteSettings {
        style: match dark_mode() {
            true => PaletteStyle::Dark,
            false => PaletteStyle::Light,
        },
        base_chroma: base_chroma(),
        hl_lightness: hl_lightness(),
        hl_chroma: hl_chroma(),
        ..Default::default()
    };

    let b24_style = Signal::derive(move || {
        style_from_bytes(
            &image_bytes(),
            &parse_colors_settings(),
            &palette_settings(),
        )
        .unwrap()
    });
    let bg_image_style = move || {
        format!(
            "background-image: url(\"data:image/png;base64,{}\");",
            base64_data()
        )
    };
    let bg_style = move || {
        let hex = lch_to_hex(&b24_style().palette[0]);
        format!("{} background-color: #{hex};", bg_image_style())
    };
    let color_chips = move || {
        b24_style
            .get()
            .palette
            .into_iter()
            .map(|color| view! { <ColorChip color=color/> })
            .collect::<Vec<_>>()
    };

    let yaml =
        Signal::derive(move || serde_yaml::to_string(&b24_style()).expect("serializable style"));

    view! {
        <div
            style=bg_style
            class="
            bg-center
            bg-contain
            bg-no-repeat
            size-full
            flex
            flex-row
            items-center
            place-content-center
            gap-2
            p-2
            overflow-auto
            "
        >
            <div class="flex flex-col items-center place-content-center size-full gap-2">
                <div class="flex flex-row gap-2">
                    <div class="flex flex-col gap-2">
                        <Toggle signal=dark_mode true_label="Dark Mode" false_label="Light Mode"/>
                        <ValueSlider
                            name="Segment Size"
                            value_signal=segment_size
                            min=1.0
                            max=180.0
                            step=1.0
                        />
                        <ValueSlider
                            name="Base Chroma"
                            value_signal=base_chroma
                            min=0.0
                            max=0.15
                            step=0.005
                        />
                        <ValueSlider
                            name="Highlight Chroma"
                            value_signal=hl_chroma
                            min=0.0
                            max=0.15
                            step=0.005
                        />
                        <ValueSlider
                            name="Highlight Lightness"
                            value_signal=hl_lightness
                            min=0.0
                            max=1.0
                            step=0.05
                        />
                    </div>
                    <div class="flex flex-row gap-2">
                        <CopyButton title="Copy YAML colorscheme to clipboard" content=yaml/>
                        <ImageUpload set_bytes=set_image_bytes/>
                    </div>
                </div>
                <div class="grid grid-cols-8 grid-rows-3 gap-x-1 gap-y-1">{color_chips}</div>
                <CodePreview style=b24_style/>
            </div>
        </div>
    }
}
