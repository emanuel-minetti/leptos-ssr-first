use leptos::{component, IntoView};
use leptos::html::{a, div, h1, nav, ElementChild};
use leptos::prelude::ClassAttribute;

#[component]
pub fn NavBar() -> impl IntoView {
    nav()
        .class("navbar navbar-expand-lg bg-body-tertiary")
        .child(
            div().class("container-fluid").child(
                a().class("navbar-brand link-underline link-underline-opacity-0 link-dark")
                    .href("/")
                    .child(h1().child("Leptos SSR First")),
            ),
        )
}