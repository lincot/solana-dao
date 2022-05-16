use crate::instructions::*;
use anchor_lang::prelude::*;

pub mod error;
pub mod instructions;
pub mod state;
pub mod utils;

declare_id!("A46Aij7LFNy5JW1EtDbQpCtHbw3EB1CWFvirnQMJ2Fdk");

#[program]
pub mod company {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        instructions::initialize(ctx)
    }

    pub fn employ(ctx: Context<Employ>, employee_authority: Pubkey, salary: u64) -> Result<()> {
        instructions::employ(ctx, employee_authority, salary)
    }
}
