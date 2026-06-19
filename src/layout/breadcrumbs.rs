use crate::i18n::use_i18n;
use crate::model::route::Route;
use leptos::html::{a, li, nav, ol};
use leptos::prelude::*;
use leptos_sync_ssr::portlet::PortletCtx;
use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub(crate) struct Breadcrumbs{
    pub crumbs: Vec<Route>,
}

impl IntoRender for Breadcrumbs {
    type Output = AnyView;

    fn into_render(self) -> Self::Output {
        let i18n = use_i18n();
        let crumbs = self.crumbs;
        let len = crumbs.len();
        let crumbs_view_vector = move || {
            crumbs
                .iter()
                .enumerate()
                .map(|(i, crumb)| {
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
                })
                .collect::<Vec<_>>()
        };

        nav()
            .aria_label("breadcrumb")
            .class("container-fluid")
            .child(ol().class("breadcrumb").child(crumbs_view_vector))
            .into_any()
    }
}

#[component]
pub fn ShowBreadcrumbs() -> impl IntoView {
    <PortletCtx<Breadcrumbs>>::render()
}

