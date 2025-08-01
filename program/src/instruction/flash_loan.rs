use shank::ShankInstruction;
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

use borsh::{BorshSerialize, BorshDeserialize};

#[derive(ShankInstruction, BorshSerialize, BorshDeserialize)]
pub enum FlashLoanInstruction {
    /// 0 - Initialize the flash loan pool
    ///
    /// Creates a new pool with a vault and LP token mint.
    #[account(0, signer, name = "user", desc = "User initializing the pool")]
    #[account(1, writable, name = "pool", desc = "Pool state account (PDA) ")]
    #[account(2, name = "token_mint", desc = "Token mint to be used in the pool")]
    #[account(3, writable, name = "vault", desc = "Vault token account (PDA)")]
    #[account(4, writable, name = "lp_mint_account", desc = "LP token mint account (PDA)")]
    #[account(5, writable, name = "user_lp_ata", desc = "User's LP token associated token account")]
    #[account(6, name = "token_program", desc = "SPL Token program")]
    #[account(7, name = "clock", desc = "Clock sysvar for timestamps")]
    #[account(8, name = "rent", desc = "Rent sysvar for rent exemption")]
    InitPool {
        initial_amount: u64,
        fees_bps: u16,
        bump: u8,
        mint: Pubkey,
        lp_mint_bump: u8,
    },

    /// 1 - Liquidate the pool
    ///
    /// User deposits tokens and receives LP tokens in return.
    #[account(0, signer, name = "user", desc = "User initiating liquidation")]
    #[account(1, writable, name = "user_ata", desc = "User's token account to deposit")]
    #[account(2, writable, name = "pool", desc = "Pool state account (PDA)")]
    #[account(3, writable, name = "vault", desc = "Vault token account (PDA)")]
    #[account(4, name = "lp_mint", desc = "LP token mint")]
    #[account(5, name = "lp_ata", desc = "User's LP token account")]
    #[account(6, name = "token_program", desc = "SPL Token program")]
    LiquidatePool {
        pool_id: u8,
        token_amount: u64,
    },

    /// 2 - De-liquidate the pool
    ///
    /// User redeems LP tokens to get original tokens back.
    #[account(0, signer, name = "user", desc = "User initiating de-liquidation")]
    #[account(1, writable, name = "user_ata", desc = "User's token account to deposit")]
    #[account(2, writable, name = "pool", desc = "Pool state account (PDA)")]
    #[account(3, writable, name = "vault", desc = "Vault token account (PDA)")]
    #[account(4, name = "lp_mint", desc = "LP token mint")]
    #[account(5, name = "lp_ata", desc = "User's LP token account")]
    #[account(6, name = "token_program", desc = "SPL Token program")]
    DeLiquidatePool {
        pool_id: u8,
        lp_amount: u64,
    },

    /// 3 - Borrow a flash loan
    ///
    /// Borrow tokens from the vault with the requirement of same-transaction repayment.
    #[account(0, signer, name = "borrower", desc = "User borrowing tokens")]
    #[account(1, writable, name = "pool", desc = "Pool state account (PDA)")]
    #[account(2, writable, name = "vault", desc = "Pool's vault holding tokens")]
    #[account(3, writable, name = "borrower_token_account", desc = "Borrower's token account to receive funds")]
    #[account(4, name = "instruction_sysvar", desc = "Instruction Sysvar for CPI introspection")]
    #[account(5, name = "token_program", desc = "SPL Token program")]
    #[account(6, name = "clock", desc = "Clock sysvar for timestamps")]
    Borrow {
        pool_id: u8,      // Ensures correct pool/vault usage
        amount: u64,      // Amount to borrow
        mint: Pubkey,     // Token mint being borrowed
        bump: u8,         // PDA bump for vault authority
    },

    /// 4 - Repay flash loan
    ///
    /// Repay the borrowed amount to the vault.
    #[account(0, signer, name = "borrower", desc = "User repaying the loan")]
    #[account(1, writable, name = "pool", desc = "Pool state account (PDA)")]
    #[account(2, writable, name = "source", desc = "User's token account sending repayment")]
    #[account(3, writable, name = "vault", desc = "Vault receiving the repayment")]
    #[account(4, name = "token_program", desc = "SPL Token program")]
    #[account(5, name = "clock", desc = "Clock sysvar for timestamps")]
    Repay {
        pool_id: u8,     // Ensures repayment targets the correct pool
        amount: u64,     // Amount being repaid
    },
}

impl FlashLoanInstruction {
    /// Unpacks serialized instruction data into an enum variant
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        Self::try_from_slice(input).map_err(|_| ProgramError::InvalidInstructionData)
    }
}
