use bevy::prelude::*;

use crate::GameState;

mod editing;
mod playing;

pub use editing::Traps;
pub struct GameModesPlugin;

impl Plugin for GameModesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::EditLevel),
            (editing::setup, editing::setup_ui),
        )
        .add_systems(
            Update,
            (
                editing::cache_wall_locations,
                editing::cache_chest_locations,
                editing::check_goal,
                editing::translate_grid_coords_entities,
                (editing::dev_input, editing::process_action_request).chain(),
            )
                .run_if(in_state(GameState::EditLevel)),
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
