const anchor = require("@project-serum/anchor");
const web3 = require("@solana/web3.js");

const mntrMint = new web3.PublicKey(
  "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v"
);

async function findPDA(seeds, programId) {
  return (await web3.PublicKey.findProgramAddress(seeds, programId))[0];
}

module.exports = async function (_p) {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const academyProgram = anchor.workspace.Academy;
  const companyProgram = anchor.workspace.Company;

  const academy = await findPDA(
    [Buffer.from("academy")],
    academyProgram.programId
  );

  await academyProgram.methods
    .initialize(mntrMint)
    .accounts({
      academy: academy,
      academyAuthority: provider.wallet.publicKey,
      systemProgram: web3.SystemProgram.programId,
    })
    .rpc();

  const company = await findPDA(
    [Buffer.from("company")],
    companyProgram.programId
  );

  await companyProgram.methods
    .initialize()
    .accounts({
      company: company,
      companyAuthority: provider.wallet.publicKey,
      systemProgram: web3.SystemProgram.programId,
    })
    .rpc();
};
