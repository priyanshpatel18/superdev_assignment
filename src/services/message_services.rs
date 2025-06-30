use bs58;
use base64;
use solana_sdk::{pubkey::Pubkey, signature::{Keypair, Signature, Signer}};
use crate::models::message_model::{SignMessageRequest, SignMessageResponse, VerifyMessageRequest, VerifyMessageResponse};

pub fn sign_message(req: SignMessageRequest) -> Result<SignMessageResponse, String> {
    // Decode secret
    let secret_bytes = bs58::decode(&req.secret)
        .into_vec()
        .map_err(|_| "Invalid base58 secret")?;

    let keypair = Keypair::from_bytes(&secret_bytes)
        .map_err(|_| "Invalid secret keypair bytes")?;

    let message_bytes = req.message.as_bytes();
    let signature: Signature = keypair.sign_message(message_bytes);

    Ok(SignMessageResponse {
        signature: base64::encode(signature.as_ref()),
        public_key: keypair.pubkey().to_string(),
        message: req.message,
    })
}

pub fn verify_signature(req: VerifyMessageRequest) -> Result<VerifyMessageResponse, String> {
    let pubkey_bytes = bs58::decode(&req.pubkey)
        .into_vec()
        .map_err(|_| "Invalid base58 pubkey")?;
    let pubkey = Pubkey::try_from(pubkey_bytes.as_slice())
        .map_err(|_| "Invalid pubkey format")?;

    let signature_bytes = base64::decode(&req.signature)
        .map_err(|_| "Invalid base64 signature")?;
    let signature = Signature::try_from(signature_bytes.as_slice())
        .map_err(|_| "Invalid signature format")?;

    let valid = signature.verify(pubkey.as_ref(), req.message.as_bytes());

    Ok(VerifyMessageResponse {
        valid,
        message: req.message,
        pubkey: req.pubkey,
    })
}