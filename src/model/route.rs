//! # Route Handling and Internationalization Module
//!
//! This module provides structures and functions to define and manage application routes.
use crate::i18n::Locale;
use leptos::prelude::{AnyView, IntoAny};
use leptos_i18n::{t, I18nContext};
use serde::de::{self, Deserializer, MapAccess, Visitor};
use serde::{Deserialize, Serialize, Serializer};
use std::fmt;
use serde::ser::SerializeStruct;

/// A structure that represents a route in a web application.
///
/// This `Route` struct is used to define the basic properties of a navigational route,
/// including its href (path) and label generation logic.
///
/// # Fields
///
/// * `href` - A static string slice (`&'static str`) that represents the URL or path
///            of the route. It is expected to be immutable for the lifetime of the application.
///
/// * `label` - A function pointer of type `fn(I18nContext<Locale>) -> AnyView`.
///             This function is responsible for generating the label for the route
///             dynamically, based on the provided `I18nContext` that contains localization
///             and internationalization data.
///
/// # Derives
///
/// * `Clone` - The `Clone` derive macro allows creating a duplicate of the `Route`
///             instance with the same values for its fields.
///
/// # Visibility
///
/// This struct and its fields are scoped as `pub(crate)` to make them accessible
/// within the current Rust crate but not outside of it.
#[derive(Clone, Debug)]
pub(crate) struct Route {
    pub(crate) href: &'static str,
    pub(crate) label: fn(I18nContext<Locale>) -> AnyView,
}

impl PartialEq for Route {
    fn eq(&self, other: &Self) -> bool {
        self.href == other.href
    }
}

impl Serialize for Route {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        if self.href.is_empty() {
            return Err(serde::ser::Error::custom(
                "cannot serialize a Route with an empty href",
            ));
        }
        let mut state = serializer.serialize_struct("Route", 1)?;
        state.serialize_field("href", self.href)?;
        state.end()
    }
}

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
                Routes::get_by_href(&href).ok_or_else(|| de::Error::custom(format!("A route with this href should be present: {}", href))).cloned()
            }
        }

        deserializer.deserialize_map(RouteVisitor)
    }
}

pub(crate) struct Routes {
    pub(crate) imprint: Route,
    pub(crate) privacy: Route,
    pub(crate) home: Route,
    pub(crate) login: Route,
}


/// Static definition of application routes.
///
/// This structure defines a set of commonly used routes in the application,
/// including their paths (`href`) and localized labels that leverage an internationalization
/// (`i18n`) system for translations.
///
/// Each route is an instance of the `Route` struct, which requires:
/// - `href`: The path or URL associated with the route.
/// - `label`: A closure that takes an internationalization context (`i18n`)
///   and resolves the localized label for the route.
///
/// # Fields
///
/// - `imprint`
///   - Path: `/imprint`
///   - Label: Localized version of the "imprint" string.
/// - `privacy`
///   - Path: `/privacy`
///   - Label: Localized version of the "privacy" string.
/// - `home`
///   - Path: `/`
///   - Label: Localized version of the "homePageTitle" string.
/// - `login`
///   - Path: `/login`
///   - Label: Localized version of the "login" string.
/// # Notes
///
/// - The `label` closures convert the localized string into a format that
///   can support any type conversion (`into_any()`). Make sure the result is `sync`!
///
/// # Example
///
/// ```rust
/// let imprint_href = ROUTES.imprint.href; // retrieves "/imprint"
/// let imprint_label = (ROUTES.imprint.label)(i18n); // retrieves localized "imprint" label
/// ```
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
};

impl Routes {
    pub(crate) fn get_by_name(key: &str) -> Option<&'static Route> {
        match key {
            "imprint" => Some(&ROUTES.imprint),
            "privacy" => Some(&ROUTES.privacy),
            "home" => Some(&ROUTES.home),
            "login" => Some(&ROUTES.login),
            _ => None,
        }
    }

    pub(crate) fn get_by_href(href: &str) -> Option<&'static Route> {
        match href {
            "/imprint" => Some(&ROUTES.imprint),
            "/privacy" => Some(&ROUTES.privacy),
            "/" => Some(&ROUTES.home),
            "/login" => Some(&ROUTES.login),
            _ => None,
        }
    }
}