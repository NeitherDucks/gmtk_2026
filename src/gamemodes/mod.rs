use bevy::prelude::*;
use bevy_ecs_ldtk::{
    GridCoords, IntGridRendering, LdtkEntity, LdtkIntCell, LdtkSettings, SetClearColor,
    app::LdtkEntityAppExt,
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
            // .register_default_ldtk_int_cell::<HideIntCell>()
            .insert_resource(LdtkSettings {
                int_grid_rendering: IntGridRendering::Invisible,
                set_clear_color: SetClearColor::FromLevelBackground,
                ..Default::default()
            })
            .add_systems(
                OnEnter(GameState::EditLevel),
                (editing::setup, editing::setup_ui),
            )
            .add_systems(
                Update,
                (
                    editing::center_camera_to_level,
                    editing::get_hand,
                    editing::cache_wall_locations,
                    editing::cache_chest_locations,
                    editing::check_goal,
                    editing::translate_grid_coords_entities,
                    (editing::dev_input, editing::process_action_request).chain(),
                    editing::update_hand_ui.run_if(
                        resource_exists::<editing::Hand>
                            .and_then(resource_changed::<editing::Hand>),
                    ),
                )
                    .run_if(in_state(GameState::EditLevel)),
            )
            .add_systems(
                Update,
                editing::playing_input.run_if(in_state(GameState::PlayLevel)),
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
pub struct HideIntCell {
    tile_visible: TileVisible,
}

impl Default for HideIntCell {
    fn default() -> Self {
        Self {
            tile_visible: TileVisible(false),
        }
    }
}
