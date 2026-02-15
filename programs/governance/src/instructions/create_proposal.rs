use anchor_lang::prelude::*;
use crate::error::GovernanceError;
use crate::state::{Governance, Proposal, VoterRecord, MAX_DESCRIPTION_URI_LEN};

#[derive(Accounts)]
#[instruction(title_hash: [u8; 32], description_uri: String)]
pub struct CreateProposal<'info> {
    #[account(
        mut,
        seeds = [Governance::SEED_PREFIX, governance.admin.as_ref()],
        bump = governance.bump,
    )]
    pub governance: Account<'info, Governance>,

    #[account(
        seeds = [
            VoterRecord::SEED_PREFIX,
            governance.key().as_ref(),
            proposer.key().as_ref(),
        ],
        bump = voter_record.bump,
    )]
    pub voter_record: Account<'info, VoterRecord>,

    #[account(
        init,
        payer = proposer,
        space = 8 + Proposal::INIT_SPACE,
        seeds = [
            Proposal::SEED_PREFIX,
            governance.key().as_ref(),
            governance.proposal_count.to_le_bytes().as_ref(),
        ],
        bump,
    )]
    pub proposal: Account<'info, Proposal>,

    #[account(mut)]
    pub proposer: Signer<'info>,

    pub system_program: Program<'info, System>,
}

pub fn handler(
    ctx: Context<CreateProposal>,
    title_hash: [u8; 32],
    description_uri: String,
) -> Result<()> {
    require!(
        title_hash != [0u8; 32],
        GovernanceError::EmptyProposalTitle
    );
    require!(
        description_uri.len() <= MAX_DESCRIPTION_URI_LEN,
        GovernanceError::DescriptionUriTooLong
    );

    let clock = Clock::get()?;
    let now = clock.unix_timestamp;

    let governance = &mut ctx.accounts.governance;
    let proposal_id = governance.next_proposal_id();

    let proposal = &mut ctx.accounts.proposal;
    proposal.governance = governance.key();
    proposal.proposal_id = proposal_id;
    proposal.proposer = ctx.accounts.proposer.key();
    proposal.title_hash = title_hash;
    proposal.description_uri = description_uri;
    proposal.yes_votes = 0;
    proposal.no_votes = 0;
    proposal.start_ts = now;
    proposal.end_ts = now
        .checked_add(governance.voting_period)
        .ok_or(GovernanceError::ArithmeticOverflow)?;
    proposal.finalized = false;
    proposal.bump = ctx.bumps.proposal;

    msg!(
        "Proposal {} created by {}, voting ends at {}",
        proposal_id,
        ctx.accounts.proposer.key(),
        proposal.end_ts
    );

    Ok(())
}
