use crate::i18n::*;
use crate::model::user::{Language, User};
use leptos::children::ToChildren;
use leptos::form::ActionForm;
use leptos::html::*;
use leptos::prelude::*;
use leptos::{component, server, IntoView};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Credentials {
    username: String,
    password: String,
}

#[component]
pub fn Login(set_user: WriteSignal<Option<User>>) -> impl IntoView {
    let i18n = use_i18n();
    let login = ServerAction::<Login>::new();
    let message = move || match login.value().get() {
        None => {
            if login.pending().get() {
                div()
                    .class("alert alert-info")
                    .child("Waiting for server")
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
                        lang: response.preferred_language.to_string(),
                        token: response.session_id,
                        expires: response.expires_at,
                    }));
                    //TODO redirect to orig url
                    div()
                        .class("alert alert-success")
                        .child("Redirecting ...")
                        .into_any()
                } else {
                    div()
                        .class("alert alert-danger")
                        .child(response.error)
                        .into_any()
                }
            }
            Err(err) => div()
                .class("alert alert-danger")
                .child(
                    "Server Error: "
                        .to_string()
                        .push_str(err.to_string().as_str()),
                )
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
                                        .name("creds[username]")
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
                                            .name("creds[password]")
                                    },
                                ))
                            },
                            {
                                button()
                                    .r#type("submit")
                                    .class("btn btn-primary")
                                    .child(t![i18n, login])
                            },
                            { div().class("mt-2").child( move || message() ) },
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
pub async fn login(creds: Credentials) -> Result<LoginServerResponse, ServerFnError> {
    use crate::model::user::Language;
    use bcrypt::verify;
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
        creds.username
    )
    .fetch_optional(&db_pool)
    .await;

    match account_row_result {
        Ok(account_row) => match account_row {
            None => {
                error = "Invalid username or password".to_string();
            }
            Some(account_row_record) => {
                if !verify(&creds.password, &account_row_record.pw_hash).unwrap() {
                    error = "Invalid username or password".to_string();
                } else {
                    name = account_row_record.name;
                    preferred_lang = account_row_record.preferred_language;
                    let session_row = query!(
                        r#"
                            INSERT INTO session (account_id) VALUES ($1) RETURNING id, expires_at
                            "#,
                        account_row_record.id
                    )
                    .fetch_one(&db_pool)
                    .await;
                    match session_row {
                        Ok(session_row_record) => {
                            expires_at = session_row_record.expires_at.as_utc().unix_timestamp();
                            session_id = session_row_record.id.to_string();
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
