use leptos::html::{li, nav, ol};
use leptos::prelude::*;
use crate::model::route::Route;

#[component]
pub fn Breadcrumbs(crumbs: ReadSignal<Vec<Route>>) -> impl IntoView {
    let crumbs_vector = move || {
        let crumbs = crumbs.get();
        crumbs.iter().enumerate().map(|(i, crumb)| {
            let mut li = li().class("breadcrumb-item").child(crumb.i18n_key);
            if i == crumbs.len() - 1 {
                li = li.class("active").aria_current("page");
            }
            li.into_any()
        }).collect::<Vec<_>>()
    };

    nav().aria_label("breadcrumb").child(ol().class("breadcrumb").child(crumbs_vector))
}