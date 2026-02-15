use anchor_lang::prelude::*;
use crate::error::GovernanceError;
use crate::state::{Governance, Proposal};

#[derive(Accounts)]
pub struct FinalizeProposal<'info> {
    #[account(
        seeds = [Governance::SEED_PREFIX, governance.admin.as_ref()],
        bump = governance.bump,
    )]
    pub governance: Account<'info, Governance>,

    #[account(
        mut,
        seeds = [
            Proposal::SEED_PREFIX,
            governance.key().as_ref(),
            proposal.proposal_id.to_le_bytes().as_ref(),
        ],
        bump = proposal.bump,
        constraint = proposal.governance == governance.key() @ GovernanceError::Unauthorized,
    )]
    pub proposal: Account<'info, Proposal>,

    pub caller: Signer<'info>,
}

pub fn handler(ctx: Context<FinalizeProposal>) -> Result<()> {
    let proposal = &mut ctx.accounts.proposal;

    require!(!proposal.finalized, GovernanceError::ProposalAlreadyFinalized);

    let clock = Clock::get()?;
    require!(
        proposal.is_voting_ended(clock.unix_timestamp),
        GovernanceError::VotingPeriodNotEnded
    );

    proposal.finalized = true;

    let net = proposal.net_votes();
    let result = if net > 0 {
        "PASSED"
    } else if net < 0 {
        "REJECTED"
    } else {
        "TIED"
    };

    msg!(
        "Proposal {} finalized: {} (yes: {}, no: {}, net: {})",
        proposal.proposal_id,
        result,
        proposal.yes_votes,
        proposal.no_votes,
        net
    );

    Ok(())
}
