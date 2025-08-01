use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    program_error::ProgramError,
    pubkey::Pubkey,
    rent::Rent,
    system_instruction,
    program::invoke,
    sysvar::Sysvar,
}; 
use crate::{
    instruction::FlashLoanInstruction,
    utils::transfer_tokens,
    error::FlashLoanError,
    state::Pool as PoolState,
};

use borsh::BorshDeserialize;

pub fn repay(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    pool_id: u8,
    amount: u64,
) -> ProgramResult {
    
    let account_info_iter = &mut accounts.iter();

    let borrower = next_account_info(account_info_iter)?;
    let pool = next_account_info(account_info_iter)?;
    let source = next_account_info(account_info_iter)?;
    let vault = next_account_info(account_info_iter)?;
    let token_program = next_account_info(account_info_iter)?;
    let clock_sysvar = next_account_info(account_info_iter)?;
    
    let clock = solana_program::sysvar::clock::Clock::from_account_info(clock_sysvar)?;
    let current_time = clock.unix_timestamp as u8;

    //The pool id is the open_time of the pool, so we check if the current time is greater than the pool id
    if current_time > pool_id {
        return Err(FlashLoanError::InvalidPoolId.into());
    }
    if !borrower.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }

    if amount == 0 {
        return Err(FlashLoanError::InvalidAmount.into());
    }

    let pool_data = PoolState::try_from_slice(&pool.try_borrow_data()?)?;
    if pool_id != pool_data.pool_id {
        return Err(ProgramError::InvalidAccountData);
    }

    transfer_tokens(
        source.clone(),
        vault.clone(),
        borrower.clone(),
        token_program.clone(),
        amount,
        None,
    )?;

    Ok(())
}