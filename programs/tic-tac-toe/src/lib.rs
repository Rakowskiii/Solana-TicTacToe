use anchor_lang::prelude::*;
pub mod instructions; 
pub mod state;

use instructions::*;



declare_id!("DLBUzNyHxH8Yqn6Jdm15wKchLUwvRaQtT5WoaFQxEsWp");

#[program]
pub mod tic_tac_toe {
    use super::*;

    pub fn initialize_challange(ctx: Context<InitializeChallange>, stake: u64) -> Result<()> {
        instructions::initialize_challange::handler(ctx, stake)
    }
    
    pub fn initialize_game(ctx: Context<InitializeGame>) -> Result<()> {
        instructions::initialize_game::handler(ctx)
    }

    pub fn take_move(ctx: Context<TakeTurn>, x: u8, y: u8) -> Result<()> {
        instructions::take_turn::handler(ctx, x, y)
    }

    pub fn cancel_challange(ctx: Context<CancelChallange>) -> Result<()> {
        instructions::cancel_challange::handler(ctx)
    }

    pub fn withdraw_stake(ctx: Context<WithdrawStake>) -> Result<()> {
        instructions::withdraw_stake::handler(ctx)
    }
}

