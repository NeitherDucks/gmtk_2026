#![allow(unused)]

use bevy::prelude::*;

use crate::LevelState;

pub struct PlayingGameModePlugin;

impl Plugin for PlayingGameModePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(LevelState::Playing), setup);
    }
}

pub fn setup() {}
