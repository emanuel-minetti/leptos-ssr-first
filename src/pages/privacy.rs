use leptos::html::{div, h1, ElementChild};
use leptos::prelude::{ClassAttribute};
use leptos::{component, IntoView};
use crate::i18n::*;

#[component]
pub fn Privacy() -> impl IntoView {
    let i18n = use_i18n();

    div().class("container").child((
        { h1().child(t![i18n, privacy]) },
    ))
}