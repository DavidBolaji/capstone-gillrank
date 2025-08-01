import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { CapstoneGillrank } from "../target/types/capstone_gillrank";
import { assert } from "chai";

describe("capstone-gillrank", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.CapstoneGillrank as Program<CapstoneGillrank>;
  const user = provider.wallet;
  const programId = anchor.web3.SystemProgram.programId;


  it("creates user profile", async () => {

    const [userProfilePda, bump] = anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from("profile"),
        user.publicKey.toBuffer(),
      ],
      // programId
      program.programId
    )
    // create profile
    const tx = await program.methods.createUserProfile().accountsPartial({
      user: user.publicKey,
      userProfile: userProfilePda,
      systemProgram: programId,
    }).rpc()

    const userProfile = await program.account.userProfile.fetch(userProfilePda);
    
     // Check 1: Account is created
    assert.ok(userProfile);

    // Check 2: Owner is correctly set
    assert.strictEqual(userProfile.owner.toBase58(), user.publicKey.toBase58());

    // Check 3: Initial values
    assert.strictEqual(userProfile.wins, 0);
    assert.strictEqual(userProfile.loses, 0);
    assert.strictEqual(userProfile.nfts, 0);

    // Check 4: Bump
    assert.strictEqual(userProfile.bump, bump);
    console.log("Your transaction signature", tx);
  });
});
