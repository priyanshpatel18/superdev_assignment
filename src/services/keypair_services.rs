use solana_sdk::{signature::Keypair, signer::Signer};
use bs58;

use crate::models::keypair_model::KeypairData;

pub fn create_keypair() -> Result<KeypairData, String> {
    let keypair = Keypair::new();
    Ok(KeypairData {
        pubkey: keypair.pubkey().to_string(),
        secret: bs58::encode(keypair.to_bytes()).into_string(),
    })
}
