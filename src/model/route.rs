use crate::i18n::Locale;
use leptos::prelude::{AnyView, IntoAny};
use leptos_i18n::{t, I18nContext};

#[derive(Clone)]
pub struct Route {
    pub href: &'static str,
    pub label: fn(I18nContext<Locale>) -> AnyView,
}

pub struct Routes {
    pub imprint: Route,
    pub privacy: Route,
    pub home: Route,
    pub login: Route,
    pub not_found: Route,
}

static ROUTES: Routes = Routes {
    imprint: Route {
        href: "/imprint",
        label: |i18n| t!(i18n, imprint).into_any(),
    },
    privacy: Route {
        href: "/privacy",
        label: |i18n| t!(i18n, privacy).into_any(),
    },
    home: Route {
        href: "/",
        label: |i18n| t!(i18n, homePageTitle).into_any(),
    },
    login: Route {
        href: "/login",
        label: |i18n| t!(i18n, login).into_any(),
    },
    not_found: Route {
        href: "",  // should not be used
        label: |i18n| t!(i18n, notFound).into_any(),
    },
};

impl Routes {
    pub fn get_by_name(key: &str) -> &'static Route {
        match key {
            "imprint" => &ROUTES.imprint,
            "privacy" => &ROUTES.privacy,
            "homePageTitle" => &ROUTES.home,
            "login" => &ROUTES.login,
            "not_found" => &ROUTES.not_found,
            _ => &ROUTES.home,
        }
    }
}