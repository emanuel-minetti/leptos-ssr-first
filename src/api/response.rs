use serde::{Deserialize, Serialize};
use crate::api::error::ApiError;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ApiResponse<T> {
    pub expires_at: i64,
    pub token: String,
    pub error: Option<ApiError>,
    pub data: T,
}
