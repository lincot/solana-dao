use crate::instructions::*;
use anchor_lang::prelude::*;

pub mod error;
pub mod instructions;
pub mod state;
pub mod utils;

declare_id!("6hkoUJeFTLQuB4Xt7G8VAW5vsHo9nvvU2Pz4QWPtBw8k");

#[program]
pub mod academy {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, mntr_mint: Pubkey) -> Result<()> {
        instructions::initialize(ctx, mntr_mint)
    }

    pub fn register_student(
        ctx: Context<RegisterStudent>,
        mentors: Vec<Pubkey>,
        tasks_amount: u8,
        task_time: u32,
    ) -> Result<()> {
        instructions::register_student(ctx, mentors, tasks_amount, task_time)
    }

    pub fn set_grade(ctx: Context<SetGrade>, student_authority: Pubkey, grade: u64) -> Result<()> {
        instructions::set_grade(ctx, student_authority, grade)
    }

    pub fn end_task(ctx: Context<EndTask>) -> Result<()> {
        instructions::end_task(ctx)
    }

    pub fn expel_student(ctx: Context<ExpelStudent>) -> Result<()> {
        instructions::expel_student(ctx)
    }
}
