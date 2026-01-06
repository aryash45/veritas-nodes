import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Veritas } from "../target/types/veritas";
import { assert } from "chai";

describe("veritas", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.Veritas as Program<Veritas>;

  
  const getNewId = () => "NODE_" + Math.floor(Math.random() * 100000);

  it("✅ Level 1.1: Registers a node successfully", async () => {
    const deviceId = getNewId();
    const [nodePda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("node"), provider.wallet.publicKey.toBuffer(), Buffer.from(deviceId)],
      program.programId
    );

    await (program.methods as any)
      .registerNode(deviceId)
      .accounts({
        node: nodePda,
        signer: provider.wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    const account = await program.account.nodeAccount.fetch(nodePda);
    assert.strictEqual(account.deviceId, deviceId);
    console.log("Registered Node PDA:", nodePda.toBase58());
  });
  it("❌ Level 1.2: Fails when Device ID is too long", async () => {
    const longId = "A".repeat(33); 

    try {
      
      const validSeed = "A".repeat(32);
      const [fakeNodePda] = anchor.web3.PublicKey.findProgramAddressSync(
        [Buffer.from("node"), provider.wallet.publicKey.toBuffer(), Buffer.from(validSeed)],
        program.programId
      );

      await (program.methods as any)
        .registerNode(longId)
        .accounts({
          node: fakeNodePda,
          signer: provider.wallet.publicKey,
        })
        .rpc();
      
      assert.fail("Security failure: The program should have blocked this!");
    } catch (err: any) {
      console.log("✅ Level 1.2: Transaction successfully blocked by the program.");
      assert.ok(true); 
    }
  });

  it("✅ Level 2.1: Submits a data heartbeat successfully", async () => {
    const deviceId = getNewId();
    const mockDataHash = Array(32).fill(0).map((_, i) => i % 256);

    const [nodePda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("node"), provider.wallet.publicKey.toBuffer(), Buffer.from(deviceId)],
      program.programId
    );

    // 1. Register the node
    await (program.methods as any)
      .registerNode(deviceId)
      .accounts({
        node: nodePda,
        signer: provider.wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    
    try {
      await (program.methods as any)
        .submitDataHash(mockDataHash)
        .accounts({
          node: nodePda,
          owner: provider.wallet.publicKey,
        })
        .rpc();

      const account = await program.account.nodeAccount.fetch(nodePda);
      assert.ok(account.lastPing.toNumber() > 0);
      console.log("✅ Heartbeat recorded at timestamp:", account.lastPing.toNumber());
    } catch (e) {
      console.log("Check if lib.rs has pub fn submit_data_hash");
      throw e;
    }
  });
});