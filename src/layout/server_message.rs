use leptos::{component, server, IntoView};
use leptos::html::{div, ElementChild};
use serde::{Deserialize, Serialize};
use server_fn::ServerFnError;

#[derive(Serialize, Deserialize, Default)]
enum MessageOfTheDayLevel {
    #[default]
    Info,
    Warn, Error
}

#[derive(Serialize, Deserialize, Default)]
struct MessageOfTheDay {
    message: String,
    emphasized: Vec<String>,
}

#[derive(Serialize, Deserialize, Default)]
pub struct ServerMessageOfTheDay {
    // default is false
    enabled: bool,
    level: MessageOfTheDayLevel,
    de: MessageOfTheDay,
    en: MessageOfTheDay,
}

#[component]
pub fn ServerMessage() -> impl IntoView {
    div().child("This is the server message")
}

#[server(endpoint="get_message")]
pub async fn get_message() -> Result<ServerMessageOfTheDay, ServerFnError> {
    use log::{log, Level};
    use std::fs::File;
    use std::io::Read;
    use leptos::serde_json;

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
