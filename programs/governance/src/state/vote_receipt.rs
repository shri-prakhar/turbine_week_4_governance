use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct VoteReceipt {
    pub proposal: Pubkey,
    pub voter: Pubkey,
    pub num_votes: u64,
    pub credits_spent: u64,
    pub vote_for: bool,
    pub voted_at: i64,
    pub bump: u8,
}

impl VoteReceipt {
    pub const SEED_PREFIX: &'static [u8] = b"receipt";
}
