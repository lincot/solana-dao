use anchor_lang::prelude::*;

#[account]
pub struct Academy {
    pub bump: u8,
    pub authority: Pubkey,
    pub mntr_mint: Pubkey,
}
impl Academy {
    pub const LEN: usize = 1 + 32 + 32;
}

#[account]
pub struct Mentor {
    pub bump: u8,
}
impl Mentor {
    pub const LEN: usize = 1;
}

#[derive(AnchorSerialize, AnchorDeserialize, Copy, Clone, Debug)]
pub struct Grade {
    pub mentor: Pubkey,
    pub set_grade: Option<u64>,
    pub max_grade: u64,
}
impl Grade {
    pub const LEN: usize = 32 + 9 + 8;
}

#[account]
pub struct Student {
    pub bump: u8,
    pub total_tasks: u8,
    pub completed_tasks: u8,
    pub task_duration: u32,
    pub current_task_start_ts: u32,
    pub current_grades: Vec<Grade>,
}
impl Student {
    pub const LEN: usize = 1 + 1 + 1 + 4 + 4 + 4;
}
