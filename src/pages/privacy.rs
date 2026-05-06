use crate::i18n::*;
use crate::layout::breadcrumbs::Breadcrumbs;
use crate::model::route::Routes;
use leptos::context::use_context;
use leptos::html::{div, h1, ElementChild};
use leptos::prelude::{ClassAttribute, Set, WriteSignal};
use leptos::{component, IntoView};

#[component]
pub fn Privacy() -> impl IntoView {
    let i18n = use_i18n();
    let route = Routes::get_by_name("privacy");
    let set_crumbs =
        use_context::<WriteSignal<Breadcrumbs>>().expect("no crumbs specified in context");
    set_crumbs.set(vec![route.clone()]);

    div()
        .class("container")
        .child(h1().child((route.label)(i18n)))
}
