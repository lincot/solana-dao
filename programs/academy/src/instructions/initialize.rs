use crate::state::*;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = academy_authority, seeds = [b"academy"], bump, space = 8 + Academy::LEN)]
    academy: Account<'info, Academy>,
    #[account(mut)]
    academy_authority: Signer<'info>,
    system_program: Program<'info, System>,
}

pub fn initialize(ctx: Context<Initialize>, mntr_mint: Pubkey) -> Result<()> {
    ctx.accounts.academy.bump = *ctx.bumps.get("academy").unwrap();
    ctx.accounts.academy.mntr_mint = mntr_mint;
    ctx.accounts.academy.authority = ctx.accounts.academy_authority.key();

    emit!(InitializeEvent {});

    Ok(())
}

#[event]
struct InitializeEvent {}
