use serde::{Deserialize, Serialize};

#[derive(Clone)]
pub struct User {
    pub(crate) name: String,
    pub(crate) lang: String,
    pub(crate) token: String,
    pub(crate) expires: u64,
}

#[cfg_attr(feature = "ssr", derive(sqlx::Type))]
#[cfg_attr(feature = "ssr", sqlx(type_name = "lang", rename_all = "lowercase"))]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum Language {
    #[default]
    En,
    De,
}
