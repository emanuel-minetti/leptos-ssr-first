use crate::i18n::*;
use leptos::html::{a, div, footer, li, span, ul, ElementChild};
use leptos::prelude::ClassAttribute;
use leptos::{component, IntoView};

#[component]
pub fn Footer() -> impl IntoView {
    let i18n = use_i18n();

    footer()
        .class("navbar bg-body-tertiary fixed-bottom")
        .child(
            div()
                .class("container-fluid d-flex justify-content-between")
                .child((
                    {
                        ul().class("list-unstyled").child((
                            {
                                li().child(
                                    a().class("link-dark")
                                        .href("/imprint")
                                        .child(t![i18n, imprint]),
                                )
                            },
                            {
                                li().child(
                                    a().class("link-dark")
                                        .href("/privacy")
                                        .child(t![i18n, privacy]),
                                )
                            },
                        ))
                    },
                    { span().class("mb-2").child("© 2025 Emanuel Minetti") },
                )),
        )
}
