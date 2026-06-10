use crate::i18n::*;
use crate::layout::breadcrumbs::Breadcrumbs;
use crate::model::route::Routes;
use leptos::html::{div, h1, ElementChild};
use leptos::prelude::{expect_context, ClassAttribute};
use leptos::{component, IntoView};
use leptos_sync_ssr::portlet::PortletCtx;

#[component]
pub fn Privacy() -> impl IntoView {
    let i18n = use_i18n();
    let route = Routes::get_by_name("privacy");
    // let set_crumbs =
    //     use_context::<WriteSignal<Breadcrumbs>>().expect("no crumbs specified in context");
    // set_crumbs.set(vec![route.clone()]);
    let breadcrumb_ctx = expect_context::<PortletCtx<Breadcrumbs>>();
    breadcrumb_ctx.set_with(move ||  {
        async move {
            Some(Breadcrumbs {crumbs: vec![route.clone()]})
        }
    });
    div()
        .class("container")
        .child(h1().child((route.label)(i18n)))
}
