use crate::{error::*, state::*, utils::*};
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct EndTask<'info> {
    #[account(seeds = [b"academy"], bump = academy.bump)]
    academy: Account<'info, Academy>,
    #[account(address = academy.authority)]
    academy_authority: Signer<'info>,
    #[account(mut, seeds = [b"student", student_authority.key().as_ref()], bump = student.bump)]
    student: Account<'info, Student>,
    /// CHECK:
    #[account(mut)]
    student_authority: UncheckedAccount<'info>,
}

pub fn end_task(ctx: Context<EndTask>) -> Result<()> {
    let now = Clock::get()?.unix_timestamp as u32;

    if ctx.accounts.student.current_task_start_ts + ctx.accounts.student.task_duration > now {
        return err!(AcademyError::TaskTimelock);
    }

    let mut set_grades_sum = 0;
    let mut max_grades_sum = 0;

    for &grade in &ctx.accounts.student.current_grades {
        set_grades_sum += grade
            .set_grade
            .ok_or(AcademyError::NotAllMentorsHaveVoted)?;
        max_grades_sum += grade.max_grade;
    }

    if set_grades_sum < max_grades_sum * 2 / 3 {
        dismiss_student(&ctx)?;
    } else {
        ctx.accounts.student.completed_tasks += 1;
        if ctx.accounts.student.completed_tasks == ctx.accounts.student.total_tasks {
            graduate_student(&ctx)?;
        }
    }

    emit!(EndTaskEvent {});

    Ok(())
}

#[event]
struct EndTaskEvent {}

fn dismiss_student(ctx: &Context<EndTask>) -> Result<()> {
    close(
        ctx.accounts.student.to_account_info(),
        ctx.accounts.student_authority.to_account_info(),
    )?;

    emit!(DismissStudentEvent {});

    Ok(())
}

#[event]
struct DismissStudentEvent {}

fn graduate_student(ctx: &Context<EndTask>) -> Result<()> {
    close(
        ctx.accounts.student.to_account_info(),
        ctx.accounts.student_authority.to_account_info(),
    )?;

    emit!(GraduateStudentEvent {});

    Ok(())
}

#[event]
struct GraduateStudentEvent {}
