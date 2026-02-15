use anchor_lang::prelude::*;

#[error_code]
pub enum GovernanceError {
    #[msg("Governance instance already exists")]
    GovernanceAlreadyInitialized,

    #[msg("Voting period must be greater than zero")]
    InvalidVotingPeriod,

    #[msg("Voice credits per voter must be at least 1")]
    InvalidCreditAllocation,

    #[msg("Voter is already registered")]
    VoterAlreadyRegistered,

    #[msg("Must be a registered voter to create proposals")]
    NotRegisteredVoter,

    #[msg("Proposal title hash cannot be empty")]
    EmptyProposalTitle,

    #[msg("Proposal has already been finalized")]
    ProposalAlreadyFinalized,

    #[msg("Voting has not started yet")]
    VotingNotStarted,

    #[msg("Voting period has ended")]
    VotingPeriodEnded,

    #[msg("Must cast at least one vote")]
    ZeroVotes,

    #[msg("Insufficient voice credits for quadratic cost")]
    InsufficientCredits,

    #[msg("Voter has already voted on this proposal")]
    AlreadyVoted,

    #[msg("Arithmetic overflow in vote cost calculation")]
    ArithmeticOverflow,

    #[msg("Voting period has not ended yet")]
    VotingPeriodNotEnded,

    #[msg("Unauthorized: signer mismatch")]
    Unauthorized,

    #[msg("Description URI too long (max 256 bytes)")]
    DescriptionUriTooLong,

    #[msg("Invalid vote type (must be 0 or 1)")]
    InvalidVoteType,

    #[msg("Token account must be for the governance mint")]
    InvalidGovernanceMint,
}