use anchor_lang::prelude::*;

pub const MAX_DESCRIPTION_URI_LEN: usize = 256;

#[account]
#[derive(InitSpace)]
pub struct Proposal {
    pub governance: Pubkey,
    pub proposal_id: u64,
    pub proposer: Pubkey,
    pub title_hash: [u8; 32],
    #[max_len(MAX_DESCRIPTION_URI_LEN)]
    pub description_uri: String,
    pub yes_votes: u64,
    pub no_votes: u64,
    pub start_ts: i64,
    pub end_ts: i64,
    pub finalized: bool,
    pub bump: u8,
}

impl Proposal {
    pub const SEED_PREFIX: &'static [u8] = b"proposal";

    pub fn is_voting_open(&self, current_ts: i64) -> bool {
        current_ts >= self.start_ts && current_ts < self.end_ts && !self.finalized
    }

    pub fn is_voting_ended(&self, current_ts: i64) -> bool {
        current_ts >= self.end_ts
    }

    pub fn net_votes(&self) -> i128 {
        self.yes_votes as i128 - self.no_votes as i128
    }
}
