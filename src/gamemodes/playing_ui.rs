use bevy::prelude::*;

use crate::LevelState;

pub struct PlayingUIGameModePlugin;

impl Plugin for PlayingUIGameModePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(LevelState::Playing), setup_ui);
    }
}

fn setup_ui() {}
