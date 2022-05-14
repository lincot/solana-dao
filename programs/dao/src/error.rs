use anchor_lang::prelude::*;

#[error_code]
pub enum DaoError {
    /// 6000 0x1770
    #[msg("Overflow")]
    Overflow,
    /// 6001 0x1771
    #[msg("0 is not allowed as a value")]
    Zero,
    /// 6002 0x1772
    #[msg("Mentor is not assigned to user")]
    NotMentorOfStudent,
    /// 6003 0x1773
    #[msg("Grade cannot be higher than mentor's power")]
    NotEnoughPower,
    /// 6004 0x1774
    #[msg("Task duration has not yet elapsed")]
    TaskTimelock,
    /// 6005 0x1775
    #[msg("Not all mentors have set a grade yet")]
    NotAllMentorsHaveVoted,
}
