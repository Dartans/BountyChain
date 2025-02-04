use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount};
use crate::state::{BountyBoard, Bounty, BountyStatus};

// Account validation structure for CreateBounty instruction
#[derive(Accounts)]
pub struct CreateBounty<'info> {
    #[account(mut, has_one = admin)] // Validate admin matches board
    pub bounty_board: Account<'info, BountyBoard>,
    
    #[account(mut)]
    pub admin: Signer<'info>, // Payer and authority
    
    #[account(
        init,
        payer = admin,
        space = 8 + Bounty::LEN, // Allocate space for bounty account
        seeds = [b"bounty", bounty_board.key().as_ref(), &bounty_board.total_bounties.to_le_bytes()],
        bump // Generate PDA for deterministic address
    )]
    pub bounty: Account<'info, Bounty>,
    
    #[account(mut, 
        associated_token::mint = token_mint, 
        associated_token::authority = bounty_board
    )]
    pub escrow_account: Account<'info, TokenAccount>,
    
    pub token_mint: Account<'info, anchor_spl::token::Mint>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

// Instruction handler implementation
pub fn handler(
    ctx: Context<CreateBounty>,
    amount: u64,
    github_issue: String,
    expires_at: i64,
) -> Result<()> {
    let bounty = &mut ctx.accounts.bounty;
    
    // Initialize bounty state
    bounty.amount = amount;
    bounty.github_issue = github_issue;
    bounty.expires_at = expires_at;
    bounty.status = BountyStatus::Open;
    bounty.bump = *ctx.bumps.get("bounty").unwrap();

    // Transfer tokens to escrow using CPI
    let transfer_ctx = CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        Transfer {
            from: ctx.accounts.admin.to_account_info(),
            to: ctx.accounts.escrow_account.to_account_info(),
            authority: ctx.accounts.admin.to_account_info(),
        },
    );
    
    token::transfer(transfer_ctx, amount)?;
    
    // Update board state with overflow protection
    ctx.accounts.bounty_board.total_bounties = ctx
        .accounts
        .bounty_board
        .total_bounties
        .checked_add(1)
        .ok_or(ErrorCode::NumericalOverflow)?;

    Ok(())
}