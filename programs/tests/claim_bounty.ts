import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { BountyBoard } from "../target/types/bounty_board";

describe("Claim Bounty", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  
  const program = anchor.workspace.BountyBoard as Program<BountyBoard>;

  it("Allows valid bounty claim", async () => {
    // Test setup code...
    
    await program.methods
      .claimBounty()
      .accounts({
        bounty: bountyPda,
        bountyBoard: boardPda,
        claimant: claimant.publicKey,
        claimantRewardAccount: claimantTokenAccount,
        escrowAccount: escrowPda,
      })
      .signers([claimant])
      .rpc();

    // Verification code...
  });
});