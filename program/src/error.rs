use thiserror::Error;
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
    msg,
    program_error::PrintProgramError,
}; 

// Custom error types for the Flash Loan program.
#[derive(Clone, Debug, Eq, Error, PartialEq)]
pub enum FlashLoanError {
    /// Thrown when the provided authority does not match the expected one.
    #[error("Invalid authority provided.")]
    InvalidAuthority,

    /// Thrown when the pool has insufficient liquidity for the loan.
    #[error("Insufficient liquidity in pool.")]
    InsufficientLiquidity,

    /// Thrown when the loan repayment amount is not correct.
    #[error("Repayment amount is incorrect.")]
    InvalidRepaymentAmount,

    /// Thrown when the flash loan isn't repaid within the same transaction.
    #[error("Loan must be repaid in the same transaction.")]
    LoanMustBeRepaidImmediately,

    /// Thrown when a flash loan has already been taken within the same transaction.
    #[error("Flash loan has already been taken.")]
    FlashLoanAlreadyTaken,

    /// Thrown when there is a mismatch in the expected and actual token mint.
    #[error("Token mismatch detected.")]
    InvalidTokenMint,

    /// Thrown when an arithmetic overflow or underflow occurs.
    #[error("Arithmetic overflow or underflow occurred.")]
    MathError,

    /// Thrown when an account accessed is not authorized to perform the action.
    #[error("Unauthorized account access.")]
    UnauthorizedAccess,

    #[error("Insufficient funds in the account")]
    InsufficientFunds,

    #[error("Invalid ATA passed who's owner isn't the specified borrower")]
    InvalidAccountData,

    #[error("Invalid pool id passed in the instruction as it doesnt match with the pool's pda pool_id")]
    InvalidPoolId,

    #[error("Amount can't be zero")]
    InvalidAmount,

    #[error("The passed account is_signer flag is set to false")]
    MissingRequiredSignature,

    #[error("Invalid instruction data provided")]
    InvalidInstructionData
}

// -----------------------------
// Error Integration with Solana
// -----------------------------

/// Allows conversion of `FlashLoanError` into a `ProgramError`
/// Used in your instruction logic like:
/// `return Err(FlashLoanError::InvalidAuthority.into());`
impl From<FlashLoanError> for ProgramError {
    fn from(e: FlashLoanError) -> Self {
        // Convert enum variant to a unique u32 for Solana to interpret
        ProgramError::Custom(e as u32)
    }
}

/// Makes error messages readable in Solana logs.
/// When `PrintProgramError` is called, logs like:
/// `Flash Loan Error: Invalid authority provided.`
impl PrintProgramError for FlashLoanError {
    fn print<E>(&self) {
        msg!("Flash Loan Error: {:#?}", self);
    }
}