use anchor_lang::{
    prelude::*, 
};
// use anchor_spl::{
//     associated_token::AssociatedToken,
//     mint,
//     token::{TokenAccount, Mint, Token}
// };
use crate::state::*;


pub fn handler(_ : Context<CancelChallange>) -> Result<()> {
    Ok(())
}

#[derive(Accounts)]
pub struct CancelChallange<'info>{
    #[account(
        mut,
    )]
    pub player: Signer<'info>,
    #[account(
        seeds = [player.key().as_ref(), b"challange"],
        bump,
        mut,
        close = player
    )]
    pub challange_address: Account<'info, Challange>,
}