use leptos::*;

#[component]
pub fn Toggle(
    signal: RwSignal<bool>,
    true_label: &'static str,
    false_label: &'static str,
) -> impl IntoView {
    let label = move || match signal.get() {
        true => true_label,
        false => false_label,
    };
    let callback = move |ev| signal.set(event_target_checked(&ev));
    view! {
        <div class="flex flex-row gap-3 items-center">
            <span>{label}</span>
            <label class="switch">
                <input
                    on:change=callback
                    checked=signal.get_untracked()
                    type="checkbox"
                    class="switch"
                />
                <span class="slider"></span>
            </label>
        </div>
    }
}
