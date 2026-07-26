use std::collections::{HashMap, HashSet, VecDeque};

use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use crate::entities::TrapType;

mod dwarf;
mod dwarfcharacter;
mod dwarftool;
mod dwarfcolor;
mod dwarfdirection;
mod dwarfaction;
mod dwarfresource;
mod dwarfactionrequest;
mod editing;
mod editing_ui;
mod loading;
mod playing;
mod playing_ui;
mod hand;

pub use dwarfcharacter::DwarfCharacter;
pub use dwarftool::DwarfTool;
pub use dwarfcolor::DwarfColor;
pub use dwarfdirection::DwarfDirection;
pub use dwarfaction::DwarfAction;
pub use dwarfresource::DwarfResource;

use dwarfcharacter::DwarfCharacterPlugin;
use dwarftool::DwarfToolPlugin;
use dwarfcolor::DwarfColorPlugin;
use dwarfdirection::DwarfDirectionPlugin;
use dwarfaction::DwarfActionPlugin;
use dwarfresource::DwarfResourcePlugin;
use dwarfactionrequest::DwarfActionRequest;

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
pub enum Item {
    Wall,
    Chest,
    Trap((Entity, TrapType)),
    Dwarf,
    Door,
}

#[derive(Default, Resource, Debug)]
pub struct Grid {
    items: HashMap<GridCoords, Item>,
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
        self.is_outside_level(grid_coords) || self.items.contains_key(grid_coords)
    }
}

#[derive(Default, Resource, Debug)]
pub struct LevelWalls {
    wall_locations: HashSet<GridCoords>,
    level_width: i32,
    level_height: i32,
}

impl LevelWalls {
    fn in_wall(&self, grid_coords: &GridCoords) -> bool {
        grid_coords.x < 0
            || grid_coords.y < 0
            || grid_coords.x >= self.level_width
            || grid_coords.y >= self.level_height
            || self.wall_locations.contains(grid_coords)
    }
}

#[derive(Default, Resource, Debug)]
pub struct LevelChests {
    chest_locations: HashSet<GridCoords>,
}

impl LevelChests {
    pub fn at_item(&self, grid_coords: &GridCoords) -> bool {
        self.chest_locations.contains(grid_coords)
    }
}

#[derive(Resource, Default)]
pub struct ActionsRequested(pub VecDeque<DwarfActionRequest>);
