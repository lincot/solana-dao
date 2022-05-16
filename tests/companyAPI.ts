import { SystemProgram } from "@solana/web3.js";
import { Context } from "./ctx";

export async function initializeCompany(ctx: Context): Promise<void> {
  await ctx.companyProgram.methods
    .initialize()
    .accounts({
      company: ctx.company,
      companyAuthority: ctx.companyAuthority.publicKey,
      systemProgram: SystemProgram.programId,
    })
    .signers([ctx.companyAuthority])
    .rpc();
}
