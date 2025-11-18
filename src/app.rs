use crate::i18n::{use_i18n, Locale};
use crate::layout::navbar::{NavBar, NavBarProps};
use crate::pages::home_page::HomePage;
use crate::pages::not_found::NotFound;
use leptos::html::main;
use leptos::prelude::*;
use leptos_i18n::context::{init_i18n_context_with_options, I18nContextOptions};
use leptos_i18n::I18nContext;
use leptos_meta::{provide_meta_context, Stylesheet, StylesheetProps, Title, TitleProps};
use leptos_router::components::{RouteProps, RouterProps, RoutesProps};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment, WildcardSegment,
};

#[component]
pub fn App() -> impl IntoView {
    let i18n: I18nContext<Locale, _> = init_i18n_context_with_options(I18nContextOptions {
        enable_cookie: true,
        cookie_name: Default::default(),
        cookie_options: Default::default(),
        ssr_lang_header_getter: Default::default(),
    });
    provide_context(i18n);
    let i18n_signal = use_i18n();
    i18n_signal.set_locale(Locale::en);

    let (lang, set_lang) = signal("en".to_string());
    Effect::new(move |_| {
        let browser_lang = get_lang_from_browser();
        if browser_lang.is_some() {
            let browser_lang = browser_lang.unwrap();
            let i18n = use_i18n();
            if browser_lang == "de".to_string() {
                set_lang.set(browser_lang);
                i18n.set_locale(Locale::de);
            } else {
                set_lang.set("en".to_string());
                i18n.set_locale(Locale::en);
            }
        }
    });
    provide_context(lang);

    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    View::new((
        Stylesheet(
            StylesheetProps::builder()
                .href("/pkg/leptos-ssr-first.css")
                .id("leptos")
                .build(),
        ),
        Title(TitleProps::builder().text("Welcome to Leptos").build()),
        Router(
            RouterProps::builder()
                .children(ToChildren::to_children(move || {
                    (
                        { NavBar(NavBarProps::builder().lang_setter(set_lang).build()) },
                        {
                            main().child(Routes(
                                RoutesProps::builder()
                                    .fallback(move || "Not Found")
                                    .children(ToChildren::to_children(move || {
                                        (
                                            {
                                                Route(
                                                    RouteProps::builder()
                                                        .path(StaticSegment(""))
                                                        .view(HomePage)
                                                        .build(),
                                                )
                                            },
                                            {
                                                Route(
                                                    RouteProps::builder()
                                                        .path(WildcardSegment("any"))
                                                        .view(NotFound)
                                                        .build(),
                                                )
                                            },
                                        )
                                    }))
                                    .build(),
                            ))
                        },
                    )
                }))
                .build(),
        ),
    ))
}

fn get_lang_from_browser() -> Option<String> {
    let window = web_sys::window().expect("no global `window` exists");
    let navigator_lang = window.navigator().language();
    let local_storage = window.local_storage().expect("no global storage exists");
    let local_storage_lang = local_storage
        .unwrap()
        .get_item("lang")
        .expect("failed to get lang from storage");

    if local_storage_lang.is_none() && navigator_lang.is_some() {
        Some(navigator_lang.unwrap()[0..2].to_string())
    } else {
        local_storage_lang
    }
}
