use crate::models::message_model::{SignMessageRequest, VerifyMessageRequest};
use crate::models::responses::{ErrorResponse, SuccessResponse};
use crate::services::message_services::{sign_message as sign_message_service, verify_signature};
use axum::{Json, http::StatusCode, response::IntoResponse};

pub async fn sign_message_handler(Json(payload): Json<SignMessageRequest>) -> impl IntoResponse {
    if payload.message.is_empty() || payload.secret.is_empty() {
        return (
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                success: false,
                error: "Missing required fields".to_string(),
            }),
        )
            .into_response();
    }

    match sign_message_service(payload) {
        Ok(data) => (
            StatusCode::OK,
            Json(SuccessResponse {
                success: true,
                data,
            }),
        )
            .into_response(),
        Err(e) => (
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                success: false,
                error: e,
            }),
        )
            .into_response(),
    }
}

pub async fn verify_message(Json(payload): Json<VerifyMessageRequest>) -> impl IntoResponse {
    match verify_signature(payload) {
        Ok(data) => (
            StatusCode::OK,
            Json(SuccessResponse {
                success: true,
                data,
            }),
        )
            .into_response(),
        Err(e) => (
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                success: false,
                error: e,
            }),
        )
            .into_response(),
    }
}
