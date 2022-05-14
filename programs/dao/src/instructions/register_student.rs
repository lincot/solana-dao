use crate::{error::*, state::*};
use anchor_lang::prelude::*;

#[derive(Accounts)]
#[instruction(mentors: Vec<Pubkey>)]
pub struct RegisterStudent<'info> {
    #[account(seeds = [b"dao"], bump = dao.bump)]
    dao: Account<'info, Dao>,
    #[account(address = dao.authority)]
    dao_authority: Signer<'info>,
    #[account(
        init,
        payer = student_authority,
        seeds = [b"student", student_authority.key().as_ref()],
        bump,
        space = 8 + Student::LEN + mentors.len() * Grade::LEN,
    )]
    student: Account<'info, Student>,
    #[account(mut)]
    student_authority: Signer<'info>,
    system_program: Program<'info, System>,
}

pub fn register_student(
    ctx: Context<RegisterStudent>,
    mentors: Vec<Pubkey>,
    total_tasks: u8,
    task_duration: u32,
) -> Result<()> {
    let now = Clock::get()?.unix_timestamp as u32;

    if total_tasks == 0 {
        return err!(DaoError::Zero);
    }

    ctx.accounts.student.bump = *ctx.bumps.get("student").unwrap();
    ctx.accounts.student.total_tasks = total_tasks;
    ctx.accounts.student.task_duration = task_duration;
    ctx.accounts.student.current_task_start_ts = now;
    ctx.accounts.student.current_grades = mentors
        .into_iter()
        .map(|mentor| Grade {
            mentor,
            set_grade: None,
            max_grade: 0,
        })
        .collect();

    emit!(AddStudentEvent {});

    Ok(())
}

#[event]
struct AddStudentEvent {}
