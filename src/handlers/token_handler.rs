use axum::{Json, http::StatusCode, response::IntoResponse};
use crate::models::responses::{SuccessResponse, ErrorResponse};
use crate::models::token_model::{CreateTokenRequest, MintTokenRequest};
use crate::services::token_services::{create_initialize_mint_instruction, create_mint_to_instruction};

pub async fn create_token_instruction(
    Json(payload): Json<CreateTokenRequest>,
) -> impl IntoResponse {
    match create_initialize_mint_instruction(payload) {
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

pub async fn mint_token_instruction(
    Json(payload): Json<MintTokenRequest>,
) -> impl IntoResponse {
    match create_mint_to_instruction(payload) {
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
