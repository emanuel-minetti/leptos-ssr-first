use crate::i18n::{use_i18n, Locale};

pub fn set_lang_to_locale_storage(lang: &str) {
    let window = web_sys::window().expect("no global `window` exists");
    let local_storage = window
        .local_storage()
        .expect("no global storage exists")
        .unwrap();
    local_storage.set_item("lang", lang).unwrap();
}

pub fn set_lang_to_i18n(lang: &str) {
    let i18n = use_i18n();
    if lang == "de" {
        i18n.set_locale(Locale::de);
    } else {
        i18n.set_locale(Locale::en);
    }
}