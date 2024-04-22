import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Sol404 } from "../target/types/sol_404";
import { amount, Metaplex } from "@metaplex-foundation/js"
import * as fs from 'fs';
import * as spl from "@solana/spl-token"
import { assert } from "chai"
import { PROGRAM_ID as TOKEN_METADATA_PROGRAM_ID } from "@metaplex-foundation/mpl-token-metadata"



describe("sol_404", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Sol404 as Program<Sol404>;
  const connection = program.provider.connection
  const metaplex = Metaplex.make(connection)
  
  const privateKeyJson = "/Users/zhoujianing/.config/solana/id.json"
  const privateKeyString = fs.readFileSync(privateKeyJson, { encoding: 'utf8' });
  const privateKeyUint8Array = new Uint8Array(JSON.parse(privateKeyString));
  const admin = anchor.web3.Keypair.fromSecretKey(privateKeyUint8Array);


  const [JellyTokenMintPDA] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("jelly")],
    program.programId
  )

  const [UsdcTokenMintPDA] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("usdc")],
    program.programId
  )

  const [NftTokenMintPDA] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("nft")],
    program.programId
  )

  const JellyTokenAccount = spl.getAssociatedTokenAddressSync(
    JellyTokenMintPDA,
    admin.publicKey
  )

  const UsdcTokenAccount = spl.getAssociatedTokenAddressSync(
    UsdcTokenMintPDA,
    admin.publicKey
  )

  const NftTokenAccount = spl.getAssociatedTokenAddressSync(
    NftTokenMintPDA,
    admin.publicKey
  )

  const jellymetadata = {
    uri: "https://raw.githubusercontent.com/solana-developers/program-examples/new-examples/tokens/tokens/.assets/spl-token.json",
    name: "Jelly",
    symbol: "jelly",
  }

  const usdcmetadata = {
    uri: "https://raw.githubusercontent.com/solana-developers/program-examples/new-examples/tokens/tokens/.assets/spl-token.json",
    name: "usdc",
    symbol: "usdc",
  }

  const nftmetadata = {
    uri: "https://raw.githubusercontent.com/solana-developers/program-examples/new-examples/tokens/tokens/.assets/spl-token.json",
    name: "nft",
    symbol: "nft",
  }

  it("Is create mint!", async () => {
    const jellyTokenMintMetadataPDA = await metaplex
    .nfts()
    .pdas()
    .metadata({ mint: JellyTokenMintPDA });

    const usdcTokenMintMetadataPDA = await metaplex
    .nfts()
    .pdas()
    .metadata({ mint: UsdcTokenMintPDA });

    const nftTokenMintMetadataPDA = await metaplex
    .nfts()
    .pdas()
    .metadata({ mint: NftTokenMintPDA });

    const tx = await program.methods
    .createmint(jellymetadata.uri, usdcmetadata.uri, nftmetadata.uri, jellymetadata.name, usdcmetadata.name, nftmetadata.name, jellymetadata.symbol, usdcmetadata.symbol, nftmetadata.symbol)
    .accounts({
      admin: admin.publicKey,
      jellyTokenMint: JellyTokenMintPDA,
      usdcTokenMint: UsdcTokenMintPDA,
      nftMint: NftTokenMintPDA,
      jellyTokenAccount: JellyTokenAccount,
      usdcTokenAccount: UsdcTokenAccount,
      jellyMetadataAccount: jellyTokenMintMetadataPDA,
      usdcMetadataAccount: usdcTokenMintMetadataPDA,
      nftMetadataAccount: nftTokenMintMetadataPDA,
      tokenMetadataProgram: TOKEN_METADATA_PROGRAM_ID,
    })
    .rpc();
    console.log("Your transaction signature", tx);
  });
});
