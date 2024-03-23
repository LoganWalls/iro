use crate::backdrop::Backdrop;
use iro::{lch_to_hex, Base24Style};
use leptos::leptos_dom::logging::console_log;
use leptos::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = TS, js_name = setLanguage)]
    async fn set_parser_language(language: String);

    #[wasm_bindgen(js_namespace = TS, js_name = highlight)]
    fn highlight_code(code: &str) -> String;
}

#[component]
pub fn CodePreview(style: Signal<Base24Style>) -> impl IntoView {
    let (language, set_language) = create_signal("rust".to_string());
    let languages = include_str!("../treesitter/languages.csv")
        .lines()
        .map(|name| {
            let selected = name == language.get_untracked();
            view! {
                <option selected=selected value=name>
                    {name}
                </option>
            }
        })
        .collect::<Vec<_>>();
    let ts_ready = create_local_resource(language, set_parser_language);

    let test_code = r#"/// Parses a line
fn parse_line(input: &str) -> IResult<&str, Case> {
    let (input, is_negated) = opt(tag("!"))(input)?;
    let is_negated = is_negated.is_some();
    let (input, _) = tag("r ")(input)?;
    let (input, content) = take_till(|c| c == '\n')(input)?;
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
}"#
    .trim();
    let css_template = include_str!("../static/css-style-template.mustache");
    let style_content = move || {
        let mut t = css_template.to_string();
        for (idx, color) in style().palette.iter().enumerate() {
            let hex = lch_to_hex(color);
            t = t.replace(&format!("{{{{base{idx:02X}-hex}}}}"), &hex);
        }
        t
    };
    let bg_style = Signal::derive(move || {
        let hex = lch_to_hex(&style().palette[0]);
        format!("background-color: #{hex};")
    });
    let code_html = move || match ts_ready() {
        Some(_) => highlight_code(test_code),
        None => test_code.to_string(),
    };

    view! {
        <div class="relative">
            <style type="text/css" media="screen" inner_html=style_content></style>
            <Backdrop style=bg_style/>
            <div class="code-controls flex flex-row place-content-between px-2 py-1 rounded-t-md relative z-10">
                <select
                    name="language"
                    class="py-0 border-none bg-none rounded-md font-sans text-lg"
                    on:change=move |ev| { set_language(event_target_value(&ev)) }
                >
                    {languages}
                </select>
                <button>
                    <svg
                        xmlns="http://www.w3.org/2000/svg"
                        viewBox="0 0 20 20"
                        fill="currentColor"
                        class="w-5 h-5"
                    >
                        <path d="m5.433 13.917 1.262-3.155A4 4 0 0 1 7.58 9.42l6.92-6.918a2.121 2.121 0 0 1 3 3l-6.92 6.918c-.383.383-.84.685-1.343.886l-3.154 1.262a.5.5 0 0 1-.65-.65Z"></path>
                        <path d="M3.5 5.75c0-.69.56-1.25 1.25-1.25H10A.75.75 0 0 0 10 3H4.75A2.75 2.75 0 0 0 2 5.75v9.5A2.75 2.75 0 0 0 4.75 18h9.5A2.75 2.75 0 0 0 17 15.25V10a.75.75 0 0 0-1.5 0v5.25c0 .69-.56 1.25-1.25 1.25h-9.5c-.69 0-1.25-.56-1.25-1.25v-9.5Z"></path>
                    </svg>
                </button>
            </div>
            <pre class="relative z-10 px-4 py-2">
                <code class="language-rust px-4" inner_html=code_html></code>
            </pre>
        </div>
    }
}
