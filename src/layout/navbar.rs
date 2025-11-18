use leptos::html::{a, div, form, nav, option, p, select, ElementChild};
use leptos::prelude::{ClassAttribute, OnAttribute, Set, WriteSignal};
use leptos::{component, IntoView};
use leptos::ev;
use wasm_bindgen::{JsCast};
use web_sys::{Event, HtmlSelectElement};

#[component]
pub fn NavBar(lang_setter: WriteSignal<String>) -> impl IntoView {
    nav().class("navbar bg-body-tertiary").child(
        div()
            .class("container-fluid d-flex justify-content-between")
            .child((
                {
                    a().class("navbar-brand link-underline link-underline-opacity-0 link-dark")
                        .href("/")
                        .child(p().class("fs-3").child("Leptos SSR First"))
                },
                {
                    form().class("d-inline-flex p-2").child(
                        select()
                            .class("form-select")
                            .on(
                                ev::change,
                                move |ev: Event| {
                                    let option_value = ev.target().unwrap().value_of().unchecked_into::<HtmlSelectElement>().value();
                                    lang_setter.set(option_value.clone());
                                    set_lang_to_browser(option_value);
                                }
                            )
                            .child(({ option().value("en").child("English") }, {
                                option().value("de").child("German")
                            })),
                    )
                },
            )),
    )
}

fn set_lang_to_browser(lang: String) {
    let window = web_sys::window().expect("no global `window` exists");
    let local_storage = window.local_storage().expect("no global storage exists").unwrap();
    local_storage.set_item("lang", &lang).unwrap();
}
