import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { PublicKey, SystemProgram, Connection} from '@solana/web3.js';
import { LotteryTicketsAnchor } from "../target/types/lottery_tickets_anchor";

describe("lottery-tickets-anchor", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const wallet = provider.wallet;

  const program = anchor.workspace.LotteryTicketsAnchor as Program<LotteryTicketsAnchor>;

  // Setup
  const numbersArray = [10, 8, 3, 4, 5, 6]; // Use a regular array of numbers
  const numbers = new Uint8Array(numbersArray); // Convert to Uint8Array if needed

  // Derive the PDA for your ticket account using the seed and program ID
  const [ticketPda, bump] = PublicKey.findProgramAddressSync(
    [Buffer.from(numbers)],
    program.programId
  );
  const tokenAddress = anchor.utils.token.associatedAddress({
    mint: ticketPda,
    owner: wallet.publicKey
  });


  it("Is initialized!", async () => {
    // Add your test here
    const tx = await program.methods.mintTicket(numbersArray, bump)
      .accounts({
        mintAccount: ticketPda,
        payer: wallet.publicKey,
        tokenAccount: tokenAddress,
      })
      .rpc();
    console.log("Your transaction signature", tx);
  });
});
