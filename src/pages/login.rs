use std::str::FromStr;
use crate::api::error::ApiError;
use crate::api::response::ApiResponse;
use crate::i18n::*;
use crate::model::language::Language;
use crate::model::user::User;
use crate::utils::{set_login_data_to_session_storage};
use leptos::children::ToChildren;
use leptos::form::ActionForm;
use leptos::html::*;
use leptos::prelude::*;
use leptos::reactive::spawn_local;
use leptos::{component, server, IntoView};
use leptos::leptos_dom::log;
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
                if response.error.is_none() {
                    // TODO get user from server
                    set_login_data_to_session_storage(response.token.as_str(), response.expires_at);

                    spawn_local(async move {
                        let user = match get_user().await {
                            Ok(res) => Some(User {
                                name: res.data.name.to_string().clone(),
                                preferred_language: res.data.preferred_language.to_string().clone(),
                            }),
                            Err(_) => None,
                        };
                        set_user.set(user);
                    });

                    // set_user.set(Some(User {
                    //     name: response.name,
                    //     token: response.session_id.clone(),
                    //     expires: response.expires_at,
                    // }));
                    // set lang (in cookie, local storage, context) if applicable
                    // if response.preferred_language.to_string() != lang.get() {
                    //     let new_lang = response.preferred_language.into();
                    //     lang_setter.set(new_lang);
                    //     set_lang_to_locale_storage(new_lang);
                    //     set_lang_to_i18n(new_lang);
                    // }
                    // set expires and token to session storage
                    // set_to_session_storage("token", response.token.as_str());
                    // set_to_session_storage("expires", response.expires_at.to_string().as_str());
                    div()
                        .class("alert alert-success")
                        .child(t!(i18n, redirecting))
                        .into_any()
                } else {
                    let error_message = if response.error.clone().unwrap().to_string()
                        == "Invalid username or password"
                    {
                        t!(i18n, invalidCredentials).into_any()
                    } else {
                        t!(
                            i18n,
                            serverError,
                            error = response.error.unwrap().to_string()
                        )
                        .into_any()
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

pub type LoginServerResponse = ApiResponse<()>;

#[server]
pub async fn login(params: LoginCallParams) -> Result<LoginServerResponse, ServerFnError> {
    use bcrypt::verify;
    use leptos_actix::redirect;
    use sqlx::query;
    use sqlx::{Pool, Postgres};

    let mut error: Option<ApiError> = None;
    let mut expires_at = 0;
    let mut session_id = "".to_string();
    let db_pool = use_context::<Pool<Postgres>>().expect("No db pool?");

    let account_row_result = query!(
        r#"
                SELECT pw_hash, id
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
                error = Some(ApiError::InvalidCredentials);
            }
            Some(account_row_record) => {
                if !verify(&params.password, &account_row_record.pw_hash).unwrap() {
                    error = Some(ApiError::InvalidCredentials);
                } else {
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
                        Err(err) => {
                            error = Some(ApiError::DbError(err.to_string()));
                        }
                    }
                }
            }
        },
        Err(_) => {
            error = Some(ApiError::DBConnectionError);
        }
    }

    Ok(LoginServerResponse {
        error: None,
        expires_at,
        token: session_id,
        data: (),
    })
}

#[server(client = crate::client::AddAuthHeaderClient)]
pub async fn get_user() -> Result<ApiResponse<User>, ServerFnError> {
    use actix_web::HttpMessage;
    use leptos_actix::extract;
    use sqlx::query;
    use sqlx::types::Uuid;
    use sqlx::{Pool, Postgres};

    let req: actix_web::HttpRequest = extract().await?;
    log!("middleware context: {:?}", req.extensions_mut().get::<String>().unwrap());
    let session_id = req.extensions_mut().get::<String>().unwrap().to_string();
    let user_row_result = query!(
        r#"
            SELECT name, preferred_language as "preferred_language: Language"
            FROM account a
                JOIN session s ON a.id = s.account_id
            WHERE s.id = $1
        "#, Uuid::from_str(&session_id).unwrap());
    let user_row = user_row_result.fetch_one(&use_context::<Pool<Postgres>>().unwrap()).await?;
    Ok(ApiResponse {
        expires_at: 0,
        token: "".to_string(),
        error: None,
        data: {
            User {
                name: user_row.name,
                preferred_language: user_row.preferred_language.to_string(),
            }
        },
    })
}
