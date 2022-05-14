use crate::state::*;
use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token};

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = dao_authority, seeds = [b"dao"], bump, space = 8 + Dao::LEN)]
    dao: Account<'info, Dao>,
    #[account(mut)]
    dao_authority: Signer<'info>,
    #[account(
        init,
        payer = dao_authority,
        seeds = [b"mntr_mint"],
        bump,
        mint::authority = dao,
        mint::decimals = 6,
    )]
    mntr_mint: Account<'info, Mint>,
    rent: Sysvar<'info, Rent>,
    token_program: Program<'info, Token>,
    system_program: Program<'info, System>,
}

pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
    ctx.accounts.dao.bump = *ctx.bumps.get("dao").unwrap();
    ctx.accounts.dao.bump_mntr_mint = *ctx.bumps.get("mntr_mint").unwrap();
    ctx.accounts.dao.authority = ctx.accounts.dao_authority.key();

    emit!(InitializeEvent {});

    Ok(())
}

#[event]
struct InitializeEvent {}
