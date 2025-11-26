use serde::{Deserialize, Serialize};

#[derive(Clone)]
pub struct User {
    pub(crate) name: String,
    pub(crate) lang: String,
    pub(crate) token: String,
    pub(crate) expires: i64,
}

#[cfg_attr(feature = "ssr", derive(sqlx::Type))]
#[cfg_attr(feature = "ssr", sqlx(type_name = "lang", rename_all = "lowercase"))]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum Language {
    #[default]
    En,
    De,
}

impl ToString for Language {
    fn to_string(&self) -> std::string::String {
        match &self {
            Language::En => {"en".to_string()}
            Language::De => {"de".to_string()}
        }
    }
}
