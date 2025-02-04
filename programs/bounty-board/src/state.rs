use anchor_lang::prelude::*;
use anchor_spl::token::TokenAccount;

// Main program state stored as an account
#[account]
#[derive(Default)]
pub struct BountyBoard {
    pub admin: Pubkey,          // Admin wallet address
    pub token_mint: Pubkey,     // Mint address of the reward token
    pub total_bounties: u64,    // Counter for total bounties created
    pub total_payouts: u64,     // Total tokens paid out across all bounties
    pub last_payout: i64,       // Timestamp of last successful payout
    pub bump: u8,               // PDA bump seed for security
}

// Individual bounty account structure
#[account]
#[derive(Default)]
pub struct Bounty {
    pub amount: u64,            // Bounty reward amount in tokens
    pub github_issue: String,   // Associated GitHub issue URL
    pub expires_at: i64,        // Unix timestamp for bounty expiration
    pub status: BountyStatus,   // Current state of the bounty
    pub claimant: Option<Pubkey>, // Wallet address of claimant (if any)
    pub bump: u8,               // PDA bump seed for security
}

// Enum representing bounty lifecycle states
#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum BountyStatus {
    Open,      // Available for claiming
    Claimed,   // Work claimed but not completed
    Completed, // Work verified and paid out
    Expired,   // Bounty expired without completion
}