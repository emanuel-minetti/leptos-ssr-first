pub struct Route {
    pub path: &'static str,
    pub i18n_key: &'static str,
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
        path: "/imprint",
        i18n_key: "imprint",
    },
    privacy: Route {
        path: "/privacy",
        i18n_key: "privacy",
    },
    home: Route {
        path: "/",
        i18n_key: "homePageTitle",
    },
    login: Route {
        path: "/login",
        i18n_key: "login",
    },
    not_found: Route {
        path: "",  // should not be used
        i18n_key: "not_found",
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