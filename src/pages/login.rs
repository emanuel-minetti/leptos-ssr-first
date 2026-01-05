use crate::api::error::ApiError;
use crate::api::response::ApiResponse;
use crate::i18n::*;
use crate::model::user::User;
use crate::utils::{
    set_lang_to_i18n, set_lang_to_locale_storage, set_login_data_to_session_storage,
};
use leptos::form::ActionForm;
use leptos::html::*;
use leptos::prelude::*;
use leptos::reactive::spawn_local;
use leptos::tachys::html::event;
use leptos::{component, server, IntoView};
use leptos_router::hooks::{use_navigate, use_query_map};
use leptos_router::NavigateOptions;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::sync::OnceLock;
use wasm_bindgen::JsCast;
use web_sys::SubmitEvent;

const USERNAME_MAX_LENGTH: u8 = 20;
const PASSWORD_MAX_LENGTH: u8 = 32;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LoginCallParams {
    username: String,
    password: String,
}

enum LoginCallParamsError {
    InvalidUsername,
    InvalidPassword,
}

impl LoginCallParams {
    fn validated(&self) -> Result<LoginCallParams, LoginCallParamsError> {
        // TODO: Review code duplication
        let username = &self.username;
        let username_graphems_length = username.chars().count();
        if username.is_empty() || <usize as TryInto<u8>>::try_into(username_graphems_length).unwrap_or_else(|_| u8::MAX)
            > USERNAME_MAX_LENGTH
        {
            return Err(LoginCallParamsError::InvalidUsername);
        };
        let password = &self.password;
        let password_graphems_length = password.chars().count();
        if password.is_empty() || <usize as TryInto<u8>>::try_into(password_graphems_length).unwrap_or_else(|_| u8::MAX)
            > PASSWORD_MAX_LENGTH
        {
            return Err(LoginCallParamsError::InvalidPassword);
        };

        Ok(self.clone())
    }
}

const URL_VALIDATION_REGEX: &str = r"^(/[^/].*)$";
static URL_REGEX: OnceLock<Regex> = OnceLock::new();

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
    let navigate = use_navigate();
    let url_validation_matcher =
        URL_REGEX.get_or_init(|| Regex::new(URL_VALIDATION_REGEX).unwrap());

    Effect::new(move || {
        if let Some(Ok(response)) = login.value().get() {
            if response.error.is_none() {
                set_login_data_to_session_storage(response.token.as_str(), response.expires_at);

                let orig_url_clone = orig_url.clone();
                let url_capture = match url_validation_matcher.captures(&orig_url_clone) {
                    None => "/".to_string(),
                    Some(capture) => capture.get(1).unwrap().as_str().to_string(),
                };
                let validated_orig_url = if url_capture.contains("//") {
                    "/".to_string()
                } else {
                    url_capture
                };

                let navigate = navigate.clone();

                spawn_local(async move {
                    if let Ok(res) = get_user().await {
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
                        navigate(
                            &validated_orig_url,
                            NavigateOptions {
                                resolve: false,
                                replace: false,
                                scroll: true,
                                state: Default::default(),
                            },
                        );
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
                    let error_message = match response.error.unwrap() {
                        ApiError::InvalidCredentials => t!(i18n, invalidCredentials).into_any(),
                        _ => t!(i18n, serverError, error = "").into_any(),
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

    let validated_on_client = move |ev: SubmitEvent| {
        let data = Login::from_event(&ev);
        if data.is_err() {
            ev.prevent_default();
        }
        else {
            let data = data.unwrap().clone();
            let form: web_sys::HtmlFormElement = ev.target().unwrap().unchecked_into();
            match data.validated() {
                Ok(_) => {}
                Err(error) => {
                    match error {
                        LoginCallParamsError::InvalidUsername => {
                            let input_div = form.children().get_with_index(0).unwrap();
                            let input = input_div.children().get_with_index(1).unwrap();
                            input.class_list().add_1("is-invalid").unwrap();
                            ev.prevent_default();
                        }
                        LoginCallParamsError::InvalidPassword => {
                            let input_div = form.children().get_with_index(1).unwrap();
                            let input = input_div.children().get_with_index(1).unwrap();
                            input.class_list().add_1("is-invalid").unwrap();
                            ev.prevent_default();
                        }
                    }
                }
            }
            // let username = data.username.clone();
            // let username_graphems_length = username.chars().count();
            // if username.is_empty() || <usize as TryInto<u8>>::try_into(username_graphems_length).unwrap_or_else(|_| u8::MAX)
            //     > USERNAME_MAX_LENGTH {
            //
            //     web_sys::console::log_1(&"Submit prevented".into());
            //     ev.prevent_default();
            // }
            // let password = data.password.clone();
            // let password_graphems_length = password.chars().count();
            // if password.is_empty() || <usize as TryInto<u8>>::try_into(password_graphems_length).unwrap_or_else(|_| u8::MAX)
            //     > PASSWORD_MAX_LENGTH {
            //     web_sys::console::log_1(&"Submit prevented".into());
            //     ev.prevent_default();
            // }

            // if !form.check_validity() {
            //     ev.prevent_default();
            //     ev.stop_propagation()
            // }
            //
            // form.class_list().add_1("was-validated").unwrap();
        }
    };

    div()
        .class("container")
        .child(({ h1().child(t![i18n, login]) }, {
            ActionForm(
                ActionFormProps::builder()
                    .action(login)
                    .children(ToChildren::to_children(move || {
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
                                {
                                    div()
                                        .class("invalid-feedback")
                                        .child(t!(i18n, usernameRequired))
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
                                            .required(true)
                                            .maxlength(PASSWORD_MAX_LENGTH as i64)
                                    },
                                    {
                                        div()
                                            .class("invalid-feedback")
                                            .child(t!(i18n, passwordRequired))
                                    },
                                ))
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
            .attr("novalidate", "true")
                .add_any_attr(event::on(event::capture(event::submit), validated_on_client))
        }))
}

#[server]
pub async fn login(params: LoginCallParams) -> Result<ApiResponse<()>, ServerFnError> {
    use crate::api::error::return_early;
    use crate::api::error::ApiError;
    use crate::api::jwt::{JwtClaim, JwtKeys};
    use actix_web::web::Data;
    use bcrypt::verify;
    use jsonwebtoken::encode;
    use jsonwebtoken::Header;
    use log::{log, Level};
    use sqlx::query;
    use sqlx::{Pool, Postgres};

    let db_pool_result = use_context::<Data<Pool<Postgres>>>();
    let db_pool = match db_pool_result {
        None => {
            log!(Level::Warn, "No database pool found in context");
            return return_early(ApiError::DBConnectionError);
        }
        Some(db_pool) => db_pool,
    };
    let dummy_hash_result = use_context::<Data<String>>();
    let dummy_hash = match dummy_hash_result {
        None => {
            log!(Level::Warn, "No dummy hash found in context");
            return return_early(ApiError::UnexpectedError("Configuration Error".to_string()));
        }
        Some(dummy_hash) => dummy_hash,
    };
    let params = match params.validated() {
        Err(_) => {
            log!(Level::Warn, "Invalid login params");
            return return_early(ApiError::InvalidCredentials);
        }
        Ok(params) => params,
    };
    let account_row_result = query!(
        r#"
            SELECT pw_hash, id
            FROM account
            WHERE username = $1
        "#,
        params.username
    )
    .fetch_optional(&**db_pool)
    .await;

    match account_row_result {
        Ok(account_row) => match account_row {
            None => {
                // hinder timing attacks.
                let _ = verify(&params.password, dummy_hash.as_str()).ok();
                return_early(ApiError::InvalidCredentials)
            }
            Some(account_row_record) => {
                let verify_result = verify(&params.password, &account_row_record.pw_hash);
                let verified = verify_result.unwrap_or_else(|e| {
                    log!(Level::Warn, "Error verifying password: {}", e);
                    false
                });
                if !verified {
                    return return_early(ApiError::InvalidCredentials);
                }
                let session_row = query!(
                    r#"INSERT INTO session (account_id) VALUES ($1) RETURNING id, expires_at"#,
                    account_row_record.id
                )
                .fetch_one(&**db_pool)
                .await;
                match session_row {
                    Ok(session_row_record) => {
                        let jwt_keys =
                            use_context::<Data<JwtKeys>>().expect("No JWT keys from server");
                        let claim = JwtClaim::new(session_row_record.id);
                        let token = encode(&Header::default(), &claim, &jwt_keys.encode_key)
                            .expect("JWT encode failed");
                        log!(Level::Info, "Logged in: {}", params.username);
                        Ok(ApiResponse {
                            error: None,
                            expires_at: session_row_record.expires_at.as_utc().unix_timestamp(),
                            token,
                            data: (),
                        })
                    }
                    Err(err) => return_early(ApiError::DbError(format!(
                        "Error inserting session: {}",
                        err.to_string()
                    ))),
                }
            }
        },
        Err(_) => return_early(ApiError::DBConnectionError),
    }
}

#[server(client = crate::client::AddAuthHeaderClient)]
pub async fn get_user() -> Result<ApiResponse<User>, ServerFnError> {
    use crate::model::language::Language;
    use actix_web::web::Data;
    use actix_web::HttpMessage;
    use leptos_actix::extract;
    use sqlx::query;
    use sqlx::types::Uuid;
    use sqlx::{Pool, Postgres};

    let req: actix_web::HttpRequest = extract().await?;
    let account_id = req.extensions_mut().get::<Uuid>().unwrap().clone();
    let token = req.extensions_mut().get::<String>().unwrap().to_string();
    let expires_at = req.extensions_mut().get::<i64>().unwrap().clone();
    let user_row_result = query!(
        r#"
            SELECT name, preferred_language as "preferred_language: Language"
            FROM account
            WHERE id = $1
        "#,
        account_id
    );
    let user_row = user_row_result
        .fetch_one(&**use_context::<Data<Pool<Postgres>>>().unwrap())
        .await?;

    Ok(ApiResponse {
        expires_at,
        token,
        error: None,
        data: {
            User {
                name: user_row.name,
                preferred_language: user_row.preferred_language.to_string(),
            }
        },
    })
}
