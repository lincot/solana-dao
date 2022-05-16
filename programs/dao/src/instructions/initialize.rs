use crate::state::*;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = dao_authority, seeds = [b"dao"], bump, space = 8 + Dao::LEN)]
    dao: Account<'info, Dao>,
    #[account(mut)]
    dao_authority: Signer<'info>,
    system_program: Program<'info, System>,
}

pub fn initialize(ctx: Context<Initialize>, mntr_mint: Pubkey) -> Result<()> {
    ctx.accounts.dao.bump = *ctx.bumps.get("dao").unwrap();
    ctx.accounts.dao.mntr_mint = mntr_mint;
    ctx.accounts.dao.authority = ctx.accounts.dao_authority.key();

    emit!(InitializeEvent {});

    Ok(())
}

#[event]
struct InitializeEvent {}
