use crate::{error::*, state::*};
use anchor_lang::prelude::*;
use anchor_spl::token::TokenAccount;

#[derive(Accounts)]
#[instruction(student_authority: Pubkey)]
pub struct SetGrade<'info> {
    #[account(seeds = [b"dao"], bump = dao.bump)]
    dao: Account<'info, Dao>,
    mentor: Signer<'info>,
    #[account(associated_token::authority = mentor, associated_token::mint = dao.mntr_mint)]
    mentor_mntr: Account<'info, TokenAccount>,
    #[account(mut, seeds = [b"student", student_authority.as_ref()], bump = student.bump)]
    student: Account<'info, Student>,
}

pub fn set_grade(ctx: Context<SetGrade>, _student_authority: Pubkey, new_grade: u64) -> Result<()> {
    if new_grade > ctx.accounts.mentor_mntr.amount {
        return err!(DaoError::NotEnoughPower);
    }

    let mentor_key = ctx.accounts.mentor.key();
    let mut grade = (ctx.accounts.student.current_grades)
        .iter_mut()
        .find(|grade| grade.mentor == mentor_key)
        .ok_or(DaoError::NotMentorOfStudent)?;
    grade.set_grade = Some(new_grade);
    grade.max_grade = ctx.accounts.mentor_mntr.amount;

    emit!(SetGradeEvent {});

    Ok(())
}

#[event]
struct SetGradeEvent {}
