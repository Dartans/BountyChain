const handleClaimBounty = async (bountyPublicKey: PublicKey) => {
    if (!publicKey || !program) return;
  
    try {
      // Get bounty account data
      const bounty = await program.account.bounty.fetch(bountyPublicKey);
      
      // Get associated token accounts
      const [escrowAccount] = await PublicKey.findProgramAddress(
        [
          Buffer.from("bounty_board"),
          program.programId.toBuffer(),
          bounty.tokenMint.toBuffer(),
        ],
        program.programId
      );
  
      // Execute claim transaction
      const tx = await program.methods
        .claimBounty()
        .accounts({
          bounty: bountyPublicKey,
          bountyBoard: bounty.board,
          claimant: publicKey,
          claimantRewardAccount: getAssociatedTokenAddressSync(
            bounty.tokenMint,
            publicKey
          ),
          escrowAccount,
          tokenProgram: TOKEN_PROGRAM_ID,
        })
        .rpc();
  
      console.log("Claim transaction:", tx);
      notify.success("Bounty claimed successfully!");
    } catch (error) {
      console.error("Claim failed:", error);
      notify.error("Failed to claim bounty");
    }
  };