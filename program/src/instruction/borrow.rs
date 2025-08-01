use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    program_error::ProgramError,
    pubkey::Pubkey,
    sysvar::{instructions::{load_current_index_checked, load_instruction_at_checked}, Sysvar},
};
use spl_token::state::Account as TokenAccount;

use crate::{
    error::FlashLoanError,
    instruction::FlashLoanInstruction,
    state::Pool as PoolState,
    utils::transfer_tokens,
};
use borsh::{BorshSerialize, BorshDeserialize};
use spl_token::solana_program::program_pack::Pack;

pub fn borrow(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    pool_id: u8,
    amount: u64,
    mint: &Pubkey,
    bump: u8, // <-- Add bump as parameter
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();

    // === Account Inputs ===
    let borrower = next_account_info(account_info_iter)?;                 // signer
    let pool = next_account_info(account_info_iter)?;                     // pool state PDA
    let vault = next_account_info(account_info_iter)?;                    // pool vault (source)
    let borrower_token_account = next_account_info(account_info_iter)?;  // recipient ATA (destination)
    let token_program = next_account_info(account_info_iter)?;           // token program
    let instructions_sysvar_account = next_account_info(account_info_iter)?; // instruction sysvar
    let clock_sysvar = next_account_info(account_info_iter)?; // clock sysvar

    let clock = solana_program::sysvar::clock::Clock::from_account_info(clock_sysvar)?;
    let current_time = clock.unix_timestamp as u8;

    //The pool id is the open_time of the pool, so we check if the current time is greater than the pool id
    if current_time > pool_id {
        return Err(FlashLoanError::InvalidPoolId.into());
    }

    // === Signer check ===
    if !borrower.is_signer {
        return Err(FlashLoanError::MissingRequiredSignature.into());
    }

    if amount == 0 {
        return Err(FlashLoanError::InvalidAmount.into());
    }

    let pool_data = PoolState::try_from_slice(&pool.try_borrow_data()?)?;
    if pool_id != pool_data.pool_id {
        return Err(FlashLoanError::InvalidPoolId.into());
    }

    // === Check if ATA exists and is valid ===
    if borrower_token_account.lamports() == 0 {
        return Err(FlashLoanError::InvalidAccountData.into()); // ATA must be pre-created externally
    } else {
        let ata_data = TokenAccount::unpack(&borrower_token_account.try_borrow_data()?)?;
        if ata_data.owner != *borrower.key || ata_data.mint != *mint {
            return Err(FlashLoanError::InvalidAccountData.into());
        }
    }

    // === Fee calculation ===
    let fee = amount
        .checked_mul(pool_data.fees_bps as u64)
        .ok_or(FlashLoanError::MathError)?
        .checked_div(10_000)
        .ok_or(FlashLoanError::MathError)?;

    let borrow_amount = amount
        .checked_add(fee)
        .ok_or(FlashLoanError::MathError)?;

    // === Vault balance check (ensure it has enough tokens) ===
    // Optional: Load vault token account and check amount, similar to ATA

    // === Instruction introspection to enforce repay next ===
    let current_ix_index = load_current_index_checked(instructions_sysvar_account)?;
    let repay_ix = load_instruction_at_checked(current_ix_index as usize + 1, instructions_sysvar_account)?;

    if repay_ix.program_id != *program_id {
        return Err(FlashLoanError::InvalidInstructionData.into());
    }

    let repay_ix_data = FlashLoanInstruction::try_from_slice(&repay_ix.data)
        .map_err(|_| FlashLoanError::InvalidInstructionData)?;

    match repay_ix_data {
        FlashLoanInstruction::Repay {
            pool_id: repay_pool_id,
            amount: repay_amount,
        } => {
            if repay_pool_id != pool_data.pool_id || repay_amount != borrow_amount {
                return Err(FlashLoanError::InvalidInstructionData.into());
            }
        }
        _ => return Err(FlashLoanError::InvalidInstructionData.into()),
    }

    // === Transfer tokens from vault to borrower ATA ===
    let signer_seeds: &[&[u8]] = &[b"lp_mint", &pool_id.to_le_bytes(), &[bump]];
    let signer_seed_slice: &[&[&[u8]]] = &[signer_seeds];

    transfer_tokens(
        vault.clone(),
        borrower_token_account.clone(),
        pool.clone(), // vault authority (PDA)
        token_program.clone(),
        borrow_amount,
        Some(signer_seed_slice),
    )?;

    Ok(())
}
