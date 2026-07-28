use std::collections::{HashMap, HashSet};

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
mod loading;
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
                loading::LoadingGameModePlugin,
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

#[derive(Default, Resource, Debug)]
pub struct Grid {
    items: HashMap<GridCoords, Tile>,
    width: i32,
    height: i32,
}

impl Grid {
    pub fn is_outside_level(&self, grid_coords: &GridCoords) -> bool {
        grid_coords.x < 0
            || grid_coords.y < 0
            || grid_coords.x >= self.width
            || grid_coords.y >= self.height
    }

    pub fn is_collision(&self, grid_coords: &GridCoords) -> bool {
        self.is_outside_level(grid_coords)
            || match self.items.get(grid_coords) {
                Some(Tile::Wall)
                | Some(Tile::Chest)
                | Some(Tile::Dwarf)
                | Some(Tile::Door(Lock::Gold))
                | Some(Tile::Door(Lock::Silver)) => true,
                Some(Tile::Trap((_, trap))) => match trap {
                    TrapType::Nothing
                    | TrapType::Up
                    | TrapType::Left
                    | TrapType::Down
                    | TrapType::Right
                    | TrapType::Catapult => false,
                    TrapType::Rock => true,
                },
                Some(Tile::Door(Lock::Unlocked)) | None => false,
            }
    }

    pub fn is_passable(&self, grid_coords: &GridCoords) -> bool {
        !self.is_collision(grid_coords)
    }
}

#[derive(Default, Resource, Debug)]
pub struct LevelWalls {
    wall_locations: HashSet<GridCoords>,
    level_width: i32,
    level_height: i32,
}

/*
impl LevelWalls {
    fn in_wall(&self, grid_coords: &GridCoords) -> bool {
        grid_coords.x < 0
            || grid_coords.y < 0
            || grid_coords.x >= self.level_width
            || grid_coords.y >= self.level_height
            || self.wall_locations.contains(grid_coords)
    }
}
*/

#[derive(Default, Resource, Debug)]
pub struct LevelChests {
    chest_locations: HashSet<GridCoords>,
}

impl LevelChests {
    pub fn at_item(&self, grid_coords: &GridCoords) -> bool {
        self.chest_locations.contains(grid_coords)
    }
}
