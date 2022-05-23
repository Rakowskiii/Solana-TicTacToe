use anchor_lang::{prelude::*};
use num_derive::*;



#[derive(
    AnchorSerialize,
    AnchorDeserialize,
    FromPrimitive,
    ToPrimitive,
    Copy,
    Clone,
    PartialEq,
    Eq
)]

pub enum Sign {
    X = 0,
    O = 1
}

#[account]
pub struct Game {
    pub players: [Pubkey; 2],
    pub gameboard: [[Option<Sign>;3];3],
    pub whose_turn: Sign, 
    pub stake: u64, 
	pub last_move_height: i64,
    pub winner: Option<Pubkey>,
}



#[account]
pub struct Challange {
    pub offerer: Pubkey,
    pub stake: u64, 
}