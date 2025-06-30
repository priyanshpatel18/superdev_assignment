use axum::{Router, routing::post};
use tokio::net::TcpListener;
use tracing_subscriber;

mod handlers;
mod models;
mod services;

use crate::handlers::*;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/keypair", post(generate_keypair))
        .route(
            "/token/create",
            post(handlers::token_handler::create_token_instruction),
        )
        .route(
            "/token/mint",
            post(handlers::token_handler::mint_token_instruction),
        )
        .route(
            "/message/sign",
            post(handlers::message_handler::sign_message_handler),
        )
        .route(
            "/message/verify",
            post(handlers::message_handler::verify_message),
        )
        .route("/send/sol", post(handlers::transfer_handler::send_sol))
        .route("/send/token", post(handlers::transfer_handler::send_token));

    println!("Listening on http://localhost:3000");
    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
