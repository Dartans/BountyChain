use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Bounty is not in open state")]
    InvalidBountyStatus,
    #[msg("Bounty has expired")]
    BountyExpired,
    #[msg("Bounty already claimed")]
    AlreadyClaimed,
    #[msg("Insufficient funds in escrow")]
    InsufficientFunds,
    #[msg("Invalid token account owner")]
    InvalidTokenAccount,
    // ... (other error codes remain same)
}