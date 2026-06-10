use crate::i18n::Locale;
use leptos::prelude::{AnyView, IntoAny};
use leptos_i18n::{t, I18nContext};
use serde::de::{self, Deserializer, MapAccess, Visitor};
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Clone, Serialize)]
pub struct Route {
    pub href: &'static str,
    #[serde(skip)]
    pub label: fn(I18nContext<Locale>) -> AnyView,
}

impl PartialEq for Route {
    fn eq(&self, other: &Self) -> bool {
        self.href == other.href
    }
}

// TODO: remove this AI-generated code
impl<'de> Deserialize<'de> for Route {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct RouteVisitor;

        impl<'de> Visitor<'de> for RouteVisitor {
            type Value = Route;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct Route")
            }

            fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut href: Option<String> = None;
                while let Some(key) = map.next_key::<String>()? {
                    match key.as_str() {
                        "href" => href = Some(map.next_value()?),
                        _ => { map.next_value::<de::IgnoredAny>()?; }
                    }
                }
                let href = href.ok_or_else(|| de::Error::missing_field("href"))?;
                Ok(Routes::get_by_name(&href).clone())
            }
        }

        deserializer.deserialize_map(RouteVisitor)
    }
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