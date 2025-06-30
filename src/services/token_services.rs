use crate::models::token_model::{
    AccountMetaResponse, CreateTokenRequest, CreateTokenResponse, MintTokenRequest,
};
use base64;
use bs58;
use solana_sdk::{instruction::AccountMeta, pubkey::Pubkey};
use spl_token::instruction::{initialize_mint, mint_to};

pub fn create_initialize_mint_instruction(
    input: CreateTokenRequest,
) -> Result<CreateTokenResponse, String> {
    if input.mint.is_empty() || input.mintAuthority.is_empty() {
        return Err("Missing required fields".to_string());
    }

    let mint_pubkey = bs58::decode(&input.mint)
        .into_vec()
        .map_err(|_| "Invalid mint public key")?;
    let mint_pubkey =
        Pubkey::try_from(mint_pubkey.as_slice()).map_err(|_| "Invalid mint public key bytes")?;

    let mint_authority = bs58::decode(&input.mintAuthority)
        .into_vec()
        .map_err(|_| "Invalid mint authority")?;
    let mint_authority =
        Pubkey::try_from(mint_authority.as_slice()).map_err(|_| "Invalid mint authority bytes")?;

    let ix = initialize_mint(
        &spl_token::ID,
        &mint_pubkey,
        &mint_authority,
        None,
        input.decimals,
    )
    .map_err(|e| format!("Failed to build instruction: {}", e))?;

    let accounts = ix
        .accounts
        .into_iter()
        .map(|a| AccountMetaResponse {
            pubkey: a.pubkey.to_string(),
            is_signer: a.is_signer,
            is_writable: a.is_writable,
        })
        .collect();

    Ok(CreateTokenResponse {
        program_id: ix.program_id.to_string(),
        accounts,
        instruction_data: base64::encode(ix.data),
    })
}

pub fn create_mint_to_instruction(input: MintTokenRequest) -> Result<CreateTokenResponse, String> {
    let mint = decode_pubkey(&input.mint)?;
    let destination = decode_pubkey(&input.destination)?;
    let authority = decode_pubkey(&input.authority)?;

    let ix = mint_to(
        &spl_token::ID,
        &mint,
        &destination,
        &authority,
        &[],
        input.amount,
    )
    .map_err(|e| format!("Failed to build instruction: {}", e))?;

    Ok(CreateTokenResponse {
        program_id: ix.program_id.to_string(),
        accounts: convert_accounts(ix.accounts),
        instruction_data: base64::encode(ix.data),
    })
}

fn decode_pubkey(base58_str: &str) -> Result<Pubkey, String> {
    let bytes = bs58::decode(base58_str)
        .into_vec()
        .map_err(|_| "Invalid base58 pubkey")?;
    Pubkey::try_from(bytes.as_slice()).map_err(|_| "Invalid pubkey bytes".to_string())
}

fn convert_accounts(accounts: Vec<AccountMeta>) -> Vec<AccountMetaResponse> {
    accounts
        .into_iter()
        .map(|a| AccountMetaResponse {
            pubkey: a.pubkey.to_string(),
            is_signer: a.is_signer,
            is_writable: a.is_writable,
        })
        .collect()
}
