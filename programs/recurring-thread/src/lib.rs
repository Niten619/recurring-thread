use anchor_lang::prelude::*;
use clockwork_sdk::state::{Thread, ThreadAccount};
use anchor_lang::solana_program::{
    instruction::Instruction,
    // native_token::LAMPORTS_PER_SOL
};
use anchor_lang::InstructionData;

declare_id!("J73oVDeCpLvffqA2R6eGozL2N76ucod1LpEGdB22wneU");

#[program]
pub mod recurring_thread {
    use std::vec;

    use super::*;

    pub fn increment(ctx: Context<Increment>) -> Result<()> {
        msg!("increment invoked!");
        let thread_authority = &ctx.accounts.thread_authority;
        let thread_id_old = b"old-thread-1";
        let thread_id_new = b"new-thread-1";
        
        let counter = &mut ctx.accounts.counter;
        counter.counter_value = counter.counter_value.checked_add(1).unwrap();
        counter.update_time = Clock::get().unwrap().unix_timestamp;

        let thread = &ctx.accounts.thread;
        let thread_new = &ctx.accounts.thread_new;
        msg!("thread old: {}", thread.key());
        msg!("thread new: {}", thread_new.key());
        
        let clockwork_program = &ctx.accounts.clockwork_program;
        let payer = &ctx.accounts.payer;
        // let system_program = &ctx.accounts.system_program;
        msg!("Thread address old: {}", Thread::pubkey(thread_authority.key(), thread_id_old.to_vec()));
        msg!("Thread address new: {}", Thread::pubkey(thread_authority.key(), thread_id_new.to_vec()));

        // Delete the old thread (created at initialize_thread function) via CPI.
        let bump = *ctx.bumps.get("thread_authority").unwrap();
        msg!("bump: {}", bump);
        clockwork_sdk::cpi::thread_delete(CpiContext::new_with_signer(
            clockwork_program.to_account_info(),
            clockwork_sdk::cpi::ThreadDelete {
                authority: thread_authority.to_account_info(),
                close_to: payer.to_account_info(),
                thread: thread.to_account_info(),
            },
            &[&[THREAD_AUTHORITY_SEED, &[bump]]],
        ))?;

        // let inx = Instruction{
        //     program_id: ID,
        //     accounts: crate::accounts::Increment{
        //         counter: counter.key(),
        //         // thread: thread.key(),
        //         // thread_authority: thread_authority.key()
        //         payer: payer.key(),
        //         system_program: system_program.key(),
        //         thread: thread.key(),
        //         thread_new: thread_new.key(),
        //         thread_authority: thread_authority.key(),
        //         clockwork_program: clockwork_program.key()
        //     }.to_account_metas(Some(true)),
        //     data: crate::instruction::Increment{}.data()
        // };

        // Defining Trigger for the thread
        // let current_timestamp = Clock::get()?.unix_timestamp as i64;
        // msg!("unix: {}", (current_timestamp + 120));
        // let trigger = clockwork_sdk::state::Trigger::Timestamp { 
        //     unix_ts: (current_timestamp + 120)  // 2 mins = 120 secs
        // };

        // Create a new Thread via CPI
        // let bump = *ctx.bumps.get("thread_authority").unwrap();
        // msg!("bump: {}", bump);
        // clockwork_sdk::cpi::thread_create(
        //     CpiContext::new_with_signer(
        //         clockwork_program.to_account_info(), 
        //         clockwork_sdk::cpi::ThreadCreate{
        //             authority: thread_authority.to_account_info(),
        //             payer: payer.to_account_info(),
        //             system_program: system_program.to_account_info(),
        //             thread: thread_new.to_account_info()
        //         }, 
        //         &[&[THREAD_AUTHORITY_SEED, &[bump]]]
        //     ), 
        //     // 0.01 as u64 * LAMPORTS_PER_SOL, 
        //     10_000_000 as u64,
        //     thread_id_new.to_vec(), 
        //     vec![inx.into()], 
        //     trigger
        // )?;

        Ok(())
    }

    pub fn initialize_thread(ctx: Context<Initialize>, unix_timestamp: i64) -> Result<()> {
        msg!("initialize thread invoked!");
        msg!("unix_timestamp: {}", unix_timestamp);
        // let thread_id = Clock::get()?.unix_timestamp as u64;
        let thread_id_old = b"old-thread";

        let counter = &mut ctx.accounts.counter;
        let thread = &ctx.accounts.thread;
        let thread_authority = &ctx.accounts.thread_authority;
        let clockwork_program = &ctx.accounts.clockwork_program;
        let payer = &ctx.accounts.payer;
        let system_program = &ctx.accounts.system_program;
        msg!("Thread address: {}", Thread::pubkey(thread_authority.key(), thread_id_old.to_vec()));
        // msg!("Thread address: {}", Thread::pubkey(thread_authority.key(), thread_id.to_le_bytes().to_vec()));

        let thread_new = &ctx.accounts.thread_new;
        // Preparing inx to be automated
        let inx = Instruction{
            program_id: ID,
            accounts: crate::accounts::Increment{
                counter: counter.key(),
                payer: payer.key(),
                // system_program: system_program.key(),
                thread: thread.key(),
                thread_new: thread_new.key(),
                thread_authority: thread_authority.key(),
                clockwork_program: clockwork_program.key()
            }.to_account_metas(Some(true)),
            data: crate::instruction::Increment{}.data()
        };
        // Defining Trigger for the thread
        // let trigger = clockwork_sdk::state::Trigger::Cron { 
        //     schedule: "*/10 * * * * * *".into(), 
        //     skippable: true 
        // };
        // let current_timestamp = Clock::get()?.unix_timestamp as i64;
        // msg!("unix: {}", (current_timestamp + 120));
        let trigger = clockwork_sdk::state::Trigger::Timestamp { 
            unix_ts: unix_timestamp  // 2 mins = 120 secs
        };

        // Create a Thread via CPI
        let bump = *ctx.bumps.get("thread_authority").unwrap();
        msg!("bump: {}", bump);
        clockwork_sdk::cpi::thread_create(
            CpiContext::new_with_signer(
                clockwork_program.to_account_info(), 
                clockwork_sdk::cpi::ThreadCreate{
                    authority: thread_authority.to_account_info(),
                    payer: payer.to_account_info(),
                    system_program: system_program.to_account_info(),
                    thread: thread.to_account_info()
                }, 
                &[&[THREAD_AUTHORITY_SEED, &[bump]]]
            ), 
            10_000_000 as u64, 
            thread_id_old.to_vec(), 
            vec![inx.into()], 
            trigger
        )?;

        Ok(())
    }

    pub fn reset(ctx: Context<Reset>) -> Result<()> {
        // Get accounts
        let clockwork_program = &ctx.accounts.clockwork_program;
        let payer = &ctx.accounts.payer;
        let thread = &ctx.accounts.thread;
        let thread_authority = &ctx.accounts.thread_authority;

        // Delete thread via CPI.
        let bump = *ctx.bumps.get("thread_authority").unwrap();
        clockwork_sdk::cpi::thread_delete(CpiContext::new_with_signer(
            clockwork_program.to_account_info(),
            clockwork_sdk::cpi::ThreadDelete {
                authority: thread_authority.to_account_info(),
                close_to: payer.to_account_info(),
                thread: thread.to_account_info(),
            },
            &[&[THREAD_AUTHORITY_SEED, &[bump]]],
        ))?;
        Ok(())
    }
}


// seed for counter pda
pub const COUNTER_SEED: &[u8] = b"counter";
// seed for thread_authority pda
pub const THREAD_AUTHORITY_SEED: &[u8] = b"thread_autho";


#[derive(Accounts)]
// #[instruction(thread_id: Vec<u8>)]
pub struct Increment<'info> {
    #[account(mut, seeds = [COUNTER_SEED], bump)]
    pub counter: Account<'info, Counter>,

    // IT RAN AFTER REMOVING THESE TWO
    #[account(mut)]
    pub payer: Signer<'info>,
    // pub system_program: Program<'info, System>,

    #[account(
        mut,
        address = Thread::pubkey(thread_authority.key(), b"old-thread-1".to_vec()),
    )]
    pub thread: Account<'info, Thread>,
    #[account(
        mut,
        address = Thread::pubkey(thread_authority.key(), b"new-thread-1".to_vec()),
    )]
    pub thread_new: SystemAccount<'info>,
    #[account(seeds = [THREAD_AUTHORITY_SEED], bump)]
    pub thread_authority: SystemAccount<'info>,
    pub clockwork_program: Program<'info, clockwork_sdk::ThreadProgram>
}

#[derive(Accounts)]
// #[instruction(thread_id: Vec<u8>)]
pub struct Initialize<'info> {
    #[account(init, payer = payer, seeds = [COUNTER_SEED], space = 8 + 8 + 8, bump)]
    pub counter: Account<'info, Counter>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
    #[account(
        mut,
        address = Thread::pubkey(thread_authority.key(), b"old-thread-1".to_vec()),
    )]
    pub thread: SystemAccount<'info>,
    #[account(
        mut,
        address = Thread::pubkey(thread_authority.key(), b"new-thread-1".to_vec()),
    )]
    pub thread_new: SystemAccount<'info>,
    #[account(seeds = [THREAD_AUTHORITY_SEED], bump)]
    pub thread_authority: SystemAccount<'info>,
    pub clockwork_program: Program<'info, clockwork_sdk::ThreadProgram>
}

#[derive(Accounts)]
pub struct Reset<'info> {
    // The signer.
    #[account(mut)]
    pub payer: Signer<'info>,
    // The Clockwork thread program.
    #[account(address = clockwork_sdk::ID)]
    pub clockwork_program: Program<'info, clockwork_sdk::ThreadProgram>,
    // The thread to reset.
    #[account(
        mut, 
        address = thread.pubkey(), 
        constraint = thread.authority.eq(&thread_authority.key())
    )]
    pub thread: Account<'info, Thread>,
    // The pda that owns and manages the thread.
    #[account(seeds = [THREAD_AUTHORITY_SEED], bump)]
    pub thread_authority: SystemAccount<'info>,
    // Close the counter account
    #[account(
        mut,
        seeds = [COUNTER_SEED],
        bump,
        close = payer
    )]
    pub counter: Account<'info, Counter>,
}

#[account]
pub struct Counter{
    pub counter_value: u64,
    pub update_time: i64,
}
