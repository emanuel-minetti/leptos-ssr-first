use leptos::{component, IntoView};
use leptos::context::use_context;
use leptos::html::{h1, ElementChild};
use leptos::prelude::{Set, WriteSignal};
use crate::i18n::use_i18n;
use crate::layout::breadcrumbs::Breadcrumbs;
use crate::model::route::Routes;

#[component]
pub fn NotFound() -> impl IntoView {
    // set an HTTP status code 404
    // this is feature gated because it can only be done during
    // initial server-side rendering
    // if you navigate to the 404 page subsequently, the status
    // code will not be set because there is not a new HTTP request
    // to the server
    #[cfg(feature = "ssr")]
    {
        use leptos::prelude::*;
        // this can be done inline because it's synchronous
        // if it were async, we'd use a server function
        let resp = expect_context::<leptos_actix::ResponseOptions>();
        resp.set_status(actix_web::http::StatusCode::NOT_FOUND);
    }
    let i18n = use_i18n();
    let route = Routes::get_by_name("not_found");
    let set_crumbs = use_context::<WriteSignal<Breadcrumbs>>().expect("no crumbs specified in context");
    set_crumbs.set(vec![route.clone()]);

    h1().child((route.label)(i18n))
}