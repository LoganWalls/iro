use anyhow::Result;
use base64::{engine::general_purpose::STANDARD as BASE64_STANDARD, Engine as _};
use image::imageops::FilterType::Nearest;
use image::io::Reader as ImageReader;
use iro::base24::PaletteSettings;
use iro::{generate_palette, lch_to_hex, parse_colors, Base24Style, Oklch, ParseColorsSettings};
use leptos::html::Code;
use leptos::*;

use std::io::Cursor;
use std::str::FromStr;
use std::time::Duration;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = Prism, js_name = highlightElement)]
    fn highlight_element(el: &web_sys::HtmlElement);
}

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
    let style = format!("width: 3em; height: 3em; border-radius: 3em; background-color: #{hex};");
    view! { <div style={style}></div>  }
}

#[component]
pub fn CodePreview<F: Fn() -> Base24Style + 'static>(style: F) -> impl IntoView {
    let test_code = r#"
fn parse_line(input: &str) -> IResult<&str, Case> {
    let (input, is_negated) = opt(tag("!"))(input)?;
    let is_negated = is_negated.is_some();
    let (input, _) = tag("r ")(input)?;
    let (input, content) = take_till(|c| c == '\\n')(input)?;
    let regex = Regex::new(content);
    Ok((
        input,
        Case {
            regex: regex.map_err(|_| {
                println!("regex {} failed to compile", content);
                nom::Err::Failure(
                  ParseError::from_error_kind(
                    input,
                    ErrorKind::Fail
                  )
                )
            })?,
            negated: is_negated,
        },
    ))
}"#;
    let css_template = include_str!("../static/css-style-template.mustache");
    let style_content = move || {
        let mut t = css_template.to_string();
        for (idx, color) in style().palette.iter().enumerate() {
            let hex = lch_to_hex(color);
            t = t.replace(&format!("{{{{base{idx:02X}-hex}}}}"), &hex);
        }
        t
    };
    let code_ref = create_node_ref::<Code>();
    let on_load = move || {
        let node = code_ref.get().expect("code tag loaded");
        highlight_element(&node);
    };
    set_timeout(on_load, Duration::from_millis(100));

    view! {
        <style type="text/css" media="screen" inner_html={style_content}></style>
        <pre><code _ref={code_ref} class="language-rust">{test_code}</code></pre>
    }
}

#[component]
pub fn ValueSlider<T>(
    name: &'static str,
    value_signal: RwSignal<T>,
    min: T,
    max: T,
    step: T,
) -> impl IntoView
where
    T: 'static + FromStr + ToString + Clone,
    <T as FromStr>::Err: std::fmt::Debug,
{
    let name_slug = name.to_lowercase().replace(' ', "-");
    let callback = move |ev| {
        value_signal.set(
            event_target_value(&ev)
                .parse()
                .expect("value to be valid number"),
        );
    };
    view! {
        <div>
        <label for={&name_slug}>{name}</label>
        <input type="range" on:change=callback name={&name_slug} min={min.to_string()} max={max.to_string()} step={step.to_string()} value={value_signal.get_untracked().to_string()} />
        <span>{move || value_signal().to_string()}</span>
        </div>
    }
}

#[component]
pub fn ImagePreview() -> impl IntoView {
    let image_bytes = include_bytes!("../static/shirasuka-shiomi-slope.png");
    let base64_data = BASE64_STANDARD.encode(image_bytes);
    let segment_size = create_rw_signal(16.0);
    // create_effect(move |_| console_log(&segment_size().to_string()));

    let parse_colors_settings = move || ParseColorsSettings {
        segment_size: segment_size.get(),
    };

    let palette_settings = move || PaletteSettings {
        ..Default::default()
    };

    let b24_style = move || {
        style_from_bytes(image_bytes, &parse_colors_settings(), &palette_settings()).unwrap()
    };
    let palette_color_chips = move || {
        b24_style()
            .palette
            .into_iter()
            .map(|color| view! { <ColorChip color={color}/> })
            .collect::<Vec<_>>()
    };

    view! {
        <div style="display: flex; flex-direction: row; gap: 2em;">
            <img style="width: 80em;" src={format!("data:image/png;base64,{base64_data}")} />
            <div style="display: flex; width: 60em; flex-wrap: wrap; gap: 0.5em;">
                <div>
                    <ValueSlider name="Segment Size" value_signal={segment_size} min=1.0 max=180.0 step=1.0 />
                </div>
                <div style="display: grid; grid-template-columns: repeat(8, 1fr); grid-template-rows: repeat(3, 1fr); gap: 0.5em 0.5em;">{palette_color_chips}</div>
                <CodePreview style={b24_style} />
            </div>
        </div>
    }
}
