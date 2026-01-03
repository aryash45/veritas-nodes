import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Veritas } from "../target/types/veritas";
import { assert } from "chai";

describe("veritas", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.Veritas as Program<Veritas>;

  it("✅ Level 1.1: Registers a node successfully", async () => {
    const deviceId = "NODE_001";
    
    // We derive the PDA for the assertion later
    const [nodePda] = anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from("node"),
        provider.wallet.publicKey.toBuffer(),
        Buffer.from(deviceId),
      ],
      program.programId
    );

    // SENIOR TIP: Use accountsPartial if you want to explicitly pass the PDA
    // even if the IDL knows how to resolve it.
    await program.methods
      .registerNode(deviceId)
      .accountsPartial({
        node: nodePda,
        signer: provider.wallet.publicKey,
      })
      .rpc();

    const account = await program.account.node.fetch(nodePda);
    assert.strictEqual(account.deviceId, deviceId);
    assert.ok(account.isActive);
    console.log("Registered Node PDA:", nodePda.toBase58());
  });

  it("❌ Level 1.2: Fails when Device ID is too long", async () => {
    const longId = "THIS_ID_IS_EXACTLY_33_CHARS_LONG_"; 

    try {
      await program.methods
        .registerNode(longId)
        .accountsPartial({
          signer: provider.wallet.publicKey,
        })
        .rpc();
      
      assert.fail("The program should have failed but it succeeded!");
    } catch (err: any) {
      // In Anchor 0.30+, the error object structure has been refined.
      // We check for our custom error 'IdTooLong'
      const errorMessage = err.toString();
      assert.include(errorMessage, "IdTooLong", "Error should be IdTooLong");
      console.log("Successfully caught expected security error.");
    }
  });
});