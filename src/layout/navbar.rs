use crate::i18n::{t, use_i18n, Locale};
use crate::model::user::User;
use leptos::ev;
use leptos::html::*;
use leptos::prelude::*;
use leptos::{component, IntoView};
use wasm_bindgen::JsCast;
use web_sys::{Event, HtmlSelectElement};

#[component]
pub fn NavBar(lang_setter: WriteSignal<String>) -> impl IntoView {
    let i18n = use_i18n();

    nav().class("navbar bg-body-tertiary").child(
        div()
            .class("container-fluid d-flex justify-content-between")
            .child((
                {
                    a().class("navbar-brand link-underline link-underline-opacity-0 link-dark")
                        .href("/")
                        .child(p().class("fs-3").child("Leptos SSR First"))
                },
                { NavBarLoginInfo() },
                {
                    form().class("d-inline-flex p-2").child(
                        select()
                            .class("form-select")
                            .on(ev::change, move |ev: Event| {
                                let option_value = ev
                                    .target()
                                    .unwrap()
                                    .value_of()
                                    .unchecked_into::<HtmlSelectElement>()
                                    .value();
                                lang_setter.set(option_value.clone());
                                set_lang_to_browser(option_value.clone());
                                set_lang_to_i18n(option_value);
                            })
                            .child(({ option().value("en").child(t![i18n, english]) }, {
                                option().value("de").child(t![i18n, german])
                            })),
                    )
                },
            )),
    )
}

#[component]
fn NavBarLoginInfo() -> impl IntoView {
    let user = use_context::<ReadSignal<Option<User>>>().expect("no user specified in context");
    let i18n = use_i18n();

    div().child(span().class("navbar-text opacity-75").child({
        move || match user.get() {
            None => t![i18n, notLoggedIn].into_any(),
            Some(user) => t![i18n, loggedInAs, name = user.name].into_any(),
        }
    }))
}

fn set_lang_to_i18n(lang: String) {
    let i18n = use_i18n();
    if lang == "de" {
        i18n.set_locale(Locale::de);
    } else {
        i18n.set_locale(Locale::en);
    }
}

fn set_lang_to_browser(lang: String) {
    let window = web_sys::window().expect("no global `window` exists");
    let local_storage = window
        .local_storage()
        .expect("no global storage exists")
        .unwrap();
    local_storage.set_item("lang", &lang).unwrap();
}
