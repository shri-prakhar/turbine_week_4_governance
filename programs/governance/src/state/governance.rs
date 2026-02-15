use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Governance {
    pub admin: Pubkey,
    pub voice_credits_per_voter: u64,
    pub voting_period: i64,
    pub proposal_count: u64,
    pub bump: u8,
}

impl Governance {
    pub const SEED_PREFIX: &'static [u8] = b"governance";

    pub fn next_proposal_id(&mut self) -> u64 {
        let id = self.proposal_count;
        self.proposal_count = self.proposal_count.saturating_add(1);
        id
    }
}
