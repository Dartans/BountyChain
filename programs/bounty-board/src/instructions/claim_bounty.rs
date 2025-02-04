use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount};
use crate::state::{Bounty, BountyStatus, BountyBoard};

/// Accounts required for claiming a bounty
#[derive(Accounts)]
pub struct ClaimBounty<'info> {
    #[account(mut)]
    pub bounty: Account<'info, Bounty>,        // Bounty account to claim
    
    #[account(mut, has_one = token_mint)]      // Verify board matches bounty
    pub bounty_board: Account<'info, BountyBoard>,
    
    #[account(mut)]                            // Claimant's token account
    pub claimant_reward_account: Account<'info, TokenAccount>,
    
    #[account(mut)]                            // Escrow holding bounty funds
    pub escrow_account: Account<'info, TokenAccount>,
    
    #[account(mut)]                            // Claimant's wallet
    pub claimant: Signer<'info>,
    
    pub token_program: Program<'info, Token>,  // Token program
    pub system_program: Program<'info, System>,// System program
    pub clock: Sysvar<'info, Clock>,           // Sysvar for time checks
}

/// Instruction handler for claiming a bounty
pub fn handler(ctx: Context<ClaimBounty>) -> Result<()> {
    let bounty = &mut ctx.accounts.bounty;
    let clock = &ctx.accounts.clock;
    
    // Validate bounty status
    require!(bounty.status == BountyStatus::Open, ErrorCode::InvalidBountyStatus);
    
    // Check expiration
    require!(clock.unix_timestamp < bounty.expires_at, ErrorCode::BountyExpired);
    
    // Verify claimant hasn't already claimed
    require!(bounty.claimant.is_none(), ErrorCode::AlreadyClaimed);
    
    // Update bounty state
    bounty.status = BountyStatus::Claimed;
    bounty.claimant = Some(ctx.accounts.claimant.key());
    
    // Transfer 50% to claimant immediately
    let initial_payout = bounty.amount.checked_div(2).ok_or(ErrorCode::NumericalOverflow)?;
    
    token::transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.escrow_account.to_account_info(),
                to: ctx.accounts.claimant_reward_account.to_account_info(),
                authority: ctx.accounts.bounty_board.to_account_info(),
            },
        )
        .with_signer(&[&[
            b"bounty_board",
            ctx.accounts.bounty_board.token_mint.as_ref(),
            &[ctx.accounts.bounty_board.bump],
        ]]),
        initial_payout,
    )?;

    // Store remaining amount in escrow for completion
    Ok(())
}