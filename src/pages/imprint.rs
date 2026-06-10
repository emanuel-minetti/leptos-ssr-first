use leptos::html::{div, h1, ElementChild};
use leptos::prelude::{ClassAttribute, Set, WriteSignal};
use leptos::{component, IntoView};
use leptos::context::use_context;
use crate::i18n::*;
use crate::layout::breadcrumbs::Breadcrumbs;
use crate::model::route::Routes;

#[component]
pub fn Imprint() -> impl IntoView {
    let i18n = use_i18n();
    let route = Routes::get_by_name("imprint");
    let set_crumbs = use_context::<WriteSignal<Breadcrumbs>>().expect("no crumbs specified in context");
    set_crumbs.set(vec![route.clone()]);

    div().class("container").child((
        { h1().child((route.label)(i18n))},
    ))
}