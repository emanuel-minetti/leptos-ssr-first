use crate::api::error::ApiError;
use crate::api::response::ApiResponse;
use crate::i18n::*;
use crate::model::user::User;
use crate::utils::{
    get_lang, set_lang_to_i18n, set_lang_to_locale_storage, set_login_data_to_session_storage,
};
use leptos::form::ActionForm;
use leptos::html::*;
use leptos::prelude::*;
use leptos::reactive::spawn_local;
use leptos::tachys::html::event;
use leptos::{component, server, IntoView};
use leptos_router::hooks::{use_navigate, use_query_map};
use leptos_router::NavigateOptions;
use serde::{Deserialize, Serialize};
use wasm_bindgen::JsCast;
use web_sys::{HtmlFormElement, SubmitEvent};

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
        if !self.length_validated(&self.username, USERNAME_MAX_LENGTH) {
            return Err(LoginCallParamsError::InvalidUsername);
        };
        if !self.length_validated(&self.password, PASSWORD_MAX_LENGTH) {
            return Err(LoginCallParamsError::InvalidPassword);
        };

        Ok(self.clone())
    }

    fn length_validated(&self, input: &String, max_size: u8) -> bool {
        let graphemes_length_as_u8 = self.graphems_length_u8(input.chars().count());

        !(input.is_empty() || graphemes_length_as_u8 > max_size)
    }

    fn graphems_length_u8(&self, usize: usize) -> u8 {
        <usize as TryInto<u8>>::try_into(usize).unwrap_or_else(|_| u8::MAX)
    }
}

#[component]
pub fn Login(
    set_user: WriteSignal<Option<User>>,
    lang_setter: WriteSignal<String>,
) -> impl IntoView {
    let i18n = use_i18n();
    let login = ServerAction::<Login>::new();
    let lang = get_lang();
    let orig_url = use_query_map()
        .get_untracked()
        .get("orig_url")
        .unwrap_or_else(|| "/".to_string());
    let navigate = use_navigate();

    Effect::new(move || {
        if let Some(Ok(response)) = login.value().get() {
            if response.error.is_none() {
                set_login_data_to_session_storage(response.token.as_str(), response.expires_at);
                let navigate = navigate.clone();

                // make sure param orig_url contains no '//' to prevent url injection
                let validated_orig_url = if orig_url.contains("//") {
                    "/".to_string()
                } else {
                    orig_url.clone()
                };

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
        } else {
            let data = data.unwrap().clone();
            let form: HtmlFormElement = ev.target().unwrap().unchecked_into();
            match data.validated() {
                Ok(_) => {}
                Err(error) => match error {
                    LoginCallParamsError::InvalidUsername => {
                        show_error(&ev, &form, 0);
                    }
                    LoginCallParamsError::InvalidPassword => {
                        show_error(&ev, &form, 1);
                    }
                },
            }
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
            .add_any_attr(event::on(
                event::capture(event::submit),
                validated_on_client,
            ))
        }))
}

fn show_error(ev: &SubmitEvent, form: &HtmlFormElement, index: u32) {
    let input_div = form.children().get_with_index(index).unwrap();
    let input = input_div.children().get_with_index(1).unwrap();
    input.class_list().add_1("is-invalid").unwrap();
    ev.prevent_default();
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
        "\
            SELECT pw_hash, id \
            FROM account \
            WHERE username = $1 \
        ",
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
                    "INSERT INTO session (account_id) VALUES ($1) RETURNING id, expires_at",
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
                            expires_at: session_row_record.expires_at.and_utc().timestamp(),
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
        "\
            SELECT name, preferred_language as \"preferred_language: Language\" \
            FROM account \
            WHERE id = $1 \
        ",
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
