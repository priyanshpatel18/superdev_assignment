use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct SignMessageRequest {
    #[serde(default)]
    pub message: String,
    #[serde(default)]
    pub secret: String,
}

#[derive(Serialize)]
pub struct SignMessageResponse {
    #[serde(default)]
    pub signature: String,
    #[serde(default)]
    pub public_key: String,
    #[serde(default)]
    pub message: String,
}

#[derive(Deserialize)]
pub struct VerifyMessageRequest {
    #[serde(default)]
    pub message: String,
    #[serde(default)]
    pub signature: String,
    #[serde(default)]
    pub pubkey: String,
}

#[derive(Serialize)]
pub struct VerifyMessageResponse {
    #[serde(default)]
    pub valid: bool,
    #[serde(default)]
    pub message: String,
    #[serde(default)]
    pub pubkey: String,
}
