use crate::i18n::use_i18n;
use leptos::context::use_context;
use leptos::html::{div, h1, ElementChild};
use leptos::prelude::{ClassAttribute, Get, ReadSignal, Set, WriteSignal};
use leptos::{component, IntoView};
use leptos_i18n::t;
use crate::model::route::{Route, Routes};

#[component]
pub fn HomePage() -> impl IntoView {
    let lang = use_context::<ReadSignal<String>>().expect("no lang specified in context");
    let i18n = use_i18n();
    let set_crumbs = use_context::<WriteSignal<Vec<Route>>>().expect("no crumbs specified in context");
    set_crumbs.set(vec![Routes::get_by_name("homePageTitle").clone()]);

    div().class("container").child((
        { h1().child(t![i18n, homePageTitle]) },
        { t![i18n, preferred] },
        { ": " },
        { move || lang.get().to_string() },
    ))
}
