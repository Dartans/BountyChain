use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};
use crate::state::BountyBoard;

// Account validation structure for ProcessPayout instruction
#[derive(Accounts)]
#[instruction(amount: u64, github_issue: String)]
pub struct ProcessPayout<'info> {
    #[account(mut, has_one = admin)] // Verify admin authority
    pub bounty_board: Account<'info, BountyBoard>,
    
    #[account(mut)]
    pub admin: Signer<'info>, // Payout authorizer
    
    #[account(mut)]
    pub escrow_account: Account<'info, TokenAccount>,
    
    #[account(mut)]
    pub destination: Account<'info, TokenAccount>, // Developer's account
    
    #[account(mut)]
    pub public_pool: Account<'info, TokenAccount>, // Community pool
    
    #[account(mut)]
    pub maintainers_pool: Account<'info, TokenAccount>, // Maintainers fund
    
    pub token_program: Program<'info, Token>,
}

// Instruction handler implementation
pub fn handler(
    ctx: Context<ProcessPayout>,
    amount: u64,
    _github_issue: String,
) -> Result<()> {
    // Calculate distribution with overflow checks
    let developer_share = amount
        .checked_mul(995)
        .and_then(|v| v.checked_div(1000))
        .ok_or(ErrorCode::NumericalOverflow)?;

    let public_pool_share = amount
        .checked_mul(3)
        .and_then(|v| v.checked_div(1000))
        .ok_or(ErrorCode::NumericalOverflow)?;

    let maintainers_share = amount
        .checked_mul(2)
        .and_then(|v| v.checked_div(1000))
        .ok_or(ErrorCode::NumericalOverflow)?;

    // Execute token transfers using CPI
    let accounts = Transfer {
        from: ctx.accounts.escrow_account.to_account_info(),
        to: ctx.accounts.destination.to_account_info(),
        authority: ctx.accounts.bounty_board.to_account_info(),
    };
    
    token::transfer(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            accounts,
            &[&[
                b"bounty_board",
                ctx.accounts.bounty_board.token_mint.as_ref(),
                &[ctx.accounts.bounty_board.bump],
            ]],
        ),
        developer_share,
    )?;

    // Similar transfers for public_pool and maintainers_pool

    // Update board state
    ctx.accounts.bounty_board.total_payouts = ctx
        .accounts
        .bounty_board
        .total_payouts
        .checked_add(amount)
        .ok_or(ErrorCode::NumericalOverflow)?;

    ctx.accounts.bounty_board.last_payout = Clock::get()?.unix_timestamp;

    Ok(())
}