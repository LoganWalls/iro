use leptos::html::Input;
use leptos::*;
use web_sys::js_sys::Uint8Array;

use wasm_bindgen::prelude::*;

#[component]
pub fn ImageUpload(set_bytes: WriteSignal<Box<[u8]>>) -> impl IntoView {
    let input_ref = create_node_ref::<Input>();
    let callback = move |_| {
        if let Some(files) = input_ref.get().and_then(|f: HtmlElement<Input>| f.files()) {
            let file = files.get(0).expect("file to contain one file");
            let reader = web_sys::FileReader::new().unwrap();
            let onload = Closure::wrap(Box::new(move |event: web_sys::Event| {
                let reader = event
                    .target()
                    .unwrap()
                    .dyn_into::<web_sys::FileReader>()
                    .unwrap();
                let result = Uint8Array::new(&reader.result().unwrap()).to_vec();
                set_bytes(result.into());
            }) as Box<dyn FnMut(_)>);
            reader
                .add_event_listener_with_callback("loadend", onload.as_ref().unchecked_ref())
                .unwrap();
            onload.forget();
            reader.read_as_array_buffer(&file).unwrap()
        }
    };

    view! {
        <input on:change=callback type="file" _ref=input_ref accept="image/*" class="hidden"/>
        <button
            title="Upload image"
            on:click=move |_| input_ref.get().expect("file input exists").click()
        >
            <svg
                xmlns="http://www.w3.org/2000/svg"
                fill="none"
                viewBox="0 0 24 24"
                stroke-width="1.5"
                stroke="currentColor"
                class="w-6 h-6"
            >
                <path
                    stroke-linecap="round"
                    stroke-linejoin="round"
                    d="M3 16.5v2.25A2.25 2.25 0 0 0 5.25 21h13.5A2.25 2.25 0 0 0 21 18.75V16.5m-13.5-9L12 3m0 0 4.5 4.5M12 3v13.5"
                ></path>
            </svg>
        </button>
    }
}
