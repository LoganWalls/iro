use anyhow::Result;
use base64::{engine::general_purpose::STANDARD as BASE64_STANDARD, Engine as _};
use image::imageops::FilterType::Nearest;
use image::io::Reader as ImageReader;
use iro::{generate_palette, lch_to_hex, parse_colors, Base24Style, Oklch};
use leptos::html::Code;
use leptos::*;

use std::io::Cursor;
use std::time::Duration;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = Prism, js_name = highlightElement)]
    fn highlight_element(el: &web_sys::HtmlElement);
}

fn style_from_bytes(image_bytes: &[u8]) -> Result<Base24Style> {
    let full_img = ImageReader::new(Cursor::new(image_bytes))
        .with_guessed_format()
        .unwrap()
        .decode()
        .unwrap();
    let mut img = full_img
        .resize(full_img.width() / 4, full_img.height() / 4, Nearest)
        .to_rgb8();
    let colors = generate_palette(parse_colors(&mut img), false)?;
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
pub fn CodePreview(style: Base24Style) -> impl IntoView {
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
    let style_tag = || {
        let mut t = css_template.to_string();
        for (idx, color) in style.palette.iter().enumerate() {
            let hex = lch_to_hex(color);
            t = t.replace(&format!("{{{{base{idx:02X}-hex}}}}"), &hex);
        }
        let tag = leptos::html::style().inner_html(t);
        tag.set_type("text/css");
        tag.set_media("screen");
        tag
    };
    let code_ref = create_node_ref::<Code>();
    let on_load = move || {
        let node = code_ref.get().expect("code tag loaded");
        highlight_element(&node);
    };
    set_timeout(on_load, Duration::from_millis(100));

    view! {
        {style_tag()}
        <pre><code _ref={code_ref} class="language-rust">{test_code}</code></pre>
    }
}

#[component]
pub fn ImagePreview() -> impl IntoView {
    let image_bytes = include_bytes!("../static/shirasuka-shiomi-slope.png");
    let base64_data = BASE64_STANDARD.encode(image_bytes);

    let b24_style = style_from_bytes(image_bytes).unwrap();
    let palette_color_chips = b24_style
        .palette
        .into_iter()
        .map(|color| view! { <ColorChip color={color}/> })
        .collect::<Vec<_>>();

    view! {
        <div style="display: flex; flex-direction: row; gap: 2em;">
            <img style="width: 80em;" src={format!("data:image/png;base64,{base64_data}")} />
            <div style="display: flex; width: 60em; flex-wrap: wrap; gap: 0.5em;">
                <div style="display: grid; grid-template-columns: repeat(8, 1fr); grid-template-rows: repeat(3, 1fr); gap: 0.5em 0.5em;">{palette_color_chips}</div>
                <CodePreview style={b24_style} />
            </div>
        </div>
    }
}
