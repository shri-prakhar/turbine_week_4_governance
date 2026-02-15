use anchor_lang::prelude::*;
use anchor_spl::token::TokenAccount;
use crate::error::GovernanceError;
use crate::state::{Governance, Proposal, Vote};

#[derive(Accounts)]
pub struct CastVoteContext<'info> {
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
            proposal_account.proposal_id.to_le_bytes().as_ref(),
        ],
        bump = proposal_account.bump,
        constraint = proposal_account.governance == governance.key() @ GovernanceError::Unauthorized,
    )]
    pub proposal_account: Account<'info, Proposal>,

    #[account(
        init,
        payer = voter,
        space = 8 + Vote::INIT_SPACE,
        seeds = [
            Vote::SEED_PREFIX,
            proposal_account.key().as_ref(),
            voter.key().as_ref(),
        ],
        bump,
    )]
    pub vote_account: Account<'info, Vote>,

    #[account(
        constraint = voter_token_account.owner == voter.key() @ GovernanceError::Unauthorized,
    )]
    pub voter_token_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub voter: Signer<'info>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<CastVoteContext>, vote_type: u8) -> Result<()> {
    require!(vote_type <= 1, GovernanceError::InvalidVoteType);

    require!(!ctx.accounts.proposal_account.finalized, GovernanceError::ProposalAlreadyFinalized);
    let clock = Clock::get()?;
    let now = clock.unix_timestamp;
    require!(now >= ctx.accounts.proposal_account.start_ts, GovernanceError::VotingNotStarted);
    require!(now < ctx.accounts.proposal_account.end_ts, GovernanceError::VotingPeriodEnded);

    let vote_account = &mut ctx.accounts.vote_account;
    let proposal_account = &mut ctx.accounts.proposal_account;
    let voting_credits: u64 = (ctx.accounts.voter_token_account.amount as f64).sqrt() as u64;

    vote_account.set_inner(Vote {
        authority: ctx.accounts.voter.key(),
        vote_type,
        voting_credits,
        bump: ctx.bumps.vote_account,
    });

    match vote_type {
        0 => proposal_account.no_votes = proposal_account.no_votes.saturating_add(voting_credits),
        _ => proposal_account.yes_votes = proposal_account.yes_votes.saturating_add(voting_credits),
    }

    Ok(())
}
