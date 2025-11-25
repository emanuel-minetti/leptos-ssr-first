use crate::i18n::*;
use crate::model::user::Language;
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
pub fn Login() -> impl IntoView {
    let i18n = use_i18n();
    let login = ServerAction::<Login>::new();

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
                        )
                    }))
                    .build(),
            )
        }))
}

#[derive(Serialize, Deserialize, Default)]
pub struct LoginServerResponse {
    name: String,
    preferred_language: Language,
    error: String,
}

#[server]
pub async fn login(creds: Credentials) -> Result<LoginServerResponse, ServerFnError> {
    use crate::model::user::Language;
    use sqlx::query;
    use sqlx::{Pool, Postgres};
    use bcrypt::verify;

    let mut name = "".to_string();
    let mut preferred_lang = Language::default();
    let mut error = "".to_string();
    let db_pool = use_context::<Pool<Postgres>>().expect("No db pool?");
    let account_row = query!(
        r#"
                SELECT name, pw_hash, preferred_language as "preferred_language: Language"
                FROM account
                WHERE username = $1
            "#,
        creds.username
    )
        .fetch_optional(&db_pool)
        .await.map_err(|e| ServerFnError::new(e))?;

    match account_row {
        None => {
            error = "Invalid username".to_string();
        },
        Some(row) => {
            if !verify(&creds.password, &row.pw_hash).unwrap() {
                error = "Invalid password".to_string();
            }
            else {
                name = row.name;
                preferred_lang = row.preferred_language;
            }
        },
    }

    Ok(LoginServerResponse {
        name,
        preferred_language: preferred_lang,
        error
    })
}
