use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct VoterRecord {
    pub governance: Pubkey,
    pub voter: Pubkey,
    pub proposals_voted_on: u64,
    pub bump: u8,
}

impl VoterRecord {
    pub const SEED_PREFIX: &'static [u8] = b"voter";
}
