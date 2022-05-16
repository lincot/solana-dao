import { BN } from "@project-serum/anchor";
import { Keypair, SystemProgram, PublicKey } from "@solana/web3.js";
import { Context } from "./ctx";

export async function initializeAcademy(ctx: Context): Promise<void> {
  await ctx.academyProgram.methods
    .initialize(ctx.mntrMint)
    .accounts({
      academy: ctx.academy,
      academyAuthority: ctx.companyAuthority.publicKey,
      systemProgram: SystemProgram.programId,
    })
    .signers([ctx.companyAuthority])
    .rpc();
}

export async function registerStudent(
  ctx: Context,
  studentAuthority: Keypair,
  mentors: PublicKey[],
  totalTasks: number,
  taskDuration: number
): Promise<void> {
  await ctx.academyProgram.methods
    .registerStudent(mentors, totalTasks, taskDuration)
    .accounts({
      academy: ctx.academy,
      academyAuthority: ctx.companyAuthority.publicKey,
      student: await ctx.student(studentAuthority.publicKey),
      studentAuthority: studentAuthority.publicKey,
      systemProgram: SystemProgram.programId,
    })
    .signers([ctx.companyAuthority, studentAuthority])
    .rpc();
}

export async function setGrade(
  ctx: Context,
  mentor: Keypair,
  studentAuthority: PublicKey,
  grade: number | BN
): Promise<void> {
  await ctx.academyProgram.methods
    .setGrade(studentAuthority, new BN(grade))
    .accounts({
      academy: ctx.academy,
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
  await ctx.academyProgram.methods
    .endTask()
    .accounts({
      academy: ctx.academy,
      academyAuthority: ctx.companyAuthority.publicKey,
      student: await ctx.student(studentAuthority),
      studentAuthority,
    })
    .signers([ctx.companyAuthority])
    .rpc();
}

export async function expelStudent(
  ctx: Context,
  mentor: Keypair,
  studentAuthority: PublicKey
): Promise<void> {
  await ctx.academyProgram.methods
    .expelStudent()
    .accounts({
      academy: ctx.academy,
      academyAuthority: ctx.companyAuthority.publicKey,
      mentor: mentor.publicKey,
      student: await ctx.student(studentAuthority),
      studentAuthority,
    })
    .signers([ctx.companyAuthority, mentor])
    .rpc();
}
