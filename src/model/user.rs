use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct User {
    pub(crate) name: String,
    pub(crate) preferred_language: String,
}
