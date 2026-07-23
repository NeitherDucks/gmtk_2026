use bevy::prelude::*;

use crate::GameState;

pub fn setup(mut next_state: ResMut<NextState<GameState>>) {
    // Temporarily by-pass the main menu
    next_state.set(GameState::EditLevel);
}

pub fn cleanup() {}
