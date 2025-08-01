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
use std::mem::size_of;
use shank::ShankAccount;
use borsh::{BorshSerialize, BorshDeserialize, from_slice, to_vec};

///Grouping fixed-size types of similar byte lengths together avoids padding.
#[repr(C)]
#[derive(Clone, Debug, Default, PartialEq, ShankAccount, BorshSerialize, BorshDeserialize)]
pub struct Pool {
    pub token_mint: Pubkey,
    pub lp_mint: Pubkey,
    
    pub vault: Pubkey,
    pub pool_id: u8,

    //Liquidity and supply
    pub total_liquidity: u64,
    pub total_lp_supply: u64,

    pub fees_bps: u8,

    pub bump: u8,
    pub lp_mint_bump: u8,
}

impl Pool {
    pub const LEN: usize = size_of::<Pool>();
}
