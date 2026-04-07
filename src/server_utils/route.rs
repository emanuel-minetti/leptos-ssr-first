use leptos::prelude::{AnyView, IntoAny};
use crate::pages::home_page::HomePage;
use crate::pages::imprint::Imprint;
use crate::pages::privacy::Privacy;

pub struct Route {
    pub path: &'static str,
    pub i18n_key: &'static str,
    pub target: fn() -> AnyView,
}

pub struct Routes {
    pub imprint: Route,
    pub privacy: Route,
    pub home: Route,
}

static ROUTES: Routes = Routes {
    imprint: Route {
        path: "/imprint",
        i18n_key: "imprint",
        target: || Imprint().into_any(),
    },
    privacy: Route {
        path: "/privacy",
        i18n_key: "privacy",
        target: || Privacy().into_any(),
    },
    home: Route {
        path: "/",
        i18n_key: "homePageTitle",
        target: || HomePage.into_any(),
    },
};

impl Routes {
    pub fn get_by_key(key: &str) -> &'static Route {
        match key {
            "imprint" => &ROUTES.imprint,
            "privacy" => &ROUTES.privacy,
            "homePageTitle" => &ROUTES.home,
            _ => &ROUTES.home,
        }
    }
}