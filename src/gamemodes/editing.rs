use crate::LevelState;
use bevy::prelude::*;
pub struct EditingGameModePlugin;

impl Plugin for EditingGameModePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(LevelState::Editing), setup);
    }
}

pub fn setup() {}
