import { BN } from "@project-serum/anchor";
import { SystemProgram, PublicKey } from "@solana/web3.js";
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

export async function employ(
  ctx: Context,
  employeeAuthority: PublicKey,
  salary: number | BN
): Promise<void> {
  await ctx.companyProgram.methods
    .employ(employeeAuthority, new BN(salary))
    .accounts({
      company: ctx.company,
      companyAuthority: ctx.companyAuthority.publicKey,
      employee: await ctx.employee(employeeAuthority),
      systemProgram: SystemProgram.programId,
    })
    .signers([ctx.companyAuthority])
    .rpc();
}
