use crate::{error::*, state::*, utils::*};
use anchor_lang::prelude::*;
use company;

const MAX_SALARY: u64 = 10000;

#[derive(Accounts)]
pub struct EndTask<'info> {
    #[account(seeds = [b"academy"], bump = academy.bump)]
    academy: Account<'info, Academy>,
    #[account(address = academy.authority)]
    academy_authority: Signer<'info>,
    #[account(mut, seeds = [b"student", student_authority.key().as_ref()], bump = student.bump)]
    student: Account<'info, Student>,
    #[account(mut)]
    student_authority: Signer<'info>,

    company: Account<'info, company::state::Company>,
    /// CHECK:
    employee: UncheckedAccount<'info>,
    company_program: Program<'info, company::program::Company>,

    system_program: Program<'info, System>,
}

pub fn end_task(ctx: Context<EndTask>) -> Result<()> {
    let now = Clock::get()?.unix_timestamp as u32;

    if ctx.accounts.student.current_task_start_ts + ctx.accounts.student.task_duration > now {
        return err!(AcademyError::TaskTimelock);
    }

    let mut current_set_grades_sum = 0;
    let mut current_max_grades_sum = 0;

    for &grade in &ctx.accounts.student.current_grades {
        current_set_grades_sum += grade
            .set_grade
            .ok_or(AcademyError::NotAllMentorsHaveVoted)?;
        current_max_grades_sum += grade.max_grade;
    }

    ctx.accounts.student.set_grades_sum += current_set_grades_sum;
    ctx.accounts.student.max_grades_sum += current_max_grades_sum;

    if current_set_grades_sum < current_max_grades_sum * 2 / 3 {
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
    let cpi_ctx = CpiContext::new(
        ctx.accounts.company_program.to_account_info(),
        company::cpi::accounts::Employ {
            company: ctx.accounts.company.to_account_info(),
            company_authority: ctx.accounts.academy_authority.to_account_info(),
            employee: ctx.accounts.employee.to_account_info(),
            employee_authority: ctx.accounts.student_authority.to_account_info(),
            system_program: ctx.accounts.system_program.to_account_info(),
        },
    );
    company::cpi::employ(
        cpi_ctx,
        MAX_SALARY * ctx.accounts.student.set_grades_sum / ctx.accounts.student.max_grades_sum,
    )?;

    close(
        ctx.accounts.student.to_account_info(),
        ctx.accounts.student_authority.to_account_info(),
    )?;

    emit!(GraduateStudentEvent {});

    Ok(())
}

#[event]
struct GraduateStudentEvent {}
