use leptos::html::{a, li, nav, ol};
use leptos::prelude::*;
use crate::i18n::use_i18n;
use crate::model::route::Route;

pub type Breadcrumbs = Vec<Route>;

#[component]
pub fn Breadcrumbs(crumbs: ReadSignal<Vec<Route>>) -> impl IntoView {
    let i18n = use_i18n();
    let crumbs_vector = move || {
        let crumbs = crumbs.get();
        let len = crumbs.len();
        crumbs.iter().enumerate().map(|(i, crumb)| {
            let label = (crumb.label)(i18n);
            let is_last = i + 1 == len;
            if is_last {
                li().class("breadcrumb-item active")
                    .attr("aria-current", "page")
                    .child(label)
                    .into_any()
            } else {
                li().class("breadcrumb-item")
                    .child(a().attr("href", crumb.href).child(label))
                    .into_any()
            }
        }).collect::<Vec<_>>()
    };

    nav().aria_label("breadcrumb").child(ol().class("breadcrumb").child(crumbs_vector))
}