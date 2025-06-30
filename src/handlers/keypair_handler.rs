use axum::{http::StatusCode, response::IntoResponse, Json};

use crate::models::responses::{SuccessResponse, ErrorResponse};
use crate::services::keypair_services::create_keypair;

pub async fn generate_keypair() -> impl IntoResponse {
    match create_keypair() {
        Ok(data) => (
            StatusCode::OK,
            Json(SuccessResponse {
                success: true,
                data,
            }),
        )
            .into_response(),
        Err(msg) => (
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                success: false,
                error: msg,
            }),
        )
            .into_response(),
    }
}
