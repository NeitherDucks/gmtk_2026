use bevy::prelude::*;
use bevy_ecs_ldtk::{
    GridCoords, LdtkEntity, LdtkIntCell,
    app::{LdtkEntityAppExt, LdtkIntCellAppExt},
};
use bevy_ecs_tilemap::tiles::TileVisible;

use crate::GameState;

mod editing;
mod playing;

pub use editing::Traps;
pub struct GameModesPlugin;

impl Plugin for GameModesPlugin {
    fn build(&self, app: &mut App) {
        app.register_ldtk_entity::<Chest>("Chest")
            .register_ldtk_int_cell::<IntCell1>(1)
            .register_ldtk_int_cell::<IntCell2>(2)
            .register_ldtk_int_cell::<IntCell3>(3)
            .register_ldtk_int_cell::<IntCell4>(4)
            .register_ldtk_int_cell::<IntCell5>(5)
            .register_ldtk_int_cell::<IntCell6>(6)
            .add_systems(
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

/// Bundle to use for the "Chest" LdtkEntity
#[derive(Bundle, LdtkEntity, Default)]
pub struct Chest {
    #[sprite_sheet]
    sprite: Sprite,
    #[grid_coords]
    grid_coords: GridCoords,
}

/// Bundle to use for IntCell "1"
#[derive(Bundle, LdtkIntCell)]
pub struct IntCell {
    tile_visible: TileVisible,
}

impl Default for IntCell {
    fn default() -> Self {
        Self {
            tile_visible: TileVisible(false),
        }
    }
}

/// Bundle to use for IntCell "1"
#[derive(Bundle, LdtkIntCell)]
pub struct IntCell1 {
    tile_visible: TileVisible,
}

impl Default for IntCell1 {
    fn default() -> Self {
        Self {
            tile_visible: TileVisible(false),
        }
    }
}

/// Bundle to use for IntCell "2"
#[derive(Bundle, LdtkIntCell)]
pub struct IntCell2 {
    tile_visible: TileVisible,
}

impl Default for IntCell2 {
    fn default() -> Self {
        Self {
            tile_visible: TileVisible(false),
        }
    }
}

/// Bundle to use for IntCell "3"
#[derive(Bundle, LdtkIntCell)]
pub struct IntCell3 {
    tile_visible: TileVisible,
}

impl Default for IntCell3 {
    fn default() -> Self {
        Self {
            tile_visible: TileVisible(false),
        }
    }
}

/// Bundle to use for IntCell "4"
#[derive(Bundle, LdtkIntCell)]
pub struct IntCell4 {
    tile_visible: TileVisible,
}

impl Default for IntCell4 {
    fn default() -> Self {
        Self {
            tile_visible: TileVisible(false),
        }
    }
}

/// Bundle to use for IntCell "5"
#[derive(Bundle, LdtkIntCell)]
pub struct IntCell5 {
    tile_visible: TileVisible,
}

impl Default for IntCell5 {
    fn default() -> Self {
        Self {
            tile_visible: TileVisible(false),
        }
    }
}

/// Bundle to use for IntCell "5"
#[derive(Bundle, LdtkIntCell)]
pub struct IntCell6 {
    tile_visible: TileVisible,
}

impl Default for IntCell6 {
    fn default() -> Self {
        Self {
            tile_visible: TileVisible(false),
        }
    }
}
