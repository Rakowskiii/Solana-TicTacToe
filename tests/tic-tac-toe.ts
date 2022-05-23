import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { createProgramAddressSync, findProgramAddressSync } from "@project-serum/anchor/dist/cjs/utils/pubkey";
import { Connection, LAMPORTS_PER_SOL, SYSVAR_CLOCK_PUBKEY } from "@solana/web3.js";
import { BN } from "bn.js";
import { expect, should } from "chai";
import { TicTacToe } from "../target/types/tic_tac_toe";

describe("tic-tac-toe", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.TicTacToe as Program<TicTacToe>;
  const connection = anchor.getProvider().connection;

  const gameKeypair = anchor.web3.Keypair.generate();
  const playerOne = anchor.web3.Keypair.generate();
  const playerTwo = anchor.web3.Keypair.generate();
  
  const [challange, _challangeBump] = findProgramAddressSync([playerOne.publicKey.toBuffer(), Buffer.from("challange")], program.programId) 
  const [game, _gameBump] = findProgramAddressSync([challange.toBuffer(), Buffer.from("game")], program.programId) 


  before("Init!", async () => {
    let signature = await connection.requestAirdrop(
      playerOne.publicKey,
      1*LAMPORTS_PER_SOL
    )
    await connection.confirmTransaction(signature)
    
    signature = await connection.requestAirdrop(
      playerTwo.publicKey,
      1*LAMPORTS_PER_SOL
    )
    await connection.confirmTransaction(signature)
  })

  it("Initialise first challange!", async () => {
    // Add your test here.
    // const tx = await program.methods.initialize().rpc();
    // console.log("Your transaction signature", tx);
    await program.methods
      .initializeChallange(new BN(0.1* LAMPORTS_PER_SOL))
      .accounts(
        {
          initializer: playerOne.publicKey,
          challange,
          systemProgram: anchor.web3.SystemProgram.programId
        }
      )
      .signers([playerOne])
      .rpc()
      
      let challangeState = await program.account.challange.fetch(challange)
      expect(challangeState.offerer.equals(playerOne.publicKey)).to.equal(true)
      expect(challangeState.stake.toNumber()).to.equal(0.1*LAMPORTS_PER_SOL)
  

    let challangeBalance = await connection.getBalance(challange)
    // console.log(challangeBalance)
    expect(challangeBalance).to.be.above(0.1*LAMPORTS_PER_SOL)
  })

  it("Initialise game!", async () => {
    await program.methods
          .initializeGame()
          .accounts({
            gameTaker: playerTwo.publicKey,
            gameAccount: game,
            challangeAddress: challange,
            opponent: playerOne.publicKey,
            systemProgram: anchor.web3.SystemProgram.programId,
            clock: anchor.web3.SYSVAR_CLOCK_PUBKEY
          })
          .signers([playerTwo])
          .rpc()
      
    let gameState = await program.account.game.fetch(game)
    
    let challangeBalance = await connection.getBalance(challange)
    let gameBalance = await connection.getBalance(game)
    expect(gameBalance).to.be.above(0.2*LAMPORTS_PER_SOL)
    expect(challangeBalance).to.be.equal(0)
  })

  it("Initialise another challlenge", async () => {
    await program.methods
        .initializeChallange(new BN(0.1* LAMPORTS_PER_SOL))
        .accounts(
          {
            initializer: playerOne.publicKey,
            challange,
            systemProgram: anchor.web3.SystemProgram.programId
          }
        )
        .signers([playerOne])
        .rpc()
      let challangeBalance = await connection.getBalance(challange)
  })

  it("Fail to initialise one more challenge, previous not consumed", async () => {
    try {
      await program.methods
        .initializeChallange(new BN(0.1* LAMPORTS_PER_SOL))
        .accounts(
          {
            initializer: playerOne.publicKey,
            challange,
            systemProgram: anchor.web3.SystemProgram.programId
          }
        )
        .signers([playerOne])
        .rpc()
    } catch {
      return
    }
    expect.fail(null, null, "Shouldn't be able to place another challenge, when one's already posted.")
    })

  it("Close the challange!", async () => {
    await program.methods
      .cancelChallange()
      .accounts({
        player: playerOne.publicKey,
        challangeAddress: challange
      })
      .signers([playerOne])
      .rpc()
  })

  it("Initialise another challlange, after closing previous one", async () => {
    await program.methods
        .initializeChallange(new BN(0.3* LAMPORTS_PER_SOL))
        .accounts(
          {
            initializer: playerOne.publicKey,
            challange,
            systemProgram: anchor.web3.SystemProgram.programId
          }
        )
        .signers([playerOne])
        .rpc()
  })

  it("Take a first move!", async () => {
    await program.methods
      .takeMove(0, 0)
      .accounts({
        player: playerTwo.publicKey,
        gameAccount: game,
        clock: SYSVAR_CLOCK_PUBKEY
      })
      .signers([playerTwo])
      .rpc()
  })

  it("Failt to take a second move as THE SAME player second time in a row!", async () => {
    try {
      await program.methods
      .takeMove(2, 2)
      .accounts({
        player: playerTwo.publicKey,
        gameAccount: game,
        clock: SYSVAR_CLOCK_PUBKEY
      })
      .signers([playerTwo])
      .rpc()  
    } catch {
      return
    }
    expect.fail(null, null, "The same player shouldn't be able to play twice in a row.")
    
  })
  
  it("Take a second VALID move as the challenger!", async () => {
    await program.methods
      .takeMove(1, 1)
      .accounts({
        player: playerOne.publicKey,
        gameAccount: game,
        clock: SYSVAR_CLOCK_PUBKEY
      })
      .signers([playerOne])
      .rpc()
  })

  it("Failt to take a third move, in the taken spot!", async () => {
    try {
      await program.methods
      .takeMove(1, 1)
      .accounts({
        player: playerTwo.publicKey,
        gameAccount: game,
        clock: SYSVAR_CLOCK_PUBKEY
      })
      .signers([playerTwo])
      .rpc()
    } catch {
      return
    }
    expect.fail(null, null, "Shouldn't be able to place another sign into same spot.")
  })

  it("Fail to take a fourth move, in a spot outside the gameboard!", async () => {
    try {
      await program.methods
      .takeMove(4, 5)
      .accounts({
        player: playerTwo.publicKey,
        gameAccount: game,
        clock: SYSVAR_CLOCK_PUBKEY
      })
      .signers([playerTwo])
      .rpc()

      let gameState = await program.account.game.fetch(game)
      console.log(gameState)
    } catch {
      return
    }
    expect.fail(null, null, "Chosen spots should be in range 0-2.")
    
    
  })
});
