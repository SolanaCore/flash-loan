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

pub mod instruction;
pub mod processor;
pub mod error;
pub mod state;
pub mod utils; 

pub use state::*;
pub use instruction::*;
pub use processor::*;
pub use error::*;
pub use utils::*;

entrypoint!(process_instruction);

use crate::{
    instruction::FlashLoanInstruction,
    processor::process,
};

pub const ID: Pubkey = solana_program::pubkey!(
    "EgB1zom79Ek4LkvJjafbkUMTwDK9sZQKEzNnrNFHpHHz"
);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let ix = FlashLoanInstruction::unpack(instruction_data)?;
    process(program_id, accounts, ix)
}

// Auto-generated integration tests for the FlashLoan program using LiteSVM

use litesvm::LiteSVM;
use solana_sdk::{
    instruction::Instruction,
    message::Message,
    pubkey::Pubkey,
    signature::{Keypair, Signer},
    transaction::Transaction,
};
use flash_loan::FlashLoanInstruction;
use borsh::BorshSerialize;

const PROGRAM_PATH: &str = "target/deploy/flash_loan.so";

fn init_test_env() -> (LiteSVM, Keypair, Pubkey) {
    let mut svm = LiteSVM::new();
    let payer = Keypair::new();
    svm.airdrop(&payer.pubkey(), 1_000_000_000).unwrap();

    let program_keypair = Keypair::new();
    let program_id = program_keypair.pubkey();
    svm.add_program_from_file(program_id, PROGRAM_PATH).unwrap();

    (svm, payer, program_id)
}

#[test]
fn test_init_pool() {
    let (mut svm, payer, program_id) = init_test_env();

    let ix_data = FlashLoanInstruction::InitPool {
        initial_amount: 1_000_000,
        fees_bps: 50,
        bump: 255,
        mint: Pubkey::new_unique(),
        lp_mint_bump: 255,
    }
    .try_to_vec()
    .unwrap();

    let ix = Instruction {
        program_id,
        accounts: vec![], // add correct AccountMetas here
        data: ix_data,
    };

    let message = Message::new(&[ix], Some(&payer.pubkey()));
    let tx = Transaction::new(&[&payer], message, svm.latest_blockhash());

    let result = svm.send_transaction(tx);
    assert!(result.is_ok(), "InitPool tx failed: {:#?}", result);
}

#[test]
fn test_liquidate_pool() {
    let (mut svm, payer, program_id) = init_test_env();

    let ix_data = FlashLoanInstruction::LiquidatePool {
        pool_id: 0,
        token_amount: 1_000,
    }
    .try_to_vec()
    .unwrap();

    let ix = Instruction {
        program_id,
        accounts: vec![], // add correct AccountMetas
        data: ix_data,
    };

    let message = Message::new(&[ix], Some(&payer.pubkey()));
    let tx = Transaction::new(&[&payer], message, svm.latest_blockhash());
    let result = svm.send_transaction(tx);
    assert!(result.is_ok(), "LiquidatePool tx failed");
}

#[test]
fn test_de_liquidate_pool() {
    let (mut svm, payer, program_id) = init_test_env();

    let ix_data = FlashLoanInstruction::DeLiquidatePool {
        pool_id: 0,
        lp_amount: 500,
    }
    .try_to_vec()
    .unwrap();

    let ix = Instruction {
        program_id,
        accounts: vec![],
        data: ix_data,
    };

    let message = Message::new(&[ix], Some(&payer.pubkey()));
    let tx = Transaction::new(&[&payer], message, svm.latest_blockhash());
    let result = svm.send_transaction(tx);
    assert!(result.is_ok(), "DeLiquidatePool tx failed");
}

#[test]
fn test_borrow() {
    let (mut svm, payer, program_id) = init_test_env();

    let ix_data = FlashLoanInstruction::Borrow {
        pool_id: 0,
        amount: 1_000,
        mint: Pubkey::new_unique(),
        bump: 255,
    }
    .try_to_vec()
    .unwrap();

    let ix = Instruction {
        program_id,
        accounts: vec![],
        data: ix_data,
    };

    let message = Message::new(&[ix], Some(&payer.pubkey()));
    let tx = Transaction::new(&[&payer], message, svm.latest_blockhash());
    let result = svm.send_transaction(tx);
    assert!(result.is_ok(), "Borrow tx failed");
}

#[test]
fn test_repay() {
    let (mut svm, payer, program_id) = init_test_env();

    let ix_data = FlashLoanInstruction::Repay {
        pool_id: 0,
        amount: 1_000,
    }
    .try_to_vec()
    .unwrap();

    let ix = Instruction {
        program_id,
        accounts: vec![],
        data: ix_data,
    };

    let message = Message::new(&[ix], Some(&payer.pubkey()));
    let tx = Transaction::new(&[&payer], message, svm.latest_blockhash());
    let result = svm.send_transaction(tx);
    assert!(result.is_ok(), "Repay tx failed");
}