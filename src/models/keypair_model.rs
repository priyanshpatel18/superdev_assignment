use serde::Serialize;

#[derive(Serialize)]
pub struct KeypairData {
    #[serde(default)]
    pub pubkey: String,
    #[serde(default)]
    pub secret: String,
}
