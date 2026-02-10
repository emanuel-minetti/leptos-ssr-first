use crate::utils::get_lang;
use leptos::html::{div, strong, ElementChild};
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

/// Represents a "Message of the Day" structure containing the message to be rendered.
///
/// The `MessageOfTheDay` struct is designed to hold a format string
/// and a collection of strings to be rendered strongish. The format string is rendered literally
/// except for `{}` substrings which are replaced by members of the collections respecting their
/// order. The number of `{}` substrings and the length of the collection must match to be
/// serialized (e.g., from a file)
///
/// # Attributes
/// - `message` (`String`): The format string for the message.
/// - `strongish` (`Vec<String>`): A collection of the strongish parts of the message.
///
/// # Implements
/// - `Deserialize`: Checks whether matches in format string and length of `strongish` match.
///
/// # Derives
/// - `Serialize`: Enables the struct to be serialized (e.g., to JSON or other formats).
/// - `Default`: Provides a default implementation for the struct, initializing `message` as an
///         empty string and `strongish` as an empty vector.
/// - `Clone`: Allows for creating a duplicate of the struct.
/// - `PartialEq`: Enables comparison of two `MessageOfTheDay` instances for equality.
///
/// # Example
/// ```rust
/// use leptos::ev::message;
/// use serde::Serialize;
///
/// #[derive(Serialize, Default, Clone, PartialEq)]
/// struct MessageOfTheDay {
///     message: String,
///     strongish: Vec<String>,
/// }
///
/// let motd = MessageOfTheDay {
///     message: String::from("{} to our application!"),
///     strongish: vec![String::from("Welcome"),]
/// };
/// ```
/// would render to `<strong>Welcome</strong> to our application!`.
#[derive(Serialize, Default, Clone, PartialEq)]
struct MessageOfTheDay {
    message: String,
    strongish: Vec<String>,
}

impl<'de> Deserialize<'de> for MessageOfTheDay {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[derive(Deserialize)]
        struct MessageOfTheDayHelper {
            message: String,
            strongish: Vec<String>,
        }

        let helper = MessageOfTheDayHelper::deserialize(deserializer)?;
        let placeholder_count = helper.message.matches("{}").count();
        if placeholder_count != helper.strongish.len() {
            return Err(serde::de::Error::custom(format!(
                "Message placeholder count ({}) does not match emphasized array length ({})",
                placeholder_count,
                helper.strongish.len()
            )));
        }

        Ok(MessageOfTheDay {
            message: helper.message,
            strongish: helper.strongish,
        })
    }
}

/// Struct representing the "Message of the Day" configuration to be read from a file on the server.
///
/// This structure defines whether the message of the day feature is enabled,
/// determines its level of importance, and provides localized versions of
/// the message in German (de) and English (en).
///
/// # Fields
///
/// * `enabled` (`bool`) - Indicates whether the "Message of the Day" feature
///   is active. Defaults to `false`.
///
/// * `level` (`MessageOfTheDayLevel`) - Specifies the importance or severity
///   level of the message. This field must be provided explicitly.
///
/// * `de` (`MessageOfTheDay`) - Contains the German localized version
///   of the "Message of the Day".
///
/// * `en` (`MessageOfTheDay`) - Contains the English localized version
///   of the "Message of the Day".
///
/// # Traits
///
/// This struct derives several traits:
/// * `Serialize` and `Deserialize` - Allow the struct to be serialized and
///   deserialized using Serde.
/// * `Default` - Provides a default value for the struct.
/// * `Clone` - Enables cloning of the struct.
/// * `PartialEq` - Allows for equality comparisons between instances.
///
/// # Example
/// See "config/message_of_the_day.json.dist" for an example.
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
    let mut strongish = message.strongish.iter();
    let raw_message_parts = raw_message.as_str().split("{}").collect::<Vec<&str>>();
    let mut message_children_vec: Vec<AnyView> = vec![];
    raw_message_parts.iter().for_each(|part| {
        message_children_vec.push(part.into_any());
        message_children_vec.push(
            strong()
                .child(strongish.next().unwrap_or(&"".to_string()).as_str())
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
