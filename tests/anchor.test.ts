import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { YieldRouter } from "../target/types/yield_router";

describe("yield-router", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.YieldRouter as Program<YieldRouter>;

  const [vaultState] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("state_v3")],
    program.programId
  );

  const [vaultPda] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("vault_pda")],
    program.programId
  );

  it("Executes Full Yield Cycle", async () => {
    // 1. Initialize
    try {
      await program.methods
        .initialize()
        .accounts({
          vault: vaultState,
          admin: provider.wallet.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .rpc();
      console.log("Vault Initialized");
    } catch (e) {
      console.log("Vault already initialized, continuing...");
    }

    // 2. Deposit (Note: CamelCase for vaultPda and systemProgram)
    const amount = new anchor.BN(0.02 * anchor.web3.LAMPORTS_PER_SOL);
    await program.methods
      .deposit(amount)
      .accounts({
        vault: vaultState,
        vaultPda: vaultPda,
        user: provider.wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();
    console.log("Deposit Successful");

    // 3. Stake
    await program.methods
      .stake()
      .accounts({
        vault: vaultState,
        admin: provider.wallet.publicKey,
      })
      .rpc();

    // 4. Verify
    const state = await program.account.vaultAccount.fetch(vaultState);
    if (state.isStaked) {
      console.log("SUCCESS: Program live and verified!");
    }
  });
});
