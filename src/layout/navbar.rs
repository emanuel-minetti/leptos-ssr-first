use crate::api::response::ApiResponse;
use crate::i18n::{t, use_i18n};
use crate::model::language::Language;
use crate::model::user::User;
use crate::utils::{set_lang_to_i18n, set_lang_to_locale_storage};
use leptos::ev;
use leptos::html::*;
use leptos::prelude::*;
use leptos::reactive::spawn_local;
use leptos::{component, IntoView};
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
                                    spawn_local(async {
                                        set_lang(lang)
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
pub async fn set_lang(lang: Language) -> Result<ApiResponse<User>, ServerFnError> {
    use actix_web::web::Data;
    use actix_web::HttpMessage;
    use leptos::logging::log;
    use leptos_actix::extract;
    use sqlx::query;
    use sqlx::types::Uuid;
    use sqlx::{Pool, Postgres};

    let db_pool = use_context::<Data<Pool<Postgres>>>().expect("No db pool?");
    let req: actix_web::HttpRequest = extract().await?;
    let account_id = req.extensions_mut().get::<Uuid>().unwrap().clone();
    let expires_at = req.extensions_mut().get::<i64>().unwrap().clone();
    let token = req.extensions_mut().get::<String>().unwrap().clone();
    log!("called with lang: {} and account_id: {}", lang, account_id);
    log!(
        "middleware context: {:?}",
        req.extensions_mut().get::<String>()
    );
    //set lang in db
    let account_row = query!(
        r#"
        UPDATE account
            SET preferred_language = ($1::text)::lang
        WHERE id = $2
        RETURNING username, preferred_language as "preferred_language: Language"
        "#,
        lang.to_string(),
        &account_id
    )
    .fetch_one(&**db_pool)
    .await?;

    Ok(ApiResponse {
        expires_at,
        token,
        error: None,
        data: {
            User {
                name: account_row.username,
                preferred_language: account_row.preferred_language.to_string(),
            }
        },
    })
}
