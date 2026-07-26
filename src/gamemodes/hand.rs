use bevy::prelude::*;
use crate::entities::TrapType;


#[derive(Debug, Resource)]
pub struct Hand(pub Vec<(TrapType, u32)>);

impl Hand {
    pub fn increment(&mut self, trap: &TrapType) -> Option<u32> {
        for (hand_trap, amount) in &mut self.0 {
            if hand_trap == trap {
                *amount = amount.saturating_add(1);

                return Some(*amount);
            }
        }

        None
    }

    pub fn decrement(&mut self, trap: &TrapType) -> Option<u32> {
        for (hand_trap, amount) in &mut self.0 {
            if hand_trap == trap {
                *amount = amount.saturating_sub(1);

                return Some(*amount);
            }
        }

        None
    }
}
