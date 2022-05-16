import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Connection, Keypair, PublicKey } from "@solana/web3.js";
import { Academy } from "../target/types/academy";
import { Company } from "../target/types/company";
import { createMint, findATA, mintTo, TokenAccount } from "./token";
import { airdrop, findPDA } from "./utils";

export class Context {
  connection: Connection;

  academyProgram: Program<Academy>;
  companyProgram: Program<Company>;

  payer: Keypair;

  mntrMint: PublicKey;
  mntrMintAuthority: Keypair;

  academy: PublicKey;

  company: PublicKey;
  companyAuthority: Keypair;

  mentor1: Keypair;
  mentor2: Keypair;
  mentor3: Keypair;

  student1: Keypair;
  student2: Keypair;

  constructor() {
    this.connection = new Connection("http://localhost:8899", "recent");
    this.academyProgram = anchor.workspace.Academy;
    this.companyProgram = anchor.workspace.Company;
    this.payer = new Keypair();

    this.mntrMintAuthority = new Keypair();
    this.companyAuthority = new Keypair();
    this.mentor1 = new Keypair();
    this.mentor2 = new Keypair();
    this.mentor3 = new Keypair();
    this.student1 = new Keypair();
    this.student2 = new Keypair();
  }

  async setup() {
    await airdrop(this, [
      this.mntrMintAuthority.publicKey,
      this.companyAuthority.publicKey,
      this.mentor1.publicKey,
      this.mentor2.publicKey,
      this.mentor3.publicKey,
      this.student1.publicKey,
      this.student2.publicKey,
    ]);

    this.academy = await findPDA(
      [Buffer.from("academy")],
      this.academyProgram.programId
    );
    this.mntrMint = await createMint(this, this.mntrMintAuthority, 3);

    this.company = await findPDA(
      [Buffer.from("company")],
      this.companyProgram.programId
    );

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
    return await findPDA(
      [Buffer.from("student"), studentAuthority.toBuffer()],
      this.academyProgram.programId
    );
  }

  async employee(employeeAuthority: PublicKey): Promise<PublicKey> {
    return await findPDA(
      [Buffer.from("employee"), employeeAuthority.toBuffer()],
      this.companyProgram.programId
    );
  }

  async mntrATA(owner: PublicKey): Promise<TokenAccount> {
    return await findATA(this, owner, this.mntrMint);
  }
}
