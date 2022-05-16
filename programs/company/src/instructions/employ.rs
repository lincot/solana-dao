use crate::state::*;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct Employ<'info> {
    #[account(seeds = [b"company"], bump = company.bump)]
    company: Account<'info, Company>,
    #[account(address = company.authority)]
    company_authority: Signer<'info>,
    #[account(
        init,
        payer = employee_authority,
        seeds = [b"employee", employee_authority.key().as_ref()],
        bump,
        space = 8 + Employee::LEN,
    )]
    employee: Account<'info, Employee>,
    #[account(mut)]
    employee_authority: Signer<'info>,
    system_program: Program<'info, System>,
}

pub fn employ(ctx: Context<Employ>, salary: u64) -> Result<()> {
    ctx.accounts.employee.bump = *ctx.bumps.get("employee").unwrap();
    ctx.accounts.employee.salary = salary;

    emit!(EmployEvent {});

    Ok(())
}

#[event]
struct EmployEvent {}
