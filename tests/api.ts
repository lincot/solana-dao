import { BN } from "@project-serum/anchor";
import { Keypair, SystemProgram, PublicKey } from "@solana/web3.js";
import { Context } from "./ctx";

export async function initialize(ctx: Context): Promise<void> {
  await ctx.program.methods
    .initialize(ctx.mntrMint)
    .accounts({
      dao: ctx.dao,
      daoAuthority: ctx.daoAuthority.publicKey,
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
  mentor: Keypair,
  studentAuthority: PublicKey,
  grade: number | BN
): Promise<void> {
  await ctx.program.methods
    .setGrade(studentAuthority, new BN(grade))
    .accounts({
      dao: ctx.dao,
      mentor: mentor.publicKey,
      mentorMntr: await ctx.mntrATA(mentor.publicKey),
      student: await ctx.student(studentAuthority),
    })
    .signers([mentor])
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

export async function expelStudent(
  ctx: Context,
  mentor: Keypair,
  studentAuthority: PublicKey
): Promise<void> {
  await ctx.program.methods
    .expelStudent()
    .accounts({
      dao: ctx.dao,
      daoAuthority: ctx.daoAuthority.publicKey,
      mentor: mentor.publicKey,
      student: await ctx.student(studentAuthority),
      studentAuthority,
    })
    .signers([ctx.daoAuthority, mentor])
    .rpc();
}
