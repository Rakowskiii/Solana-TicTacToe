use anchor_lang::{
    prelude::*, 
};

use crate::state::*;



pub fn handler(ctx: Context<WithdrawStake>) -> Result<()> {
    let game = &mut ctx.accounts.game_account;
    let clock = Clock::get()?;

    if !game.is_overdue(clock.unix_timestamp) 
    && game.winner.is_none()  {
        return Err(error!(TicTacToeError::NotFinished))
    }
    if Some(ctx.accounts.player.key()) != game.winner {
        if game.is_overdue(clock.unix_timestamp) {
            if ctx.accounts.player.key() != game.players[game.whose_turn.switch() as usize] {
                return Err(error!(TicTacToeError::NotTheWinner))
            } else {
                return Ok(())
            }
        }
        return Err(error!(TicTacToeError::NotTheWinner))
    }
    // Close handles the transfer
    Ok(())
}

#[derive(Accounts)]
pub struct WithdrawStake<'info>{
    #[account(
        mut,
    )]
    pub player: Signer<'info>,
    #[account(
        mut,
        close = player
    )]
    pub game_account: Account<'info, Game>,
    pub clock: Sysvar<'info, Clock>
}

#[error_code]
pub enum TicTacToeError {
    #[msg("Game is not yet finished.")]
    NotFinished,
    #[msg("You are not authorized to claim the winnings")]
    NotTheWinner
}