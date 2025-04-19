import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Day2 } from "../target/types/day2";

describe("day2", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.day2 as Program<Day2>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize("hello").rpc();
    console.log("Your transaction signature", tx);
  });

  it("array test", async () => {
    const tx = await program.methods.array([new anchor.BN(777), new anchor.BN(888)]).rpc();
    console.log("Your transaction signature", tx);
  });
  // @note: this test is not working
  // it("overflow test", async () => {
  //   const tx = await program.methods.overflow(new anchor.BN(0), new anchor.BN(1)).rpc();
  //   console.log("Your transaction signature", tx);
  // });
  it("power test", async () => {
    const tx = await program.methods.power(new anchor.BN(2), new anchor.BN(4)).rpc();
    console.log("Your transaction signature", tx);
  });
  it("crbt-cube root test", async () => {
    const tx = await program.methods.cube(new anchor.BN(50)).rpc();
    console.log("Your transaction signature", tx);
  });
});
