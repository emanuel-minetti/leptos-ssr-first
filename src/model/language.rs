use serde::{Deserialize, Serialize};
use std::fmt::Display;

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
            Language::En => "en".to_string(),
            Language::De => "de".to_string(),
        };
        write!(f, "{}", str)
    }
}

impl Into<&str> for Language {
    fn into(self) -> &'static str {
        match self {
            Language::En => "en",
            Language::De => "de",
        }
    }
}

impl From<&str> for Language {
    fn from(s: &str) -> Self {
        match s {
            "en" => Language::En,
            "de" => Language::De,
            _ => Language::default(),
        }
    }
}

impl From<String> for Language {
    fn from(s: String) -> Self {
        if s == "en" {
            Language::En
        } else if s == "de" {
            Language::De
        } else {
            Language::default()
        }
    }
}
