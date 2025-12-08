use crate::i18n::{use_i18n, Locale};

pub fn set_lang_to_locale_storage(lang: &str) {
    let window = web_sys::window().expect("no global `window` exists");
    let local_storage = window
        .local_storage()
        .expect("no global local storage exists")
        .unwrap();
    local_storage.set_item("lang", lang).unwrap();
}

fn set_to_session_storage(key: &str, value: &str) {
    let window = web_sys::window().expect("no global `window` exists");
    let session_storage = window
        .session_storage()
        .expect("no global session storage exists")
        .unwrap();
    session_storage.set_item(key, value).unwrap();
}

pub fn set_login_data_to_session_storage(token: &str, expires: i64) {
    set_to_session_storage("token", token);
    set_to_session_storage("expires", expires.to_string().as_str());
}

pub fn get_login_data_from_session_storage() -> (String, i64) {
    let window = web_sys::window().expect("no global `window` exists");
    let session_storage = window
        .session_storage()
        .expect("no global session storage exists")
        .expect("no session storage exists");
    let token = session_storage.get_item("token").expect("no token in session storage");
    let expires = session_storage.get_item("expires").expect("no expires in session storage");
    (token.unwrap(), expires.unwrap().parse::<i64>().unwrap())
}

pub fn set_lang_to_i18n(lang: &str) {
    let i18n = use_i18n();
    if lang == "de" {
        i18n.set_locale(Locale::de);
    } else {
        i18n.set_locale(Locale::en);
    }
}
