use anchor_lang::prelude::*;
pub mod error;
pub mod state;
pub mod instructions;

use instructions::*;

declare_id!("4WQyu78GSGe2Xkt7gf6uNe3P6hXRffGE2qRb1zhAgMVk");

#[program]
pub mod governance {
    use super::*;

    pub fn initialize_governance(
        ctx: Context<InitializeGovernance>,
        voice_credits_per_voter: u64,
        voting_period: i64,
    ) -> Result<()> {
        instructions::initialize_governance::handler(ctx, voice_credits_per_voter, voting_period)
    }

    pub fn register_voter(ctx: Context<RegisterVoter>) -> Result<()> {
        instructions::register_voter::handler(ctx)
    }

    pub fn create_proposal(
        ctx: Context<CreateProposal>,
        title_hash: [u8; 32],
        description_uri: String,
    ) -> Result<()> {
        instructions::create_proposal::handler(ctx, title_hash, description_uri)
    }

    pub fn cast_vote(ctx: Context<CastVoteContext>, vote_type: u8) -> Result<()> {
        instructions::cast_vote::handler(ctx, vote_type)
    }

    pub fn finalize_proposal(ctx: Context<FinalizeProposal>) -> Result<()> {
        instructions::finalize_proposal::handler(ctx)
    }
}
