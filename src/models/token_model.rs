use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct CreateTokenRequest {
    #[serde(default)]
    pub mintAuthority: String,
    #[serde(default)]
    pub mint: String,
    #[serde(default)]
    pub decimals: u8,
}

#[derive(Serialize)]
pub struct AccountMetaResponse {
    #[serde(default)]
    pub pubkey: String,
    #[serde(default)]
    pub is_signer: bool,
    #[serde(default)]
    pub is_writable: bool,
}

#[derive(Serialize)]
pub struct CreateTokenResponse {
    #[serde(default)]
    pub program_id: String,
    #[serde(default)]
    pub accounts: Vec<AccountMetaResponse>,
    #[serde(default)]
    pub instruction_data: String,
}

#[derive(Deserialize)]
pub struct MintTokenRequest {
    #[serde(default)]
    pub mint: String,
    #[serde(default)]
    pub destination: String,
    #[serde(default)]
    pub authority: String,
    #[serde(default)]
    pub amount: u64,
}