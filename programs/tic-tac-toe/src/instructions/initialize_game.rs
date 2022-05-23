use anchor_lang::{
    prelude::*, 
    solana_program::{
        system_instruction,
        program::{
            invoke,
        },
    },
};
// use anchor_spl::{
//     associated_token::AssociatedToken,
//     mint,
//     token::{TokenAccount, Mint, Token}
// };
use crate::state::*;


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


    // let ntx = system_instruction::transfer(
    //     &ctx.accounts.challange_address.key(),
    //     &game.key(),
    //     // game.stake-1000
    //     100
    // );

    // // TODO: Deal with unwrap
    // let challange_bump = *ctx.bumps.get("challange_address").unwrap();
    // invoke_signed(
    //     &ntx,
    //     &[
    //         ctx.accounts.challange_address.to_account_info(),
    //         game.to_account_info(),
    //     ],
    //     &[
    //         &[
    //             , &[challange_bump]]
    //         ]
    // )?;
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
        // TODO: change the seeds 
        // Maciej.key(),game_taker.key() b"challenge", b"game"
        seeds = [challange_address.key().as_ref(), b"game"],
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
    // #[account(
    //     init,
    //     payer = game_taker,
    //     associated_token::mint = mint,
    //     associated_token::authority = game_account,
    // )]
    // pub vault: Account<'info, TokenAccount>,
    // pub mint: Account<'info, Mint>,
    /// CHECK: This account will be checked by validating it as challange_address seed
    #[account(mut)]
    pub opponent: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
    pub clock: Sysvar<'info, Clock>
}