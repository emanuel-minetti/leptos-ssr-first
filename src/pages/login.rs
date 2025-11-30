use crate::i18n::*;
use crate::model::language::Language;
use crate::model::user::User;
use crate::utils::{set_lang_to_i18n, set_lang_to_locale_storage, set_to_session_storage};
use leptos::children::ToChildren;
use leptos::form::ActionForm;
use leptos::html::*;
use leptos::prelude::*;
use leptos::{component, server, IntoView};
use leptos_router::hooks::use_query_map;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LoginCallParams {
    username: String,
    password: String,
    orig_url: String,
}

#[component]
pub fn Login(
    set_user: WriteSignal<Option<User>>,
    lang_setter: WriteSignal<&'static str>,
) -> impl IntoView {
    let i18n = use_i18n();
    let login = ServerAction::<Login>::new();
    let lang = use_context::<ReadSignal<&str>>().expect("lang missing from context");
    let orig_url = move || {
        use_query_map()
            .get()
            .get("orig_url")
            .expect("orig_url missing from query")
    };
    let message = move || match login.value().get() {
        None => {
            if login.pending().get() {
                div()
                    .class("text-center")
                    .child(
                        div()
                            .class("spinner-border")
                            .role("status")
                            .child(span().class("visually-hidden").child(t!(i18n, loading))),
                    )
                    .into_any()
            } else {
                div().hidden(true).into_any()
            }
        }
        Some(result) => match result {
            Ok(response) => {
                if response.error.is_empty() {
                    set_user.set(Some(User {
                        name: response.name,
                        token: response.session_id.clone(),
                        expires: response.expires_at,
                    }));
                    // set lang (in cookie, local storage, context) if applicable
                    if response.preferred_language.to_string() != lang.get() {
                        let new_lang = response.preferred_language.into();
                        lang_setter.set(new_lang);
                        set_lang_to_locale_storage(new_lang);
                        set_lang_to_i18n(new_lang);
                    }
                    // set expires and token to session storage
                    set_to_session_storage("token", response.session_id.as_str());
                    set_to_session_storage("expires", response.expires_at.to_string().as_str());
                    div()
                        .class("alert alert-success")
                        .child(t!(i18n, redirecting))
                        .into_any()
                } else {
                    let error_message = if response.error == "Invalid username or password" {
                        t!(i18n, invalidCredentials).into_any()
                    } else {
                        t!(i18n, serverError, error = response.error).into_any()
                    };
                    div()
                        .class("alert alert-danger")
                        .child(error_message)
                        .into_any()
                }
            }
            Err(err) => div()
                .class("alert alert-danger")
                .child(t!(i18n, serverError, error = err.to_string()))
                .into_any(),
        },
    };

    div()
        .class("container")
        .child(({ h1().child(t![i18n, login]) }, {
            ActionForm(
                ActionFormProps::builder()
                    .action(login)
                    .children(ToChildren::to_children(move || {
                        let i18n = use_i18n();

                        (
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
                                        .name("params[username]")
                                },
                            )),
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
                                            .name("params[password]")
                                    },
                                ))
                            },
                            {
                                input()
                                    .r#type("hidden")
                                    .name("params[orig_url]")
                                    .value(move || orig_url())
                            },
                            {
                                button()
                                    .r#type("submit")
                                    .class("btn btn-primary")
                                    .child(t![i18n, login])
                            },
                            { div().class("mt-2").child(move || message()) },
                        )
                    }))
                    .build(),
            )
        }))
}

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct LoginServerResponse {
    expires_at: i64,
    session_id: String,
    name: String,
    preferred_language: Language,
    error: String,
}

#[server]
pub async fn login(params: LoginCallParams) -> Result<LoginServerResponse, ServerFnError> {
    use bcrypt::verify;
    use leptos_actix::redirect;
    use sqlx::query;
    use sqlx::{Pool, Postgres};

    let mut name = "".to_string();
    let mut preferred_lang = Language::default();
    let mut error = "".to_string();
    let mut expires_at = 0;
    let mut session_id = "".to_string();
    let db_pool = use_context::<Pool<Postgres>>().expect("No db pool?");
    let account_row_result = query!(
        r#"
                SELECT name, pw_hash, id, preferred_language as "preferred_language: Language"
                FROM account
                WHERE username = $1
            "#,
        params.username
    )
    .fetch_optional(&db_pool)
    .await;

    match account_row_result {
        Ok(account_row) => match account_row {
            None => {
                error = "Invalid username or password".to_string();
            }
            Some(account_row_record) => {
                if !verify(&params.password, &account_row_record.pw_hash).unwrap() {
                    error = "Invalid username or password".to_string();
                } else {
                    name = account_row_record.name;
                    preferred_lang = account_row_record.preferred_language;
                    let session_row = query!(
                        r#"INSERT INTO session (account_id) VALUES ($1) RETURNING id, expires_at"#,
                        account_row_record.id
                    )
                    .fetch_one(&db_pool)
                    .await;
                    match session_row {
                        Ok(session_row_record) => {
                            expires_at = session_row_record.expires_at.as_utc().unix_timestamp();
                            session_id = session_row_record.id.to_string();
                            redirect(params.orig_url.as_str());
                        }
                        Err(_) => {
                            error = "Error creating session".to_string();
                        }
                    }
                }
            }
        },
        Err(_) => {
            error = "No DB connection at 'login'".to_string();
        }
    }

    Ok(LoginServerResponse {
        name,
        preferred_language: preferred_lang,
        error,
        expires_at,
        session_id,
    })
}
