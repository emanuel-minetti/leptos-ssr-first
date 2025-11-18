use crate::i18n::use_i18n;
use leptos::context::use_context;
use leptos::html::{div, h1, ElementChild};
use leptos::prelude::{Get, ReadSignal};
use leptos::{component, IntoView};
use leptos_i18n::t;

#[component]
pub fn HomePage() -> impl IntoView {
    let lang = use_context::<ReadSignal<String>>().expect("no lang specified");
    let i18n = use_i18n();
    div().child((
        { h1().child("Hello Emu! Welcome to the Home Page!") },
        { move || t![i18n, preferred] },
        { ": " },
        { move || lang.get().to_string() },
    ))
}
