use leptos::{component, server, IntoView};
use leptos::html::{div, ElementChild};
use server_fn::ServerFnError;

#[component]
pub fn ServerMessage() -> impl IntoView {
    div().child("This is the server message")
}

#[server(endpoint="get_message")]
pub async fn get_message() -> Result<String, ServerFnError> {
    Ok("This is the server message".to_string())
}