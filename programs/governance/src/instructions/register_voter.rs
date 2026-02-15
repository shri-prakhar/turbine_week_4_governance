use anchor_lang::prelude::*;
use crate::state::{Governance, VoterRecord};

#[derive(Accounts)]
pub struct RegisterVoter<'info> {
    #[account(
        seeds = [Governance::SEED_PREFIX, governance.admin.as_ref()],
        bump = governance.bump,
    )]
    pub governance: Account<'info, Governance>,

    #[account(
        init,
        payer = voter,
        space = 8 + VoterRecord::INIT_SPACE,
        seeds = [
            VoterRecord::SEED_PREFIX,
            governance.key().as_ref(),
            voter.key().as_ref(),
        ],
        bump,
    )]
    pub voter_record: Account<'info, VoterRecord>,

    #[account(mut)]
    pub voter: Signer<'info>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<RegisterVoter>) -> Result<()> {
    let governance = &ctx.accounts.governance;
    let voter_record = &mut ctx.accounts.voter_record;
    let credits = governance.voice_credits_per_voter;

    voter_record.governance = governance.key();
    voter_record.voter = ctx.accounts.voter.key();
    voter_record.credits_remaining = credits;
    voter_record.credits_spent = 0;
    voter_record.proposals_voted_on = 0;
    voter_record.bump = ctx.bumps.voter_record;

    msg!("Voter registered with {} credits", credits);
    Ok(())
}
