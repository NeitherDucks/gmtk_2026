use std::collections::HashMap;
//use std::collections::HashSet;
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use crate::{
    LevelState,
    asset_loading::AssetHandles,
    entities::{ChestTag, DoorTag, StartingPointTag, WallTag},
    entities::{Lock, TrapType},
    gamemodes::{Tile, hand::Hand},
};

const TILE_SIZE: i32 = 16;

pub struct LoadingGameModePlugin;

impl Plugin for LoadingGameModePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(LevelState::Loading), setup);
        app.add_systems(
            OnTransition {
                entered: LevelState::Loading,
                exited: LevelState::Editing,
            },
            cache_item_locations,
        );

        app.add_systems(
            Update,
            (
                get_level_size,
                center_camera_to_level,
                get_hand,
                translate_grid_coords_entities,
                handle_level_loaded,
            )
                .run_if(in_state(LevelState::Loading)),
        );
    }
}

#[derive(Default, Resource, Debug)]
pub struct Grid {
    items: HashMap<GridCoords, Tile>,
    width: i32,
    height: i32,
}

impl Grid {
    pub fn get_items(&self) -> &HashMap<GridCoords, Tile> {
        &self.items
    }
    pub fn get_items_mut(&mut self) -> &mut HashMap<GridCoords, Tile> {
        &mut self.items
    }
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

fn setup(mut commands: Commands, handles: Res<AssetHandles>) {
    // We spawn our selected level
    commands.spawn(LdtkWorldBundle {
        ldtk_handle: handles.test_level.clone(),
        ..Default::default()
    });

    // This is what selects the level inside the ldtk file.
    commands.insert_resource(LevelSelection::index(0));

    commands.init_resource::<Grid>();
}

fn get_level_size(
    mut grid: ResMut<Grid>,
    mut level_messages: MessageReader<LevelEvent>,
    ldtk_project_entities: Single<&LdtkProjectHandle>,
    ldtk_project_assets: Res<Assets<LdtkProject>>,
) {
    for level_event in level_messages.read() {
        if let LevelEvent::Spawned(level_iid) = level_event {
            let ldtk_project = ldtk_project_assets
                .get(*ldtk_project_entities)
                .expect("LdtkProject should be loaded when level is spawned");
            let level = ldtk_project
                .get_raw_level_by_iid(level_iid.get())
                .expect("spawned level should exist in project");

            grid.width = level.px_wid / TILE_SIZE;
            grid.height = level.px_hei / TILE_SIZE;
        }
    }
}

/*
pub fn cache_wall_locations(
    query: Query<(&GridCoords, &TileEnumTags), Added<TileEnumTags>>,
    mut level_walls: ResMut<LevelWallCache>,
) {
    for (coords, tag) in query {
        level_walls.wall_locations.insert(*coords);
    }
}
*/

fn cache_item_locations(
    mut grid: ResMut<Grid>,
    query: Query<(
        Entity,
        &GridCoords,
        Option<&WallTag>,
        Option<&ChestTag>,
        Option<&DoorTag>,
        Option<&Lock>,
        Option<&StartingPointTag>,
        Option<&TrapType>,
    )>,
) {
    info!("cache_item_locations");
    for (entity, coord, wall, chest, door, lock, start, trap) in &query {
        if wall.is_some() {
            grid.items.insert(*coord, Tile::Wall);
        } else if chest.is_some() {
            grid.items.insert(*coord, Tile::Chest);
        } else if door.is_some() {
            grid.items
                .insert(*coord, Tile::Door(lock.copied().unwrap_or_default()));
        } else if start.is_some() {
            grid.items.insert(*coord, Tile::Dwarf);
        } else if let Some(trap) = trap {
            grid.items.insert(*coord, Tile::Trap((entity, *trap)));
        }
    }
}

/*
pub fn cache_chest_locations(
    mut level_chests: ResMut<LevelChests>,
    chests: Query<&GridCoords, With<ChestTag>>,
) {
    for coords in chests {
        info!("Found chest at: {coords:?}");
        level_chests.chest_locations.insert(*coords);
    }
}
*/

fn get_hand(
    mut commands: Commands,
    mut level_messages: MessageReader<LevelEvent>,
    ldtk_project_entities: Single<&LdtkProjectHandle>,
    ldtk_project_assets: Res<Assets<LdtkProject>>,
) {
    for level_event in level_messages.read() {
        if let LevelEvent::Spawned(level_iid) = level_event {
            let ldtk_project = ldtk_project_assets
                .get(*ldtk_project_entities)
                .expect("LdtkProject should be loaded when level is spawned");
            let level = ldtk_project
                .get_raw_level_by_iid(level_iid.get())
                .expect("spawned level should exist in project");

            info!("Loading player's hand");

            let traps = level
                .get_maybe_enums_field("HandTraps")
                .unwrap_or_default()
                .iter()
                .flatten()
                .map(TrapType::from)
                .collect::<Vec<TrapType>>();
            let amounts = level
                .get_maybe_ints_field("HandAmount")
                .unwrap_or_default()
                .iter()
                .flatten()
                .cloned()
                .map(|a| a as u32)
                .collect::<Vec<u32>>();

            let hand = traps.into_iter().zip(amounts).collect();
            info!("Player's hand: {hand:?}");
            commands.insert_resource(Hand(hand));
        }
    }
}

fn center_camera_to_level(
    mut level_messages: MessageReader<LevelEvent>,
    ldtk_project_entities: Single<&LdtkProjectHandle>,
    ldtk_project_assets: Res<Assets<LdtkProject>>,
    mut camera: Single<&mut Transform, With<Camera>>,
) {
    for level_event in level_messages.read() {
        if let LevelEvent::Spawned(level_iid) = level_event {
            let ldtk_project = ldtk_project_assets
                .get(*ldtk_project_entities)
                .expect("LdtkProject should be loaded when level is spawned");
            let level = ldtk_project
                .get_raw_level_by_iid(level_iid.get())
                .expect("spawned level should exist in project");

            let height = (level.px_hei / TILE_SIZE) / 2;
            let width = (level.px_wid / TILE_SIZE) / 2;

            let grid_offset = GridCoords::new(height, width);
            let offset = bevy_ecs_ldtk::utils::grid_coords_to_translation(
                grid_offset,
                IVec2::splat(TILE_SIZE),
            );

            camera.translation += offset.extend(0.0);
        }
    }
}

fn handle_level_loaded(
    mut level_messages: MessageReader<LevelEvent>,
    mut next_level_state: ResMut<NextState<LevelState>>,
) {
    for level_event in level_messages.read() {
        if let LevelEvent::Transformed(level_iid) = level_event {
            info!("Level with IID {} is completely loaded!", level_iid);

            // post-load logic, like moving to the editing_ui
            next_level_state.set(LevelState::Editing);
        }
    }
}

fn translate_grid_coords_entities(
    mut grid_coords_entities: Query<(&mut Transform, &GridCoords), Changed<GridCoords>>,
) {
    for (mut transform, grid_coords) in grid_coords_entities.iter_mut() {
        transform.translation =
            bevy_ecs_ldtk::utils::grid_coords_to_translation(*grid_coords, IVec2::splat(TILE_SIZE))
                .extend(transform.translation.z);
    }
}
