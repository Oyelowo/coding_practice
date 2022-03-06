import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Genesis } from "../target/types/genesis";

describe("genesis", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.Provider.env());

  const program = anchor.workspace.Genesis as Program<Genesis>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.rpc.initialize({});
    console.log("Your transaction signature", tx);
  });
});
