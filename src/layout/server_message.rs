use crate::utils::get_lang;
use leptos::html::{div, em, strong, ElementChild};
use leptos::prelude::{AnyView, ClassAttribute, Get, IntoAny, Read};
use leptos::server::OnceResource;
use leptos::{component, server, IntoView};
use serde::{Deserialize, Deserializer, Serialize};
use server_fn::ServerFnError;

#[derive(Serialize, Deserialize, Default, Clone, PartialEq)]
enum MessageOfTheDayLevel {
    #[default]
    Info,
    Warn,
    Error,
}

impl MessageOfTheDayLevel {
    fn to_alert_class(&self) -> String {
        format!(
            "text-center alert alert-{}",
            match self {
                MessageOfTheDayLevel::Info => "info",
                MessageOfTheDayLevel::Warn => "warning",
                MessageOfTheDayLevel::Error => "danger",
            }
        )
    }
}

#[derive(Serialize, Default, Clone, PartialEq)]
struct MessageOfTheDay {
    message: String,
    emphasized: Vec<String>,
}

impl<'de> Deserialize<'de> for MessageOfTheDay {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[derive(Deserialize)]
        struct MessageOfTheDayHelper {
            message: String,
            emphasized: Vec<String>,
        }

        let helper = MessageOfTheDayHelper::deserialize(deserializer)?;
        let placeholder_count = helper.message.matches("{}").count();
        if placeholder_count != helper.emphasized.len() {
            return Err(serde::de::Error::custom(format!(
                "Message placeholder count ({}) does not match emphasized array length ({})",
                placeholder_count,
                helper.emphasized.len()
            )));
        }

        Ok(MessageOfTheDay {
            message: helper.message,
            emphasized: helper.emphasized,
        })
    }
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
        None => "Loading server message ...".into_any(),
        Some(result) => match result {
            Ok(server_message) => {
                // here reactiveness (on reloading) is happening because SSR side lang is ""
                if ["de", "en"].contains(&lang.read().as_str()) {
                    show_message(&server_message).into_any()
                } else {
                    "".into_any()
                }
            }
            Err(e) => ("Server message error: ".to_string() + e.to_string().as_str()).into_any(),
        },
    })
}

fn show_message(message: &ServerMessageOfTheDay) -> impl IntoView {
    let lang = get_lang();
    let localized_message = if (move || lang.read() == "de".to_string())() {
        show_localized_message(message.de.clone())
    } else {
        show_localized_message(message.en.clone())
    };
    let class_string = move || message.level.to_alert_class();

    // if the server message (without the whole page) is reloaded, this won't be reactive
    if !message.enabled {
        div().child("").into_any()
    } else {
        div()
            .class(class_string())
            .child(localized_message)
            .into_any()
    }
}

fn show_localized_message(message: MessageOfTheDay) -> impl IntoView {
    let raw_message = message.message.clone() + "{}";
    let mut emphasized = message.emphasized.iter();
    let raw_message_parts = raw_message.as_str().split("{}").collect::<Vec<&str>>();
    let mut message_children_vec: Vec<AnyView> = vec![];
    raw_message_parts.iter().for_each(|part| {
        message_children_vec.push(part.into_any());
        message_children_vec.push(
            strong()
                .child(emphasized.next().unwrap_or(&"".to_string()).as_str())
                .into_any(),
        );
    });

    div().child(message_children_vec).into_any()
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
        .unwrap_or_else(|e| {
            log!(
                Level::Warn,
                "Couldn't parse message file: {}",
                e.to_string()
            );
            ServerMessageOfTheDay::default()
        });

    Ok(message_of_the_day)
}
