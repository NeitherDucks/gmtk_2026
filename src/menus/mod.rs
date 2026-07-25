use bevy::prelude::*;

use crate::GameState;

mod choose_level;
mod credits;
mod main_menu;
mod options;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::MainMenu), main_menu::setup)
            .add_systems(OnExit(GameState::MainMenu), main_menu::cleanup)
            .add_systems(OnEnter(GameState::ChooseLevel), choose_level::setup)
            .add_systems(OnExit(GameState::ChooseLevel), choose_level::cleanup)
            .add_systems(OnEnter(GameState::Options), options::setup)
            .add_systems(OnExit(GameState::Options), options::cleanup)
            .add_systems(OnEnter(GameState::Credits), credits::setup)
            .add_systems(OnExit(GameState::Credits), credits::cleanup);
    }
}
