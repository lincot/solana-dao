use anchor_lang::prelude::*;

#[account]
pub struct Company {
    pub bump: u8,
    pub authority: Pubkey,
}
impl Company {
    pub const LEN: usize = 1 + 32;
}

#[account]
pub struct Employee {
    pub bump: u8,
    pub salary: u64,
}
impl Employee {
    pub const LEN: usize = 1 + 8;
}
