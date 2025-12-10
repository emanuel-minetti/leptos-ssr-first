use crate::i18n::{t, use_i18n};
use crate::model::language::Language;
use crate::model::user::User;
use crate::utils::{set_lang_to_i18n, set_lang_to_locale_storage};
use leptos::ev;
use leptos::html::*;
use leptos::prelude::*;
use leptos::reactive::spawn_local;
use leptos::{component, IntoView};
#[cfg(feature = "ssr")]
use sqlx::query;
#[cfg(feature = "ssr")]
use sqlx::types::Uuid;
#[cfg(feature = "ssr")]
use sqlx::{Pool, Postgres};
use wasm_bindgen::JsCast;
use web_sys::{Event, HtmlSelectElement};

#[component]
pub fn NavBar(lang_setter: WriteSignal<String>) -> impl IntoView {
    let user = use_context::<ReadSignal<Option<User>>>().expect("no user specified in context");
    let lang = use_context::<ReadSignal<String>>().expect("lang missing from context");
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
                            // needed for the select element to be reactive
                            .prop("value", move || lang.get())
                            .on(ev::change, move |ev: Event| {
                                let option_value = ev
                                    .target()
                                    .unwrap()
                                    .value_of()
                                    .unchecked_into::<HtmlSelectElement>()
                                    .value();
                                let option_value = if option_value.to_string() == "en" {
                                    "en"
                                } else {
                                    "de"
                                };
                                lang_setter.set(option_value.to_string());
                                set_lang_to_locale_storage(&option_value);
                                set_lang_to_i18n(&option_value);
                                // set lang to server if applicable
                                if user.get().is_some() {
                                    let lang: Language = lang.get().into();
                                    // using unwrap here because we know user is Some
                                    // TODO adjust!
                                    //let token = user.get().unwrap().token;
                                    let token = "";
                                    spawn_local(async {
                                        set_lang(lang, token.into())
                                            .await
                                            .expect("Got server error setting lang");
                                    });
                                }
                            })
                            .aria_label("Language")
                            .child((
                                {
                                    option()
                                        .value("en")
                                        .selected(move || lang.get() == "en")
                                        .child(t![i18n, english])
                                },
                                {
                                    option()
                                        .value("de")
                                        .selected(move || lang.get() == "de")
                                        .child(t![i18n, german])
                                },
                            )),
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

#[server(client = crate::client::AddAuthHeaderClient)]
pub async fn set_lang(lang: Language, session_id: String) -> Result<(), ServerFnError> {
    use leptos::logging::log;
    use leptos_actix::extract;
    use actix_web::HttpMessage;


    let db_pool = use_context::<Pool<Postgres>>().expect("No db pool?");
    log!("called with lang: {} and session_id: {}", lang, session_id);
    let req: actix_web::HttpRequest = extract().await?;
    // let middle_ware_context = use_context::<String>();
    log!("middleware context: {:?}", req.extensions_mut().get::<String>());
    //set lang in db
    let _ = query!(
        r#"UPDATE account
                SET preferred_language = ($1::text)::lang
                WHERE id = (SELECT account_id FROM session WHERE id = $2)"#,
        lang.to_string(),
        Uuid::parse_str(&session_id).unwrap()
    )
    .execute(&db_pool)
    .await?;

    Ok(())
}
