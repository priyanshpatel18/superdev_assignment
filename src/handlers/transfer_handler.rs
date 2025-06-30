use axum::{Json, http::StatusCode, response::IntoResponse};
use crate::models::transfer_model::{SendSolRequest, SendSolResponse, SendTokenRequest};
use crate::models::responses::{SuccessResponse, ErrorResponse};
use crate::services::transfer_services::{create_sol_transfer_instruction, create_token_transfer_instruction};


pub async fn send_sol(
    Json(payload): Json<SendSolRequest>,
) -> impl IntoResponse {
    match create_sol_transfer_instruction(payload) {
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


pub async fn send_token(
    Json(payload): Json<SendTokenRequest>,
) -> impl IntoResponse {
    match create_token_transfer_instruction(payload) {
        Ok(data) => (
            StatusCode::OK,
            Json(SuccessResponse { success: true, data }),
        )
            .into_response(),
        Err(e) => (
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse { success: false, error: e }),
        )
            .into_response(),
    }
}
