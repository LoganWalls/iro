use leptos::*;

#[component]
pub fn Backdrop(style: Signal<String>) -> impl IntoView {
    view! {
        <div
            style=style
            class="absolute top-0 left-0 size-full rounded-lg opacity-90 backdrop-blur-sm"
        ></div>
    }
}
