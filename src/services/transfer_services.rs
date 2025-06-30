use solana_sdk::{
    instruction::Instruction,
    pubkey::Pubkey, system_instruction,
};
use base64;
use spl_token::instruction::transfer;
use crate::models::transfer_model::{AccountMetaLite, SendSolRequest, SendSolResponse, SendTokenRequest, SendTokenResponse};

pub fn create_sol_transfer_instruction(req: SendSolRequest) -> Result<SendSolResponse, String> {
    use std::str::FromStr;

    let lamports = req.lamports.ok_or("Lamports must be provided")?;
    if lamports == 0 {
        return Err("Lamports must be greater than 0".to_string());
    }

    let from_pubkey = Pubkey::from_str(&req.from).map_err(|_| "Invalid from address")?;
    let to_pubkey = Pubkey::from_str(&req.to).map_err(|_| "Invalid to address")?;

    let ix: Instruction = system_instruction::transfer(&from_pubkey, &to_pubkey, lamports);

    let instruction_data = base64::encode(&ix.data);

    Ok(SendSolResponse {
        program_id: ix.program_id.to_string(),
        accounts: ix.accounts.iter().map(|meta| meta.pubkey.to_string()).collect(),
        instruction_data,
    })
}

pub fn create_token_transfer_instruction(
    req: SendTokenRequest,
) -> Result<SendTokenResponse, String> {
    use std::str::FromStr;

    let amount = req.amount.ok_or("Amount must be provided")?;
    if amount == 0 {
        return Err("Amount must be greater than 0".to_string());
    }

    let owner = Pubkey::from_str(&req.owner).map_err(|_| "Invalid owner address")?;
    let destination = Pubkey::from_str(&req.destination).map_err(|_| "Invalid destination")?;
    let mint = Pubkey::from_str(&req.mint).map_err(|_| "Invalid mint address")?;

    let source_ata = spl_associated_token_account::get_associated_token_address(&owner, &mint);
    let dest_ata = spl_associated_token_account::get_associated_token_address(&destination, &mint);

    let ix = transfer(
        &spl_token::ID,
        &source_ata,
        &dest_ata,
        &owner,
        &[],
        amount,
    ).map_err(|e| format!("Failed to create instruction: {e}"))?;

    let instruction_data = base64::encode(&ix.data);

    Ok(SendTokenResponse {
        program_id: ix.program_id.to_string(),
        accounts: ix.accounts.iter().map(|meta| AccountMetaLite {
            pubkey: meta.pubkey.to_string(),
            is_signer: meta.is_signer,
        }).collect(),
        instruction_data,
    })
}
