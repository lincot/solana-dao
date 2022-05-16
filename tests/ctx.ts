import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Connection, Keypair, PublicKey } from "@solana/web3.js";
import { Dao } from "../target/types/dao";
import { createMint, findATA, mintTo, TokenAccount } from "./token";
import { airdrop, findPDA } from "./utils";

export class Context {
  connection: Connection;
  program: Program<Dao>;
  payer: Keypair;

  mntrMint: PublicKey;
  mntrMintAuthority: Keypair;

  dao: PublicKey;
  daoAuthority: Keypair;

  mentor1: Keypair;
  mentor2: Keypair;
  mentor3: Keypair;

  student1: Keypair;
  student2: Keypair;

  constructor() {
    this.connection = new Connection("http://localhost:8899", "recent");
    this.program = anchor.workspace.Dao;
    this.payer = new Keypair();

    this.mntrMintAuthority = new Keypair();
    this.daoAuthority = new Keypair();
    this.mentor1 = new Keypair();
    this.mentor2 = new Keypair();
    this.mentor3 = new Keypair();
    this.student1 = new Keypair();
    this.student2 = new Keypair();
  }

  async setup() {
    await airdrop(this, [
      this.mntrMintAuthority.publicKey,
      this.daoAuthority.publicKey,
      this.mentor1.publicKey,
      this.mentor2.publicKey,
      this.mentor3.publicKey,
      this.student1.publicKey,
      this.student2.publicKey,
    ]);

    this.dao = await findPDA(this, [Buffer.from("dao")]);
    this.mntrMint = await createMint(this, this.mntrMintAuthority, 3);

    await mintTo(
      this,
      await this.mntrATA(this.mentor1.publicKey),
      this.mntrMintAuthority,
      5
    );
    await mintTo(
      this,
      await this.mntrATA(this.mentor2.publicKey),
      this.mntrMintAuthority,
      10
    );
    await mintTo(
      this,
      await this.mntrATA(this.mentor3.publicKey),
      this.mntrMintAuthority,
      100
    );
  }

  async student(studentAuthority: PublicKey): Promise<PublicKey> {
    return await findPDA(this, [
      Buffer.from("student"),
      studentAuthority.toBuffer(),
    ]);
  }

  async mntrATA(owner: PublicKey): Promise<TokenAccount> {
    return await findATA(this, owner, this.mntrMint);
  }
}
