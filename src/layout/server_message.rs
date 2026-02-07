use leptos::html::{div,          ElementChild};
use leptos::prelude::{Get, IntoAny, Read};
use leptos::{component, server, IntoView};
use leptos::server::OnceResource;
use serde::{Deserialize, Serialize};
use server_fn::ServerFnError;
use crate::utils::get_lang;

#[derive(Serialize, Deserialize, Default, Clone, PartialEq)]
enum MessageOfTheDayLevel {
    #[default]
    Info,
    Warn,
    Error,
}

#[derive(Serialize, Deserialize, Default, Clone, PartialEq)]
struct MessageOfTheDay {
    message: String,
    emphasized: Vec<String>,
}

#[derive(Serialize, Deserialize, Default, Clone, PartialEq)]
pub struct ServerMessageOfTheDay {
    // default is false
    enabled: bool,
    level: MessageOfTheDayLevel,
    de: MessageOfTheDay,
    en: MessageOfTheDay,
}

#[component]
pub fn ServerMessage() -> impl IntoView {
    let message_resource = OnceResource::new(get_message());
    let lang = get_lang();

    div().child(move || match message_resource.get() {
        None => {
            "Loading server message ...".into_any()
        }
        Some(result) => {
            match result {
                Ok(server_message) => {
                    if server_message.enabled {
                        if (move || lang.read() == "de".to_string())() {
                            server_message.de.message.into_any()
                        } else if (move || lang.read() == "en".to_string())() {
                            server_message.en.message.into_any()
                        } else {
                            "".into_any()
                        }
                    } else {
                        "".into_any()
                    }
                }
                Err(e) => {
                    ("Server message error ...".to_string() + e.to_string().as_str()).into_any()
                }
            }
        }
    })
}

#[server]
pub async fn get_message() -> Result<ServerMessageOfTheDay, ServerFnError> {
    use leptos::serde_json;
    use log::{log, Level};
    use std::fs::File;
    use std::io::Read;

    let file_result = File::open("config/message_of_the_day.json");
    let mut file = match file_result {
        Ok(file) => file,
        Err(_) => {
            log!(Level::Warn, "Couldn't open message file");
            return Ok(ServerMessageOfTheDay::default());
        }
    };
    let mut file_string = String::new();
    let _ = file.read_to_string(&mut file_string);
    let message_of_the_day: ServerMessageOfTheDay = serde_json::from_str(&file_string)
        .unwrap_or_else(|_| {
            log!(Level::Warn, "Couldn't parse message file");
            ServerMessageOfTheDay::default()
        });

    Ok(message_of_the_day)
}
