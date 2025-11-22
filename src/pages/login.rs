use crate::i18n::*;
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

#[server(endpoint = "login-test")]
pub async fn login(creds: Credentials) -> Result<String, ServerFnError> {
    use sqlx::query;
    use sqlx::{Pool, Postgres};

    let db_pool = use_context::<Pool<Postgres>>().expect("No db pool?");
    let account_row = query!(
        r#"
            SELECT name
            FROM account
            WHERE username = $1
        "#,
        creds.username
    )
    .fetch_optional(&db_pool)
    .await?;

    match account_row {
        Some(account_row) => Ok(account_row.name),
        None => Ok("nicht gefunden".to_string()),
    }
}
