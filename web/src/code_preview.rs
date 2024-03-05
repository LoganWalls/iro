use crate::backdrop::Backdrop;
use iro::{lch_to_hex, Base24Style};
use leptos::html::Code;
use leptos::*;
use std::time::Duration;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = Prism, js_name = highlightElement)]
    fn highlight_element(el: &web_sys::HtmlElement);
}

#[component]
pub fn CodePreview(style: Signal<Base24Style>) -> impl IntoView {
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
    let bg_style = Signal::derive(move || {
        let hex = lch_to_hex(&style().palette[0]);
        format!("background-color: #{hex};")
    });

    let on_load = move || {
        let node = code_ref.get().expect("code tag loaded");
        highlight_element(&node);
    };
    set_timeout(on_load, Duration::from_millis(100));

    view! {
        <div class="relative">
            <style type="text/css" media="screen" inner_html=style_content></style>
            <Backdrop style=bg_style/>
            <pre class="relative z-10">
                <code _ref=code_ref class="language-rust">
                    {test_code}
                </code>
            </pre>
        </div>
    }
}
