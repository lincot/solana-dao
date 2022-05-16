use crate::{error::*, state::*, utils::close};
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct ExpelStudent<'info> {
    #[account(seeds = [b"academy"], bump = academy.bump)]
    academy: Account<'info, Academy>,
    #[account(address = academy.authority)]
    academy_authority: Signer<'info>,
    mentor: Signer<'info>,
    #[account(mut, seeds = [b"student", student_authority.key().as_ref()], bump = student.bump)]
    student: Account<'info, Student>,
    /// CHECK:
    #[account(mut)]
    student_authority: UncheckedAccount<'info>,
}

pub fn expel_student(ctx: Context<ExpelStudent>) -> Result<()> {
    let mentor_key = ctx.accounts.mentor.key();
    (ctx.accounts.student.current_grades)
        .iter()
        .find(|grade| grade.mentor == mentor_key)
        .ok_or(AcademyError::NotMentorOfStudent)?;

    close(
        ctx.accounts.student.to_account_info(),
        ctx.accounts.student_authority.to_account_info(),
    )?;

    emit!(ExpelStudentEvent {});

    Ok(())
}

#[event]
struct ExpelStudentEvent {}
