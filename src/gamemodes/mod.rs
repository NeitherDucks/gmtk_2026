use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use crate::entities::{Lock, TrapType};

mod dwarf;
mod dwarfaction;
mod dwarfactionrequest;
mod dwarfcharacter;
mod dwarfcolor;
mod dwarfdirection;
mod dwarfresource;
mod dwarftool;
mod editing;
mod editing_ui;
mod hand;
mod level;
mod playing;
mod playing_ui;
mod requests;

pub use dwarfaction::{DwarfAction, DwarfActionComponent};
pub use dwarfcharacter::DwarfCharacter;
pub use dwarfcolor::{DwarfColor, DwarfColorComponent};
pub use dwarfdirection::{DwarfDirection, DwarfDirectionComponent};
pub use dwarfresource::{DwarfResource, DwarfResourceComponent};
pub use dwarftool::{DwarfTool, DwarfToolComponent};
use requests::Requests;

use dwarfaction::DwarfActionPlugin;
use dwarfactionrequest::DwarfActionRequest;
use dwarfcharacter::DwarfCharacterPlugin;
use dwarfcolor::DwarfColorPlugin;
use dwarfdirection::DwarfDirectionPlugin;
use dwarfresource::DwarfResourcePlugin;
use dwarftool::DwarfToolPlugin;

pub struct GameModesPlugin;

impl Plugin for GameModesPlugin {
    fn build(&self, app: &mut App) {
        app //
            .insert_resource(LdtkSettings {
                int_grid_rendering: IntGridRendering::Invisible,
                set_clear_color: SetClearColor::FromLevelBackground,
                ..Default::default()
            })
            .add_plugins((
                DwarfToolPlugin,
                DwarfColorPlugin,
                DwarfDirectionPlugin,
                DwarfActionPlugin,
                DwarfResourcePlugin,
                DwarfCharacterPlugin,
                level::LoadingGameModePlugin,
                editing::EditingGameModePlugin,
                editing_ui::EditingUIGameModePlugin,
                playing::PlayingGameModePlugin,
                playing_ui::PlayingUIGameModePlugin,
            ));
    }
}

#[derive(Debug)]
pub enum Tile {
    Wall,
    Chest,
    Trap((Entity, TrapType)),
    Dwarf,
    Door(Lock),
}
