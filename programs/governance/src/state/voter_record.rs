use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct VoterRecord {
    pub governance: Pubkey,
    pub voter: Pubkey,
    pub credits_remaining: u64,
    pub credits_spent: u64,
    pub proposals_voted_on: u64,
    pub bump: u8,
}

impl VoterRecord {
    pub const SEED_PREFIX: &'static [u8] = b"voter";

    pub fn try_spend_credits(&mut self, cost: u64) -> bool {
        if let Some(remaining) = self.credits_remaining.checked_sub(cost) {
            self.credits_remaining = remaining;
            self.credits_spent = self.credits_spent.saturating_add(cost);
            self.proposals_voted_on = self.proposals_voted_on.saturating_add(1);
            true
        } else {
            false
        }
    }
}
