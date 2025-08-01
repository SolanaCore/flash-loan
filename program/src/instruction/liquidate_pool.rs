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
        FlashLoanError,
        mint_tokens, 
        burn_tokens, 
        transfer_tokens,
        state::Pool as PoolState,
    };

    use borsh::BorshDeserialize;
    use spl_associated_token_account::get_associated_token_address;


pub fn liquidate_pool(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    pool_id: u8,
    token_amount: u64,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let user = next_account_info(account_info_iter)?;
    let user_ata = next_account_info(account_info_iter)?;
    let pool = next_account_info(account_info_iter)?;
    let vault = next_account_info(account_info_iter)?;
    let lp_mint = next_account_info(account_info_iter)?;
    let lp_ata = next_account_info(account_info_iter)?;
    let token_program = next_account_info(account_info_iter)?;
    if !user.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }

    let mut pool_data = PoolState::try_from_slice(&pool.try_borrow_data()?)?;
    if pool_data.pool_id != pool_id.into() {
        return Err(FlashLoanError::InvalidPoolId.into());
    }
    if user_ata.key != &get_associated_token_address(&user.key, &pool_data.token_mint) {
        return Err(FlashLoanError::InvalidAccountData.into());
    }
    transfer_tokens(
        user_ata.clone(),
        vault.clone(),
        user.clone(),
        token_program.clone(),
        token_amount,
        None
    )?;

    let share = token_amount
        .checked_div(pool_data.total_liquidity)
        .ok_or(FlashLoanError::MathError)?;
    
        let (lp_mint_key, _bump) = Pubkey::find_program_address(
        &[b"lp_mint", &pool_data.pool_id.to_le_bytes()],
        program_id,
    );
    if lp_mint.key != &lp_mint_key {
        return Err(FlashLoanError::InvalidAccountData.into());
    }

    pool_data.total_liquidity += token_amount;
    let lp_to_mint = share * pool_data.total_lp_supply;
    if lp_to_mint == 0 {
        return Err(FlashLoanError::InvalidAmount.into());
    }
    let pool_id_bytes = &pool_id.to_le_bytes();
    let seeds = &[b"vault", pool_id_bytes.as_ref(), &[pool_data.lp_mint_bump]];
    let signer_seeds: &[&[&[u8]]] = &[&seeds[..]];

    mint_tokens(
        lp_mint.clone(),
        lp_ata.clone(),
        pool.clone(),
        token_program.clone(),
        lp_to_mint,
        signer_seeds-
    )?;

    Ok(())
}

pub fn deliquidate_pool(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    pool_id: u8,
    lp_amount: u64,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let user = next_account_info(account_info_iter)?;
    let pool = next_account_info(account_info_iter)?;
    let lp_mint = next_account_info(account_info_iter)?;
    let lp_ata = next_account_info(account_info_iter)?;
    let token_program = next_account_info(account_info_iter)?;
    if !user.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }

    let mut pool_data = PoolState::try_from_slice(&pool.try_borrow_data()?)?;
    if pool_data.pool_id != pool_id {
        return Err(FlashLoanError::InvalidPoolId.into());
    }

    if lp_mint.key != &pool_data.lp_mint {
        return Err(FlashLoanError::InvalidAccountData.into());
    }

    let share = lp_amount
        .checked_div(pool_data.total_lp_supply)
        .ok_or(FlashLoanError::MathError)?;

    let tokens_out = share
        .checked_mul(pool_data.total_liquidity)
        .ok_or(FlashLoanError::MathError)?;

    if tokens_out == 0 {
        return Err(FlashLoanError::InvalidAmount.into());
    }

    //burn the mint token
    let ata = get_associated_token_address(&user.key, &lp_mint.key);
    //verify that the mint of the ata matches the lp_mint
    if ata != *lp_ata.key {
        return Err(FlashLoanError::InvalidAccountData.into());
    }

    burn_tokens(
        lp_ata.clone(),
        lp_mint.clone(),
        user.clone(),
        token_program.clone(),
        lp_amount,
    )?;

    pool_data.total_liquidity = pool_data.total_liquidity
        .checked_sub(tokens_out)
        .ok_or(FlashLoanError::MathError)?;

    Ok(())
}