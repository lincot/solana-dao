use crate::state::*;
use anchor_lang::prelude::*;

#[derive(Accounts)]
#[instruction(employee_authority: Pubkey)]
pub struct Employ<'info> {
    #[account(seeds = [b"company"], bump = company.bump)]
    company: Account<'info, Company>,
    #[account(mut, address = company.authority)]
    company_authority: Signer<'info>,
    #[account(
        init,
        payer = company_authority,
        seeds = [b"employee", employee_authority.as_ref()],
        bump,
        space = 8 + Employee::LEN,
    )]
    employee: Account<'info, Employee>,
    system_program: Program<'info, System>,
}

pub fn employ(ctx: Context<Employ>, _employee_authority: Pubkey, salary: u64) -> Result<()> {
    ctx.accounts.employee.bump = *ctx.bumps.get("employee").unwrap();
    ctx.accounts.employee.salary = salary;

    emit!(EmployEvent {});

    Ok(())
}

#[event]
struct EmployEvent {}
