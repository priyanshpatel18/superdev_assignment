use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct SendSolRequest {
    #[serde(default)]
    pub from: String,
    #[serde(default)]
    pub to: String,
    #[serde(default)]
    pub lamports: Option<u64>,
}

#[derive(Deserialize)]
pub struct SendTokenRequest {
    #[serde(default)]
    pub destination: String,
    #[serde(default)]
    pub mint: String,
    #[serde(default)]
    pub owner: String,
    #[serde(default)]
    pub amount: Option<u64>, 
}

#[derive(Serialize)]
pub struct SendSolResponse {
    #[serde(default)]
    pub program_id: String,
    #[serde(default)]
    pub accounts: Vec<String>,
    #[serde(default)]
    pub instruction_data: String,
}

#[derive(Serialize)]
pub struct AccountMetaLite {
    #[serde(default)]
    pub pubkey: String,
    #[serde(default)]
    pub is_signer: bool,
}

#[derive(Serialize)]
pub struct SendTokenResponse {
    #[serde(default)]
    pub program_id: String,
    #[serde(default)]
    pub accounts: Vec<AccountMetaLite>,
    #[serde(default)]
    pub instruction_data: String,
}
