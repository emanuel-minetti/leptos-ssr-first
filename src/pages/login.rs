use crate::api::response::ApiResponse;
use crate::i18n::*;
use crate::model::user::User;
use crate::utils::{
    set_lang_to_i18n, set_lang_to_locale_storage, set_login_data_to_session_storage,
};
use leptos::children::ToChildren;
use leptos::form::ActionForm;
use leptos::html::*;
use leptos::prelude::*;
use leptos::reactive::spawn_local;
use leptos::{component, server, IntoView};
use leptos_router::hooks::use_query_map;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LoginCallParams {
    username: String,
    password: String,
}

#[component]
pub fn Login(
    set_user: WriteSignal<Option<User>>,
    lang_setter: WriteSignal<String>,
) -> impl IntoView {
    let i18n = use_i18n();
    let login = ServerAction::<Login>::new();
    let lang = use_context::<ReadSignal<String>>().expect("lang missing from context");
    let orig_url = use_query_map()
        .get_untracked()
        .get("orig_url")
        .unwrap_or_else(|| "/".to_string());
    let orig_url_clone = orig_url.clone();

    Effect::new(move || {
        let orig_url_clone = orig_url.clone();
        if let Some(Ok(response)) = login.value().get() {
            if response.error.is_none() {
                set_login_data_to_session_storage(response.token.as_str(), response.expires_at);

                spawn_local(async move {
                    if let Ok(res) = get_user(orig_url_clone).await {
                        let server_lang = res.data.preferred_language;
                        // shouldn't rerun on changes of lang
                        let user_lang = lang.get_untracked();
                        if server_lang != user_lang {
                            lang_setter.set(server_lang.clone());
                            set_lang_to_locale_storage(&server_lang);
                            set_lang_to_i18n(&server_lang);
                        }
                        set_user.set(Some(User {
                            name: res.data.name,
                            preferred_language: server_lang,
                        }));
                    }
                })
            }
        }
    });

    let message = move || {
        let value = login.value().get();
        let pending = login.pending().get();

        if pending {
            return div()
                .class("text-center")
                .child(
                    div()
                        .class("spinner-border")
                        .role("status")
                        .child(span().class("visually-hidden").child(t!(i18n, loading))),
                )
                .into_any();
        }

        match value {
            Some(Ok(response)) => {
                if response.error.is_none() {
                    return div()
                        .class("alert alert-success")
                        .child(t!(i18n, redirecting))
                        .into_any();
                } else {
                    let err_str = response.error.unwrap().to_string();
                    let error_message = if err_str == "Invalid username or password" {
                        t!(i18n, invalidCredentials).into_any()
                    } else {
                        t!(i18n, serverError, error = err_str).into_any()
                    };
                    div()
                        .class("alert alert-danger")
                        .child(error_message)
                        .into_any()
                }
            }
            Some(Err(err)) => div()
                .class("alert alert-danger")
                .child(t!(i18n, serverError, error = err.to_string()))
                .into_any(),
            None => div().hidden(true).into_any(),
        }
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
                                            .r#type("password")
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
                                    .value(orig_url_clone.clone())
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

#[server]
pub async fn login(params: LoginCallParams) -> Result<ApiResponse<()>, ServerFnError> {
    use crate::api::error::ApiError;
    use bcrypt::verify;
    use sqlx::query;
    use sqlx::{Pool, Postgres};
    use crate::api::error::return_early;

    let expires_at: i64;
    let session_id: String;
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
                // hinder timing attacks
                let _ = verify(
                    "",
                    "$2a$12$2W3AcX2RnI3ZJSwrvWbar.x6FL.nK63niONl.d.mv39bTG5Ru/E9G",
                )
                .unwrap();
                return return_early(ApiError::InvalidCredentials)
            }
            Some(account_row_record) => {
                if !verify(&params.password, &account_row_record.pw_hash).unwrap() {
                    return return_early(ApiError::InvalidCredentials);
                }
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
                    }
                    Err(err) => {
                        return return_early(ApiError::DbError(format!(
                            "Error inserting session: {}",
                            err.to_string()
                        )))
                    }
                }
            }
        },
        Err(_) => {
            return return_early(ApiError::DBConnectionError);
        }
    }

    Ok(ApiResponse {
        error: None,
        expires_at,
        token: session_id,
        data: (),
    })
}

#[server(client = crate::client::AddAuthHeaderClient)]
pub async fn get_user(orig_url: String) -> Result<ApiResponse<User>, ServerFnError> {
    use crate::model::language::Language;
    use actix_web::HttpMessage;
    use leptos_actix::extract;
    use leptos_actix::redirect;
    use log::{log, Level};
    use sqlx::query;
    use sqlx::types::Uuid;
    use sqlx::{Pool, Postgres};
    use std::str::FromStr;

    let req: actix_web::HttpRequest = extract().await?;
    log!(
        Level::Info,
        "middleware context: {:?}",
        req.extensions_mut().get::<String>().unwrap()
    );
    let session_id = req.extensions_mut().get::<String>().unwrap().to_string();
    let user_row_result = query!(
        r#"
            SELECT name, preferred_language as "preferred_language: Language"
            FROM account a
                JOIN session s ON a.id = s.account_id
            WHERE s.id = $1
        "#,
        Uuid::from_str(&session_id).unwrap()
    );
    let user_row = user_row_result
        .fetch_one(&use_context::<Pool<Postgres>>().unwrap())
        .await?;
    redirect(orig_url.as_str());
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
