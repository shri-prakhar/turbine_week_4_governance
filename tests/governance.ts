import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Governance } from "../target/types/governance";
import { PublicKey } from "@solana/web3.js";
import { expect } from "chai";
import {
  createMint,
  createAssociatedTokenAccount,
  mintTo,
  getAssociatedTokenAddressSync,
  TOKEN_PROGRAM_ID,
} from "@solana/spl-token";

describe("governance", () => {
  anchor.setProvider(anchor.AnchorProvider.env());
  const program = anchor.workspace.Governance as Program<Governance>;
  const programId = program.programId;

  const admin = anchor.web3.Keypair.generate();
  const proposer = anchor.web3.Keypair.generate();
  const voter1 = anchor.web3.Keypair.generate();
  const voter2 = anchor.web3.Keypair.generate();

  const voiceCreditsPerVoter = new anchor.BN(100);
  const votingPeriodSec = new anchor.BN(2);

  let governancePda: PublicKey;
  let proposerVoterRecordPda: PublicKey;
  let voter1VoterRecordPda: PublicKey;
  let voter2VoterRecordPda: PublicKey;
  let adminVoterRecordPda: PublicKey;
  let proposalPda: PublicKey;
  let mint: PublicKey;
  let voter1Ata: PublicKey;
  let voter2Ata: PublicKey;
  let adminAta: PublicKey;

  const titleHash = Buffer.alloc(32);
  titleHash[0] = 1;
  const descriptionUri = "https://example.com/proposal";

  before(async () => {
    const provider = anchor.getProvider() as anchor.AnchorProvider;
    const sig = await provider.connection.requestAirdrop(
      admin.publicKey,
      10 * anchor.web3.LAMPORTS_PER_SOL
    );
    await provider.connection.confirmTransaction(sig);
    await provider.connection.requestAirdrop(
      proposer.publicKey,
      2 * anchor.web3.LAMPORTS_PER_SOL
    );
    await provider.connection.requestAirdrop(
      voter1.publicKey,
      2 * anchor.web3.LAMPORTS_PER_SOL
    );
    await provider.connection.requestAirdrop(
      voter2.publicKey,
      2 * anchor.web3.LAMPORTS_PER_SOL
    );
  });

  it("initializes governance", async () => {
    [governancePda] = PublicKey.findProgramAddressSync(
      [Buffer.from("governance"), admin.publicKey.toBuffer()],
      programId
    );

    await program.methods
      .initializeGovernance(voiceCreditsPerVoter, votingPeriodSec)
      .accounts({
        governance: governancePda,
        admin: admin.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([admin])
      .rpc();

    const gov = await program.account.governance.fetch(governancePda);
    expect(gov.admin.equals(admin.publicKey)).to.be.true;
    expect(gov.voiceCreditsPerVoter.toNumber()).to.equal(100);
    expect(gov.votingPeriod.toNumber()).to.equal(2);
    expect(gov.proposalCount.toNumber()).to.equal(0);
  });

  it("registers voters", async () => {
    [proposerVoterRecordPda] = PublicKey.findProgramAddressSync(
      [
        Buffer.from("voter"),
        governancePda.toBuffer(),
        proposer.publicKey.toBuffer(),
      ],
      programId
    );
    [voter1VoterRecordPda] = PublicKey.findProgramAddressSync(
      [
        Buffer.from("voter"),
        governancePda.toBuffer(),
        voter1.publicKey.toBuffer(),
      ],
      programId
    );
    [voter2VoterRecordPda] = PublicKey.findProgramAddressSync(
      [
        Buffer.from("voter"),
        governancePda.toBuffer(),
        voter2.publicKey.toBuffer(),
      ],
      programId
    );

    await program.methods
      .registerVoter()
      .accounts({
        governance: governancePda,
        voterRecord: proposerVoterRecordPda,
        voter: proposer.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([proposer])
      .rpc();

    await program.methods
      .registerVoter()
      .accounts({
        governance: governancePda,
        voterRecord: voter1VoterRecordPda,
        voter: voter1.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([voter1])
      .rpc();

    await program.methods
      .registerVoter()
      .accounts({
        governance: governancePda,
        voterRecord: voter2VoterRecordPda,
        voter: voter2.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([voter2])
      .rpc();

    await program.methods
      .registerVoter()
      .accounts({
        governance: governancePda,
        voterRecord: adminVoterRecordPda,
        voter: admin.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([admin])
      .rpc();

    const rec = await program.account.voterRecord.fetch(proposerVoterRecordPda);
    expect(rec.creditsRemaining.toNumber()).to.equal(100);
  });

  it("sets up SPL mint and voter token accounts", async () => {
    const provider = anchor.getProvider() as anchor.AnchorProvider;
    mint = await createMint(
      provider.connection,
      admin,
      admin.publicKey,
      null,
      6,
      undefined,
      undefined,
      TOKEN_PROGRAM_ID
    );
    voter1Ata = getAssociatedTokenAddressSync(
      mint,
      voter1.publicKey,
      false,
      TOKEN_PROGRAM_ID
    );
    voter2Ata = getAssociatedTokenAddressSync(
      mint,
      voter2.publicKey,
      false,
      TOKEN_PROGRAM_ID
    );
    await createAssociatedTokenAccount(
      provider.connection,
      admin,
      mint,
      voter1.publicKey,
      undefined,
      TOKEN_PROGRAM_ID
    );
    await createAssociatedTokenAccount(
      provider.connection,
      admin,
      mint,
      voter2.publicKey,
      undefined,
      TOKEN_PROGRAM_ID
    );
    adminAta = getAssociatedTokenAddressSync(
      mint,
      admin.publicKey,
      false,
      TOKEN_PROGRAM_ID
    );
    await createAssociatedTokenAccount(
      provider.connection,
      admin,
      mint,
      admin.publicKey,
      undefined,
      TOKEN_PROGRAM_ID
    );
    await mintTo(
      provider.connection,
      admin,
      mint,
      voter1Ata,
      admin,
      100 * 1e6,
      undefined,
      undefined,
      TOKEN_PROGRAM_ID
    );
    await mintTo(
      provider.connection,
      admin,
      mint,
      voter2Ata,
      admin,
      49 * 1e6,
      undefined,
      undefined,
      TOKEN_PROGRAM_ID
    );
    await mintTo(
      provider.connection,
      admin,
      mint,
      adminAta,
      admin,
      1 * 1e6,
      undefined,
      undefined,
      TOKEN_PROGRAM_ID
    );
  });

  it("creates proposal", async () => {
    const proposalIdBuf = Buffer.alloc(8);
    proposalIdBuf.writeBigUInt64LE(BigInt(0));
    [proposalPda] = PublicKey.findProgramAddressSync(
      [
        Buffer.from("proposal"),
        governancePda.toBuffer(),
        proposalIdBuf,
      ],
      programId
    );

    await program.methods
      .createProposal(Array.from(titleHash), descriptionUri)
      .accounts({
        governance: governancePda,
        voterRecord: proposerVoterRecordPda,
        proposal: proposalPda,
        proposer: proposer.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([proposer])
      .rpc();

    const prop = await program.account.proposal.fetch(proposalPda);
    expect(prop.proposalId.toNumber()).to.equal(0);
    expect(prop.yesVotes.toNumber()).to.equal(0);
    expect(prop.noVotes.toNumber()).to.equal(0);
    expect(prop.finalized).to.be.false;
  });

  it("casts vote: voter1 yes (1), voter2 no (0)", async () => {
    const [vote1Pda] = PublicKey.findProgramAddressSync(
      [
        Buffer.from("vote"),
        proposalPda.toBuffer(),
        voter1.publicKey.toBuffer(),
      ],
      programId
    );
    const [vote2Pda] = PublicKey.findProgramAddressSync(
      [
        Buffer.from("vote"),
        proposalPda.toBuffer(),
        voter2.publicKey.toBuffer(),
      ],
      programId
    );

    await program.methods
      .castVote(1)
      .accounts({
        governance: governancePda,
        proposalAccount: proposalPda,
        voterRecord: voter1VoterRecordPda,
        voteAccount: vote1Pda,
        voterTokenAccount: voter1Ata,
        voter: voter1.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([voter1])
      .rpc();

    await program.methods
      .castVote(0)
      .accounts({
        governance: governancePda,
        proposalAccount: proposalPda,
        voterRecord: voter2VoterRecordPda,
        voteAccount: vote2Pda,
        voterTokenAccount: voter2Ata,
        voter: voter2.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([voter2])
      .rpc();

    const prop = await program.account.proposal.fetch(proposalPda);
    const vote1 = await program.account.vote.fetch(vote1Pda);
    const vote2 = await program.account.vote.fetch(vote2Pda);

    expect(vote1.voteType).to.equal(1);
    expect(vote2.voteType).to.equal(0);
    expect(vote1.votingCredits.toNumber()).to.equal(10000);
    expect(vote2.votingCredits.toNumber()).to.equal(7000);
    expect(prop.yesVotes.toNumber()).to.equal(10000);
    expect(prop.noVotes.toNumber()).to.equal(7000);
  });

  it("rejects double vote from same voter", async () => {
    const [vote1Pda] = PublicKey.findProgramAddressSync(
      [
        Buffer.from("vote"),
        proposalPda.toBuffer(),
        voter1.publicKey.toBuffer(),
      ],
      programId
    );
    let err: unknown;
    try {
      await program.methods
        .castVote(0)
        .accounts({
          governance: governancePda,
          proposalAccount: proposalPda,
          voterRecord: voter1VoterRecordPda,
          voteAccount: vote1Pda,
          voterTokenAccount: voter1Ata,
          voter: voter1.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([voter1])
        .rpc();
    } catch (e) {
      err = e;
    }
    expect(err).to.exist;
  });

  it("rejects invalid vote type", async () => {
    const [voteDummyPda] = PublicKey.findProgramAddressSync(
      [
        Buffer.from("vote"),
        proposalPda.toBuffer(),
        admin.publicKey.toBuffer(),
      ],
      programId
    );
    let err: unknown;
    try {
      await program.methods
        .castVote(2)
        .accounts({
          governance: governancePda,
          proposalAccount: proposalPda,
          voterRecord: adminVoterRecordPda,
          voteAccount: voteDummyPda,
          voterTokenAccount: adminAta,
          voter: admin.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([admin])
        .rpc();
    } catch (e) {
      err = e;
    }
    expect(err).to.exist;
  });

  it("finalizes proposal after voting period", async () => {
    await new Promise((r) => setTimeout(r, 2500));

    await program.methods
      .finalizeProposal()
      .accounts({
        governance: governancePda,
        proposal: proposalPda,
        caller: admin.publicKey,
      })
      .signers([admin])
      .rpc();

    const prop = await program.account.proposal.fetch(proposalPda);
    expect(prop.finalized).to.be.true;
  });

  it("rejects finalize before voting period ends", async () => {
    const proposalIdBuf = Buffer.alloc(8);
    proposalIdBuf.writeBigUInt64LE(BigInt(1));
    const [proposal2Pda] = PublicKey.findProgramAddressSync(
      [
        Buffer.from("proposal"),
        governancePda.toBuffer(),
        proposalIdBuf,
      ],
      programId
    );

    await program.methods
      .createProposal(Array.from(titleHash), descriptionUri + "/2")
      .accounts({
        governance: governancePda,
        voterRecord: proposerVoterRecordPda,
        proposal: proposal2Pda,
        proposer: proposer.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([proposer])
      .rpc();

    let err: unknown;
    try {
      await program.methods
        .finalizeProposal()
        .accounts({
          governance: governancePda,
          proposal: proposal2Pda,
          caller: admin.publicKey,
        })
        .signers([admin])
        .rpc();
    } catch (e) {
      err = e;
    }
    expect(err).to.exist;
  });
});
