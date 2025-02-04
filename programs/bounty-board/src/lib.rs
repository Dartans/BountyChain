use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};
use crate::{state::*, instructions::*};

declare_id!("BOUNTYBOARDCHAINPROGRAMID");

// Main program module containing instruction handlers
#[program]
pub mod bounty_board {
    use super::*;

    /// Initializes the bounty board with admin and token mint
    pub fn initialize_board(ctx: Context<InitializeBoard>) -> Result<()> {
        instructions::initialize_board::handler(ctx)
    }

    /// Creates a new bounty with specified parameters
    pub fn create_bounty(
        ctx: Context<CreateBounty>,
        amount: u64,
        github_issue: String,
        expires_at: i64,
    ) -> Result<()> {
        instructions::create_bounty::handler(ctx, amount, github_issue, expires_at)
    }

    /// Processes payout for completed bounty with distribution
    pub fn process_payout(
        ctx: Context<ProcessPayout>,
        amount: u64,
        github_issue: String,
    ) -> Result<()> {
        instructions::process_payout::handler(ctx, amount, github_issue)
    }
}

// Custom error codes for program validation
#[error_code]
pub enum ErrorCode {
    #[msg("Invalid bounty status for this operation")]
    InvalidBountyStatus,
    #[msg("Bounty has expired")]
    BountyExpired,
    #[msg("Unauthorized access attempt")]
    Unauthorized,
    #[msg("Arithmetic overflow/underflow detected")]
    NumericalOverflow,
    #[msg("Invalid account configuration")]
    InvalidAccountConfig,
}