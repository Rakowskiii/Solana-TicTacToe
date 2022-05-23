use anchor_lang::{
    prelude::*, 
    solana_program::{
        system_instruction,
        program::invoke,
    },
};

use crate::state::*;


pub fn handler(ctx: Context<InitializeChallange>, stake: u64) -> Result<()> {
    let challange = &mut ctx.accounts.challange;

    challange.offerer = ctx.accounts.initializer.key();
    challange.stake = stake;

    let tx = system_instruction::transfer(
        &ctx.accounts.initializer.key(),
        &challange.key(),
        stake
    );

    invoke(
        &tx,
        &[
            ctx.accounts.initializer.to_account_info(),
            ctx.accounts.challange.to_account_info(),
        ]
    )?;
    
    Ok(())
}

#[derive(Accounts)]
pub struct InitializeChallange<'info>{
    #[account(
        mut,
    )]
    pub initializer: Signer<'info>,
    #[account(
        init,
        seeds = [initializer.key().as_ref(), b"challange"],
        bump,
        payer = initializer,
        space = 8 + std::mem::size_of::<Challange>(),
    )]
    pub challange: Account<'info, Challange>,
    pub system_program: Program<'info, System>
}

