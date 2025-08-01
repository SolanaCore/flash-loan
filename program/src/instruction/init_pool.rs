use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    program_error::ProgramError,
    pubkey::Pubkey,
    rent::Rent,
    system_instruction,
    program::invoke,
    sysvar::Sysvar,
};

use borsh::BorshDeserialize;
use crate::{
    state::Pool as PoolState,
    utils::{transfer_tokens, create_mint},
    error::FlashLoanError,
};
use borsh::BorshSerialize;

pub fn init_pool(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    initial_amount: u64,
    fees_bps: u16,
    mint: Pubkey,
    pool_bump: u8,
    lp_mint_bump: u8,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();

    let user = next_account_info(account_info_iter)?;             // signer
    let pool_account = next_account_info(account_info_iter)?;     // pool state PDA
    let mint = next_account_info(account_info_iter)?;             // input token mint
    let vault = next_account_info(account_info_iter)?;            // token vault
    let lp_mint_account = next_account_info(account_info_iter)?;  // LP mint PDA
    let user_lp_ata = next_account_info(account_info_iter)?;      // user's LP token ATA
    let rent_sysvar = next_account_info(account_info_iter)?;
    let token_program = next_account_info(account_info_iter)?;
    let clock_sysvar = next_account_info(account_info_iter)?;

    if !user.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }

    if initial_amount == 0 {
        return Err(FlashLoanError::InvalidAmount.into());
    }

    let rent = &Rent::from_account_info(rent_sysvar)?;
    if !rent.is_exempt(pool_account.lamports(), pool_account.data_len()) {
        return Err(ProgramError::AccountNotRentExempt);
    }

    // Transfer tokens to vault
    transfer_tokens(
        user.clone(),
        vault.clone(),
        user.clone(),
        token_program.clone(),
        initial_amount,
        None,
    )?;

    let pool_id_bytes = &pool_id.to_le_bytes();
    let signer_seeds: &[&[u8]] = &[b"lp_mint", pool_id_bytes, &[lp_mint_bump]];
    let signer_seeds_ref: &[&[&[u8]]] = &[&signer_seeds[..]];

    // Create LP mint and mint tokens to user's ATA
    create_mint(
        lp_mint_account.clone(),
        &user_lp_ata.key,
        9,
        pool_account.clone(),
        token_program.clone(),
        signer_seeds_ref,
    )?;
    //pool id is also the open_time
    // so how to get current time
    let clock = solana_program::sysvar::clock::Clock::from_account_info(clock_sysvar)?;
    let open_time = clock.unix_timestamp as u8;

    // Initialize pool state
    let pool_data = PoolState {
        pool_id: open_time,
        vault: *vault.key,
        total_liquidity: initial_amount,
        total_lp_supply: initial_amount,
        fees_bps: fees_bps.try_into().unwrap(),
        token_mint: *mint.key,
        bump: pool_bump,
        lp_mint_bump,
        lp_mint: *lp_mint_account.key,
    };
    pool_data.serialize(&mut &mut pool_account.try_borrow_mut_data()?[..])
        .map_err(|_| ProgramError::InvalidAccountData)?;
    
        //transfer lp to the user 
    mint_tokens(
        lp_mint_account.clone(),
        user_lp_ata.clone(),
        pool_account.clone(),
        token_program.clone(),
        initial_amount,
        signer_seeds_ref,
    )?;
    
    Ok(())
}
