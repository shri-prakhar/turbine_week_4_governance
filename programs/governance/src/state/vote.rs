use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Vote {
    pub authority: Pubkey,
    pub vote_type: u8,
    pub voting_credits: u64,
    pub bump: u8,
}

impl Vote {
    pub const SEED_PREFIX: &'static [u8] = b"vote";
}
