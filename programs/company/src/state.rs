use anchor_lang::prelude::*;

#[account]
pub struct Company {
    pub bump: u8,
    pub authority: Pubkey,
}
impl Company {
    pub const LEN: usize = 1 + 32;
}
