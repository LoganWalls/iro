use leptos::*;
use std::str::FromStr;

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
        <div class="flex flex-row gap-2">
            <label for=&name_slug>{name}</label>
            <input
                type="range"
                on:change=callback
                name=&name_slug
                min=min.to_string()
                max=max.to_string()
                step=step.to_string()
                value=value_signal.get_untracked().to_string()
            />
            <span>{move || value_signal().to_string()}</span>
        </div>
    }
}
