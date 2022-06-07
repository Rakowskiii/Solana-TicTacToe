use anchor_lang::{
    prelude::*, 

};

use crate::state::*;


pub fn handler(ctx: Context<TakeTurn>, x: u8, y: u8) -> Result<()> {
    let game = &mut ctx.accounts.game_account;
    let clock = Clock::get()?;

    if !game.winner.is_none() {
        return Err(error!(TicTacToeError::GameEnded))
    }

    if ctx.accounts.player.key() != game.players[game.whose_turn as usize] {
        return Err(error!(TicTacToeError::InvalidPlayer))
    }

    let valid_range = 0..3; 

    if !(
        valid_range.contains(&x) && 
        valid_range.contains(&y)
    ) {
        return Err(error!(TicTacToeError::InvalidArugment))
    }

    let x = x as usize;
    let y = y as usize;

    if !game.gameboard[x][y].is_none() {
        return Err(error!(TicTacToeError::SpotTaken))
    }

    game.gameboard[x][y] = Some(game.whose_turn);

    match game.whose_turn {
        Sign::X => game.whose_turn = Sign::O,
        Sign::O => game.whose_turn = Sign::X
    }

    game.last_move_height = clock.unix_timestamp;

    
    let gb = &game.gameboard;
    // Horizontal checks
    if game.same([gb[0][0], gb[0][1], gb[0][2]]) ||
        game.same([gb[1][0], gb[1][1], gb[1][2]]) || 
        game.same([gb[2][0], gb[2][1], gb[2][2]]) || 

        // Vertical checks
        game.same([gb[0][0], gb[1][0], gb[2][0]]) || 
        game.same([gb[0][1], gb[1][1], gb[2][1]]) || 
        game.same([gb[0][2], gb[1][2], gb[2][2]]) ||
        
        // Cross checks
        game.same([gb[0][0], gb[1][1], gb[2][2]]) ||
        game.same([gb[2][0], gb[1][1], gb[0][2]]) {
          game.winner = Some(ctx.accounts.player.key())
    }

    Ok(())
}

#[derive(Accounts)]
pub struct TakeTurn<'info>{
    pub player: Signer<'info>,
    #[account(mut)]
    pub game_account: Account<'info, Game>,
    pub clock: Sysvar<'info, Clock>
}

#[error_code]
pub enum TicTacToeError {
    #[msg("Player key does not match expected player.")]
    InvalidPlayer,
    #[msg("X and Y should be chosen from [0,1,2].")]
    InvalidArugment,
    #[msg("Chosen spot is already taken.")]
    SpotTaken,
    #[msg("Game already ended.")]
    GameEnded,
}