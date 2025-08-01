use solana_program::{
    account_info::AccountInfo,
    program::{invoke, invoke_signed},
    program_error::ProgramError,
    pubkey::Pubkey,
    sysvar::rent::Rent,
    instruction::Instruction,
};

use spl_token::instruction::{burn, initialize_mint, mint_to, transfer};
use spl_associated_token_account::instruction::create_associated_token_account;

/// Creates a new mint account (should already be allocated & rent exempt).
pub fn create_mint<'a>(
    mint: AccountInfo<'a>,
    mint_authority: &Pubkey,
    decimals: u8,
    rent_sysvar: AccountInfo<'a>,
    token_program: AccountInfo<'a>,
    signer_seeds: &[&[&[u8]]],
) -> Result<(), ProgramError> {
    let ix = initialize_mint(
        &token_program.key,
        &mint.key,
        mint_authority,
        None,
        decimals,
    )?;

    solana_program::program::invoke_signed(&ix, &[mint, rent_sysvar, token_program], signer_seeds)
}

/// Creates an associated token account for a wallet + mint.
pub fn create_ata<'a>(
    payer: AccountInfo<'a>,
    wallet: &Pubkey,
    mint: &Pubkey,
    ata: AccountInfo<'a>,
    token_program: AccountInfo<'a>,
    ata_program: AccountInfo<'a>,
    system_program: AccountInfo<'a>,
) -> Result<(), ProgramError> {
    let ix = create_associated_token_account(
        &payer.key,
        wallet,
        mint,
        &token_program.key,
    );

    invoke(
        &ix,
        &[
            payer,
            ata,
            token_program,
            ata_program,
            system_program,
        ],
    )
}

/// Transfers tokens (signed if signer_seeds are provided).
pub fn transfer_tokens<'a>(
    source: AccountInfo<'a>,
    destination: AccountInfo<'a>,
    authority: AccountInfo<'a>,
    token_program: AccountInfo<'a>,
    amount: u64,
    signer_seeds: Option<&[&[&[u8]]]> // Some => signed, None => unsigned
) -> Result<(), ProgramError> {
    let ix = transfer(
        &token_program.key,
        &source.key,
        &destination.key,
        &authority.key,
        &[],
        amount,
    )?;

    match signer_seeds {
        Some(seeds) => invoke_signed(&ix, &[source, destination, authority, token_program], seeds),
        None => invoke(&ix, &[source, destination, authority, token_program]),
    }
}

/// Mints tokens to a destination account.
pub fn mint_tokens<'a>(
    mint: AccountInfo<'a>,
    destination: AccountInfo<'a>,
    mint_authority: AccountInfo<'a>,
    token_program: AccountInfo<'a>,
    amount: u64,
    signer_seeds: &[&[&[u8]]],
) -> Result<(), ProgramError> {
    let ix = mint_to(
        &token_program.key,
        &mint.key,
        &destination.key,
        &mint_authority.key,
        &[],
        amount,
    )?;

    invoke_signed(
        &ix,
        &[mint, destination, mint_authority, token_program],
        signer_seeds,
    )
}

/// Burns tokens from an account.
pub fn burn_tokens<'a>(
    account: AccountInfo<'a>,
    mint: AccountInfo<'a>,
    authority: AccountInfo<'a>,
    token_program: AccountInfo<'a>,
    amount: u64,
) -> Result<(), ProgramError> {
    let ix = spl_token::instruction::burn(
        &token_program.key,
        &account.key,
        &mint.key,
        &authority.key,
        &[],
        amount,
    )?;

    invoke(&ix, &[account, mint, authority, token_program])
}
