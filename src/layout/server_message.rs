use leptos::{component, IntoView};
use leptos::html::{div, ElementChild};

#[component]
pub fn ServerMessage() -> impl IntoView {
    div().child("This is the server message")
}