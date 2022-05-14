use crate::instructions::*;
use anchor_lang::prelude::*;

pub mod error;
pub mod instructions;
pub mod state;
pub mod utils;

declare_id!("DQb5ikQX5msGg7Px7YbdwEsyLapcnrNe9JTM3M9TC5Jq");

#[program]
pub mod dao {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        instructions::initialize(ctx)
    }

    pub fn register_mentor(ctx: Context<RegisterMentor>, power: u64) -> Result<()> {
        instructions::register_mentor(ctx, power)
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
}
