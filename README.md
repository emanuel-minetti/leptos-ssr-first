<picture>
    <img src="https://raw.githubusercontent.com/leptos-rs/leptos/main/docs/logos/Leptos_logo_RGB.svg" alt="Leptos Logo">
</picture>

# Description

Leptos-SSR-First is A template for a Leptos SSR web app for internal use.
Most of its pages and API calls are authenticated,
the use of HTTP status codes is avoided, and it should be localizable.
Bootstrap is used for styling.

# Installation and Running 

- Install `nodejs`
- Run `npm install`
- Install `rustup`
- Run `rustup target add wasm32-unknown-unknown`
- Install `pkg-config` and `libssl-dev` via apt
- Install `cargo-update` and `cargo-audit` via cargo
- Install `cargo-leptos` via cargo (wait a bit)
- Install `sqlx-cli` and `cargo-make` via cargo
- Install `postgresql` and `postgresql-client` via apt
- Create user and database in Postgresql. (Make sure the app can connect to it.) 
- Run `sqlx database setup`.
- Copy `config/configuration.json.dist` to `config/configuration.json` and adjust accordingly.
- Run `cargo leptos serve` and enjoy.
    