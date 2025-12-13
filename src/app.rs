use crate::i18n::{use_i18n, Locale};
use crate::layout::footer::Footer;
use crate::layout::navbar::{NavBar, NavBarProps};
use crate::model::user::User;
use crate::pages::home_page::HomePage;
use crate::pages::imprint::Imprint;
use crate::pages::login::{Login, LoginProps};
use crate::pages::not_found::NotFound;
use crate::pages::privacy::Privacy;
use leptos::html::main;
use leptos::prelude::*;
use leptos::tachys::html::element::{body, head, html};
use leptos::tachys::html::{doctype, InertElement};
use leptos_i18n::context::{init_i18n_context_with_options, I18nContextOptions};
use leptos_i18n::I18nContext;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, StylesheetProps, Title, TitleProps};
use leptos_router::components::{
    ProtectedRoute, ProtectedRouteProps, RouteProps, RouterProps, RoutesProps,
};
use leptos_router::hooks::use_params_map;
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment, WildcardSegment,
};

pub fn shell(options: LeptosOptions) -> impl IntoView {
    View::new((
        doctype("html"),
        html().lang("en").child((
            head().child((
                InertElement::new("<meta charset=\"utf-8\" />"),
                InertElement::new(
                    "<meta name=\"viewport\" content=\"width=device-width, initial-scale=1\" />",
                ),
                AutoReload(AutoReloadProps::builder().options(options.clone()).build()),
                HydrationScripts(
                    HydrationScriptsProps::builder()
                        .options(options.clone())
                        .build(),
                ),
                MetaTags(),
            )),
            body().child(App()),
        )),
    ))
}

#[component]
pub fn App() -> impl IntoView {
    // load, provide and initialize i18n context
    let i18n: I18nContext<Locale, _> = init_i18n_context_with_options(I18nContextOptions {
        enable_cookie: true,
        cookie_name: Default::default(),
        cookie_options: Default::default(),
        ssr_lang_header_getter: Default::default(),
    });
    provide_context(i18n);
    let i18n_signal = use_i18n();
    i18n_signal.set_locale(Locale::en);

    // initializing the global value lang needed by non login pages
    let (lang, set_lang) = signal("en".to_string());

    let browser_lang = move || get_lang_from_browser();

    // getting the lang from locale storage or browser settings
    Effect::new(move || {
        if browser_lang().is_some() {
            let browser_lang = if browser_lang().unwrap() == "en" {
                "en"
            } else {
                "de"
            };
            let i18n = use_i18n();
            if browser_lang == "de" {
                set_lang.set(browser_lang.to_string());
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

    // initializing and providing the user
    let (user, set_user) = signal(None::<User>);
    // let (user, set_user) = signal(Some( User {
    //     name: "Emanuel Minetti".to_string(),
    //     lang: "de".to_string(),
    //     token: "".to_string(),
    //     expires: 0
    // }));
    provide_context(user);

    // the guard for protected routes
    let is_logged_in = move || {
        if user.get().is_some() {
            Some(true)
        } else {
            Some(false)
        }
    };

    // VIEW
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
                                                        .path(StaticSegment("/imprint"))
                                                        .view(Imprint)
                                                        .build(),
                                                )
                                            },
                                            {
                                                Route(
                                                    RouteProps::builder()
                                                        .path(StaticSegment("/privacy"))
                                                        .view(Privacy)
                                                        .build(),
                                                )
                                            },
                                            {
                                                Route(
                                                    RouteProps::builder()
                                                        .path(StaticSegment("/login"))
                                                        .view(move || {
                                                            Login(
                                                                LoginProps::builder()
                                                                    .set_user(set_user)
                                                                    .lang_setter(set_lang)
                                                                    .build(),
                                                            )
                                                        })
                                                        .build(),
                                                )
                                            },
                                            {
                                                ProtectedRoute(
                                                    ProtectedRouteProps::builder()
                                                        .path(StaticSegment(""))
                                                        .view(HomePage)
                                                        .redirect_path(move || "/login?orig_url=/")
                                                        .condition(move || is_logged_in())
                                                        .build(),
                                                )
                                            },
                                            {
                                                ProtectedRoute(
                                                    ProtectedRouteProps::builder()
                                                        .path(WildcardSegment("any"))
                                                        .view(NotFound)
                                                        .redirect_path(move || {
                                                            let params = use_params_map().get();
                                                            let (_, orig_url) =
                                                                params.into_iter().last().unwrap();
                                                            format!("/login?orig_url={}", orig_url)
                                                        })
                                                        .condition(move || is_logged_in())
                                                        .build(),
                                                )
                                            },
                                        )
                                    }))
                                    .build(),
                            ))
                        },
                        { Footer() },
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
