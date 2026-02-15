use anchor_lang::prelude::*;
use crate::error::GovernanceError;
use crate::state::Governance;

#[derive(Accounts)]
pub struct InitializeGovernance<'info> {
    #[account(
        init,
        payer = admin,
        space = 8 + Governance::INIT_SPACE,
        seeds = [Governance::SEED_PREFIX, admin.key().as_ref()],
        bump,
    )]
    pub governance: Account<'info, Governance>,

    #[account(mut)]
    pub admin: Signer<'info>,

    pub system_program: Program<'info, System>,
}

pub fn handler(
    ctx: Context<InitializeGovernance>,
    voice_credits_per_voter: u64,
    voting_period: i64,
) -> Result<()> {
    require!(
        voice_credits_per_voter > 0,
        GovernanceError::InvalidCreditAllocation
    );
    require!(voting_period > 0, GovernanceError::InvalidVotingPeriod);

    let governance = &mut ctx.accounts.governance;
    governance.admin = ctx.accounts.admin.key();
    governance.voice_credits_per_voter = voice_credits_per_voter;
    governance.voting_period = voting_period;
    governance.proposal_count = 0;
    governance.bump = ctx.bumps.governance;

    msg!(
        "Governance initialized: {} credits/voter, {}s voting period",
        voice_credits_per_voter,
        voting_period
    );

    Ok(())
}
