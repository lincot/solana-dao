use crate::state::*;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = company_authority, seeds = [b"company"], bump, space = 8 + Company::LEN)]
    company: Account<'info, Company>,
    #[account(mut)]
    company_authority: Signer<'info>,
    system_program: Program<'info, System>,
}

pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
    ctx.accounts.company.bump = *ctx.bumps.get("company").unwrap();
    ctx.accounts.company.authority = ctx.accounts.company_authority.key();

    emit!(InitializeEvent {});

    Ok(())
}

#[event]
struct InitializeEvent {}
