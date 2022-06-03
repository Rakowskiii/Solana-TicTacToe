use anchor_lang::{prelude::*};
use num_derive::*;

use super::TIMEOUT;


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
impl Sign {
    pub fn switch(&self) -> Sign {
        match self {
            Sign::X => {
                Sign::O
            },
            Sign::O => {
                Sign::X
            }
        }
    }
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

impl Game {
    pub fn is_overdue(&self, current_time: i64) -> bool {
        if self.last_move_height + TIMEOUT <= current_time {
            return true
        }
        return false
    }

    pub fn same(&self, elements: [Option<Sign>;3]) -> bool {
        if let Some(sign) = elements[0] {
            if let Some(second) = elements[1] {
                if let Some(third) = elements[2] {
                    return sign == second && second == third
                }
            }
        }
        return false
    }
}

#[account]
pub struct Challange {
    pub offerer: Pubkey,
    pub stake: u64, 
}