import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { RecurringThread } from "../target/types/recurring_thread";
import { ClockworkProvider, PAYER_PUBKEY } from "@clockwork-xyz/sdk";
import { Keypair, PublicKey, SystemProgram } from "@solana/web3.js";
import { assert, expect } from "chai";
import { readFileSync } from "fs";
import { BN } from "bn.js";

describe("recurring-thread", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const wallet = provider.wallet;

  const program = anchor.workspace.RecurringThread as Program<RecurringThread>;
  const clockworkProvider = ClockworkProvider.fromAnchorProvider(provider);
  const THREAD_AUTHORITY_SEED = "thread_autho";
  const COUNTER_SEED = "counter";

  const threadId = "old-thread-1"
    const threadIdNew = "new-thread-1"
    const [threadAuthority] = PublicKey.findProgramAddressSync(
        [Buffer.from(anchor.utils.bytes.utf8.encode(THREAD_AUTHORITY_SEED))],
        program.programId
    );
    console.log("threadAuthority:", threadAuthority.toBase58())

    const [counterPda] = PublicKey.findProgramAddressSync(
        [Buffer.from(anchor.utils.bytes.utf8.encode(COUNTER_SEED))],
        program.programId
    );
    console.log("counterPda:", counterPda.toBase58())

    const [threadAddress] = clockworkProvider.getThreadPDA(
        threadAuthority,
        threadId
    );
    console.log("threadAddress:", threadAddress.toBase58())
    const [threadAddressNew] = clockworkProvider.getThreadPDA(
        threadAuthority,
        threadIdNew
    );
    console.log("threadAddressNew:", threadAddressNew.toBase58())

    const current_time_unix = Math.floor(Date.now()/1000) + 30;
    console.log("current_time_unix:", current_time_unix)

    // FETCH
    // const counter_info = await program.account.counter.fetch(counterPda);
    // console.log("counterValue:", counter_info.counterValue.toNumber())
    // console.log("updateTime:", counter_info.updateTime.toNumber())
    // assert(false)

    it("Thread Initialization & Execution!", async () => {
        const tx = await program.methods.initializeThread(
            new BN(current_time_unix)
            ).accounts({
                counter: counterPda,
                payer: wallet.publicKey,
                systemProgram: SystemProgram.programId,
                thread: threadAddress,
                threadNew: threadAddressNew,
                threadAuthority: threadAuthority,
                clockworkProgram: clockworkProvider.threadProgram.programId,
            }).rpc();
        console.log("tx:", tx)
    });

    // it("Delete Thread!", async () => {
    // // Just some cleanup to reset the test to a clean state
    // afterEach(async () => {
    //     try {
    //         await program.methods
    //             .reset()
    //             .accounts({
    //                 payer: wallet.publicKey,
    //                 clockworkProgram: clockworkProvider.threadProgram.programId,
    //                 counter: counterPda,
    //                 thread: threadAddress,
    //                 threadAuthority: threadAuthority,
    //             })
    //             .rpc();
    //     } catch (e) { }
    //     })
    // });

});
