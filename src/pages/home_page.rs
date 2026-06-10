use crate::i18n::use_i18n;
use leptos::context::use_context;
use leptos::html::{div, h1, ElementChild};
use leptos::prelude::{ClassAttribute, Get, ReadSignal, Set, WriteSignal};
use leptos::{component, IntoView};
use leptos_i18n::t;
use crate::layout::breadcrumbs::Breadcrumbs;
use crate::model::route::{Routes};

#[component]
pub fn HomePage() -> impl IntoView {
    let lang = use_context::<ReadSignal<String>>().expect("no lang specified in context");
    let i18n = use_i18n();
    let route = Routes::get_by_name("homePageTitle");
    let set_crumbs = use_context::<WriteSignal<Breadcrumbs>>().expect("no crumbs specified in context");
    set_crumbs.set(vec![route.clone()]);

    div().class("container").child((
        { h1().child((route.label)(i18n)) },
        { t![i18n, preferred] },
        { ": " },
        { move || lang.get().to_string() },
    ))
}
