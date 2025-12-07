use std::fmt::Display;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ApiError {
    InvalidCredentials,
    Unauthorized,
    DbError(String),
    DBConnectionError,
    UnexpectedError(String),
    Expired,
}

impl Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            ApiError::InvalidCredentials => "Invalid username or password".to_string(),
            ApiError::Unauthorized => "Unauthorized".to_string(),
            ApiError::DbError(err) => { format!("Database error: {}", err).to_string() },
            ApiError::UnexpectedError(err) => err.to_string(),
            ApiError::Expired => "Session expired".to_string(),
            &ApiError::DBConnectionError => "No DB connection".to_string(),
        };
        write!(f, "{}", str)
    }
}