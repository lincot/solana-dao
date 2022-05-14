use crate::state::*;
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount};

#[derive(Accounts)]
pub struct RegisterMentor<'info> {
    #[account(seeds = [b"dao"], bump = dao.bump)]
    dao: Account<'info, Dao>,
    #[account(address = dao.authority)]
    dao_authority: Signer<'info>,
    #[account(mut, seeds = [b"mntr_mint"], bump = dao.bump_mntr_mint)]
    mntr_mint: Account<'info, Mint>,
    #[account(
        init,
        payer = mentor_authority,
        seeds = [b"mentor", mentor_authority.key().as_ref()],
        bump,
        space = 8 + Mentor::LEN,
    )]
    mentor: Account<'info, Mentor>,
    #[account(mut)]
    mentor_authority: Signer<'info>,
    #[account(mut, associated_token::authority = mentor, associated_token::mint = mntr_mint)]
    mentor_mntr: Account<'info, TokenAccount>,
    token_program: Program<'info, Token>,
    system_program: Program<'info, System>,
}

fn mint_mntr(ctx: &Context<RegisterMentor>, amount: u64) -> Result<()> {
    let signer: &[&[&[u8]]] = &[&[b"dao", &[ctx.accounts.dao.bump]]];
    let cpi_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        token::MintTo {
            mint: ctx.accounts.mntr_mint.to_account_info(),
            to: ctx.accounts.mentor_mntr.to_account_info(),
            authority: ctx.accounts.dao.to_account_info(),
        },
        signer,
    );
    token::mint_to(cpi_ctx, amount)
}

pub fn register_mentor(ctx: Context<RegisterMentor>, power: u64) -> Result<()> {
    mint_mntr(&ctx, power)?;

    ctx.accounts.mentor.bump = *ctx.bumps.get("mentor").unwrap();

    emit!(RegisterMentorEvent {});

    Ok(())
}

#[event]
struct RegisterMentorEvent {}
