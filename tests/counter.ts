import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Counter } from "../target/types/counter";
import { BN } from "bn.js";
import { assert } from "chai";

describe("counter", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.Counter as Program<Counter>;

  it("Is initialized!", async () => {

    const number = new BN(23); // Initialize the counter with 23.
    const counterKp = new anchor.web3.Keypair(); // Create a new Keypair for the counter.

    const tx = await program.methods.createCounter(number).accounts({
      counter: counterKp.publicKey, // Assign the counter's Keypair to the counter account.
      authority: provider.wallet.publicKey, // Assign the provider's wallet to the authority account.
      systemProgram: anchor.web3.SystemProgram.programId, // Assign the System Program ID.
    })
    .signers([counterKp]) // Sign the transaction with the counter's Keypair.
    .rpc();
    console.log("Your transaction signature", tx);

    const counter = await program.account.counter.fetch(counterKp.publicKey);
    console.log("Counter count is: ", counter.number.toNumber());
    assert.ok(number.eq(counter.number)); // Check if the counter is initialized with 23.

  });
});
