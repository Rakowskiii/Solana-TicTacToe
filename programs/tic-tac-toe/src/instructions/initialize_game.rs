use anchor_lang::{
    prelude::*, 
    solana_program::{
        system_instruction,
        program::{
            invoke,
        },
    },
};
use crate::state::*;

// TODO:  close => game_account? 


pub fn handler(ctx: Context<InitializeGame>) -> Result<()> {
    let game = &mut ctx.accounts.game_account;
    let clock = Clock::get()?;

    game.players[0] = ctx.accounts.game_taker.key();
    game.players[1] = ctx.accounts.opponent.key();
    game.whose_turn = Sign::X;
    game.stake = ctx.accounts.challange_address.stake;
    game.last_move_height = clock.unix_timestamp;

    let tx = system_instruction::transfer(
        &ctx.accounts.game_taker.key(),
        &game.key(),
        game.stake
    );

    invoke(
        &tx,
        &[
            ctx.accounts.game_taker.to_account_info(),
            game.to_account_info(),
        ]
    )?;

    let challange = ctx.accounts.challange_address.to_account_info();
    **challange.try_borrow_mut_lamports()? -= game.stake;
    **game.to_account_info().try_borrow_mut_lamports()? += game.stake;

    Ok(())
}

#[derive(Accounts)]
pub struct InitializeGame<'info>{
    #[account(
        mut,
    )]
    pub game_taker: Signer<'info>,
    #[account(
        init,
        seeds = [
            challange_address.key().as_ref(),
            game_taker.key().as_ref(), 
            b"game"
        ],
        bump,
        payer = game_taker,
        space = 8 + std::mem::size_of::<Game>(),
    )]
    pub game_account: Account<'info, Game>,
    #[account(
        seeds = [opponent.key().as_ref(), b"challange"],
        bump,
        mut,
        close = opponent
    )]
    pub challange_address: Account<'info, Challange>,
    #[account(mut)]
    /// CHECK: This is safe, as without proper opponent,
    /// challenge_address will be incorrect - leading to incorrect 
    /// or not-existant game
    pub opponent: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
    pub clock: Sysvar<'info, Clock>
}