use crate::{error::*, state::*, utils::close};
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct ExpelStudent<'info> {
    #[account(seeds = [b"dao"], bump = dao.bump)]
    dao: Account<'info, Dao>,
    #[account(address = dao.authority)]
    dao_authority: Signer<'info>,
    mentor_authority: Signer<'info>,
    #[account(mut, seeds = [b"student", student_authority.key().as_ref()], bump = student.bump)]
    student: Account<'info, Student>,
    /// CHECK:
    #[account(mut)]
    student_authority: UncheckedAccount<'info>,
}

pub fn expel_student(ctx: Context<ExpelStudent>) -> Result<()> {
    let mentor_key = ctx.accounts.mentor_authority.key();
    (ctx.accounts.student.current_grades)
        .iter()
        .find(|grade| grade.mentor == mentor_key)
        .ok_or(DaoError::NotMentorOfStudent)?;

    close(
        ctx.accounts.student.to_account_info(),
        ctx.accounts.student_authority.to_account_info(),
    )?;

    emit!(ExpelStudentEvent {});

    Ok(())
}

#[event]
struct ExpelStudentEvent {}
