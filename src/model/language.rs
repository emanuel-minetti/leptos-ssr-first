use std::fmt::Display;
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "ssr", derive(sqlx::Type))]
#[cfg_attr(feature = "ssr", sqlx(type_name = "lang", rename_all = "lowercase"))]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum Language {
    #[default]
    En,
    De,
}

impl Display for Language {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match &self {
            Language::En => { "en".to_string() }
            Language::De => { "de".to_string() }
        };
        write!(f, "{}", str)
    }
}