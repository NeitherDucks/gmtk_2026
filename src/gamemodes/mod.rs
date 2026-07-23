use bevy::prelude::*;

use crate::GameState;

mod editing;
mod playing;

pub struct GameModesPlugin;

impl Plugin for GameModesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::EditLevel),
            (editing::setup, editing::setup_ui),
        )
        .add_systems(Update,
             editing::dev_input
        )
        .add_systems(
            OnTransition {
                exited: GameState::EditLevel,
                entered: GameState::PlayLevel,
            },
            (editing::cleanup_ui, (playing::setup, playing::setup_ui)).chain(),
        )
        .add_systems(
            OnTransition {
                exited: GameState::EditLevel,
                entered: GameState::MainMenu,
            },
            (editing::cleanup, editing::cleanup_ui),
        );
    }
}
