use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::json;

pub struct ApiError(pub slalom_core::error::CoreError);

impl From<slalom_core::error::CoreError> for ApiError {
    fn from(e: slalom_core::error::CoreError) -> Self {
        Self(e)
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        let status = match &self.0 {
            slalom_core::error::CoreError::NotFound(_) => StatusCode::NOT_FOUND,
            slalom_core::error::CoreError::Validation(_) => StatusCode::BAD_REQUEST,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        };
        (status, Json(json!({ "error": self.0.to_string() }))).into_response()
    }
}
