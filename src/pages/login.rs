use crate::i18n::*;
use leptos::html::*;
use leptos::prelude::{ClassAttribute, GlobalAttributes};
use leptos::{component, IntoView};

#[component]
pub fn Login() -> impl IntoView {
    let i18n = use_i18n();

    div()
        .class("container")
        .child(({ h1().child(t![i18n, login]) }, {
            form().child((
                {
                    div().class("mb-3 col-xs-1 col-xl-2").child((
                        {
                            label()
                                .class("form-label")
                                .r#for("ref1")
                                .child(t![i18n, username])
                        },
                        {
                            input()
                                .r#type("text")
                                .class("form-control")
                                .id("ref1")
                                .name("username")
                        },
                    ))
                },
                {
                    div().class("mb-3 col-xs-1 col-xl-2").child((
                        {
                            label()
                                .class("form-label")
                                .r#for("ref2")
                                .child(t![i18n, password])
                        },
                        {
                            input()
                                .r#type("text")
                                .class("form-control")
                                .id("ref2")
                                .name("password")
                        },
                    ))
                },
                {
                    button().r#type("submit").class("btn btn-primary").child(
                        t![i18n, login]
                    )
                },
            ))
        }))
}
