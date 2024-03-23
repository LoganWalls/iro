use crate::backdrop::Backdrop;
use iro::{lch_to_hex, Base24Style};
use leptos::*;
use wasm_bindgen::prelude::*;
use web_sys::HtmlDivElement;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = TS, js_name = setLanguage)]
    async fn set_parser_language(language: String);

    #[wasm_bindgen(js_namespace = TS, js_name = highlight)]
    fn highlight_code(code: &str) -> String;
}

const TEST_CODE: &str = r#"/// Parses a line
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
}"#;

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

    let (code, set_code) = create_signal(TEST_CODE.to_string());
    let code_html = move || match ts_ready() {
        Some(_) => highlight_code(&code()),
        None => code().to_string(),
    };

    view! {
        <div class="relative">
            <style type="text/css" media="screen" inner_html=style_content></style>
            <Backdrop style=bg_style/>
            <div class="code-controls flex flex-row place-content-end px-2 py-1 rounded-t-md relative z-10">
                <label class="flex place-items-center" for="language">
                    <select
                        name="language"
                        class="py-0 px-2 border-none bg-none rounded-md font-sans text-lg text-right"
                        on:change=move |ev| { set_language(event_target_value(&ev)) }
                    >
                        {languages}
                    </select>
                    <svg
                        xmlns="http://www.w3.org/2000/svg"
                        viewBox="0 0 20 20"
                        fill="currentColor"
                        class="inline w-5 h-5"
                    >
                        <path
                            fill-rule="evenodd"
                            d="M4.25 2A2.25 2.25 0 0 0 2 4.25v11.5A2.25 2.25 0 0 0 4.25 18h11.5A2.25 2.25 0 0 0 18 15.75V4.25A2.25 2.25 0 0 0 15.75 2H4.25Zm4.03 6.28a.75.75 0 0 0-1.06-1.06L4.97 9.47a.75.75 0 0 0 0 1.06l2.25 2.25a.75.75 0 0 0 1.06-1.06L6.56 10l1.72-1.72Zm4.5-1.06a.75.75 0 1 0-1.06 1.06L13.44 10l-1.72 1.72a.75.75 0 1 0 1.06 1.06l2.25-2.25a.75.75 0 0 0 0-1.06l-2.25-2.25Z"
                            clip-rule="evenodd"
                        ></path>
                    </svg>
                </label>
            </div>
            <pre class="relative min-w-[45em] max-w-[60em] min-h-40 z-10 px-4 py-2 overflow-x-scroll">
                <div
                    contenteditable="true"
                    spellcheck="false"
                    class="code-input px-4 py-2 absolute size-full top-0 left-0 bg-transparent text-transparent cursor-text whitespace-pre focus:outline-none overflow-visible"
                    on:input=move |ev| {
                        set_code(event_target::<HtmlDivElement>(&ev).inner_text())
                    }
                >

                    {code.get_untracked()}
                </div>
                <code class="language-rust select-none size-full" inner_html=code_html></code>
            </pre>
        </div>
    }
}
