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
    instruction::{FlashLoanInstruction, init_pool, borrow, liquidate_pool, repay, deliquidate_pool}
};

pub fn process(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction: FlashLoanInstruction,
) -> ProgramResult {
    match instruction {
        // pool_id: u8,
        // initial_amount: u64,
        // fees_bps: u16,
        // bump: u8,
        // mint: Pubkey,
        // lp_mint_bump: u8,
        FlashLoanInstruction::InitPool {
            initial_amount,
            fees_bps,
            bump,
            mint,
            lp_mint_bump,
        } => init_pool(program_id, accounts, initial_amount, fees_bps, mint, lp_mint_bump, bump),

        FlashLoanInstruction::Borrow { pool_id, amount, mint, bump} => {
            borrow(program_id, accounts, pool_id, amount, &mint, bump)
        }

        FlashLoanInstruction::Repay { pool_id, amount } => {
            repay(program_id, accounts, pool_id, amount)
        }

        FlashLoanInstruction::LiquidatePool { pool_id, token_amount } => {
            liquidate_pool(program_id, accounts, pool_id, token_amount)
        }
        FlashLoanInstruction::DeLiquidatePool { pool_id, lp_amount } => {
            deliquidate_pool(program_id, accounts, pool_id, lp_amount)
        }
    }
}
