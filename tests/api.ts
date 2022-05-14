import { BN } from "@project-serum/anchor";
import {
  Keypair,
  SystemProgram,
  SYSVAR_RENT_PUBKEY,
  PublicKey,
} from "@solana/web3.js";
import { TOKEN_PROGRAM_ID } from "@solana/spl-token";
import { Context } from "./ctx";

export async function initialize(ctx: Context): Promise<void> {
  await ctx.program.methods
    .initialize()
    .accounts({
      dao: ctx.dao,
      daoAuthority: ctx.daoAuthority.publicKey,
      mntrMint: ctx.mntrMint,
      rent: SYSVAR_RENT_PUBKEY,
      tokenProgram: TOKEN_PROGRAM_ID,
      systemProgram: SystemProgram.programId,
    })
    .signers([ctx.daoAuthority])
    .rpc();
}

export async function registerMentor(
  ctx: Context,
  mentorAuthority: PublicKey,
  power: number | BN
): Promise<void> {
  await ctx.program.methods
    .registerMentor(mentorAuthority, new BN(power))
    .accounts({
      dao: ctx.dao,
      daoAuthority: ctx.daoAuthority.publicKey,
      mntrMint: ctx.mntrMint,
      mentor: await ctx.mentor(mentorAuthority),
      mentorMntr: await ctx.mntrATA(await ctx.mentor(mentorAuthority)),
      tokenProgram: TOKEN_PROGRAM_ID,
      systemProgram: SystemProgram.programId,
    })
    .signers([ctx.daoAuthority])
    .rpc();
}

export async function registerStudent(
  ctx: Context,
  studentAuthority: Keypair,
  mentors: PublicKey[],
  totalTasks: number,
  taskDuration: number
): Promise<void> {
  await ctx.program.methods
    .registerStudent(mentors, totalTasks, taskDuration)
    .accounts({
      dao: ctx.dao,
      daoAuthority: ctx.daoAuthority.publicKey,
      student: await ctx.student(studentAuthority.publicKey),
      studentAuthority: studentAuthority.publicKey,
      systemProgram: SystemProgram.programId,
    })
    .signers([ctx.daoAuthority, studentAuthority])
    .rpc();
}

export async function setGrade(
  ctx: Context,
  mentorAuthority: Keypair,
  studentAuthority: PublicKey,
  grade: number | BN
): Promise<void> {
  await ctx.program.methods
    .setGrade(studentAuthority, new BN(grade))
    .accounts({
      dao: ctx.dao,
      mntrMint: ctx.mntrMint,
      mentor: await ctx.mentor(mentorAuthority.publicKey),
      mentorAuthority: mentorAuthority.publicKey,
      mentorMntr: await ctx.mntrATA(
        await ctx.mentor(mentorAuthority.publicKey)
      ),
      student: await ctx.student(studentAuthority),
    })
    .signers([mentorAuthority])
    .rpc();
}

export async function endTask(
  ctx: Context,
  studentAuthority: PublicKey
): Promise<void> {
  await ctx.program.methods
    .endTask()
    .accounts({
      dao: ctx.dao,
      daoAuthority: ctx.daoAuthority.publicKey,
      student: await ctx.student(studentAuthority),
      studentAuthority,
    })
    .signers([ctx.daoAuthority])
    .rpc();
}
