use leptos::{component, IntoView};
use leptos::context::use_context;
use leptos::html::{div, h1, ElementChild};
use leptos::prelude::{Get, ReadSignal};

#[component]
pub fn HomePage() -> impl IntoView {
    let lang = use_context::<ReadSignal<String>>().expect("no lang specified");
    div().child((
        { h1().child("Hello Emu! Welcome to the Home Page!") },
        { "Preferred Lang: " },
        { move || lang.get().to_string() },
    ))
}