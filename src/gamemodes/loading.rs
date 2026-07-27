use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use crate::{
    LevelState,
    asset_loading::AssetHandles,
    entities::{ChestTag, DoorTag, StartingPointTag, TrapType},
    gamemodes::{Grid, Item, LevelChests, LevelWalls, hand::Hand, playing::spawn_dwarves},
};

const TILE_SIZE: i32 = 16;

pub struct LoadingGameModePlugin;

impl Plugin for LoadingGameModePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(LevelState::Loading), (setup, spawn_dwarves).chain());
        app.add_systems(
            Update,
            (
                get_level_size,
                cache_wall_locations,
                cache_chest_locations,
                cache_items_location,
                center_camera_to_level,
                get_hand,
                translate_grid_coords_entities,
                handle_level_loaded,
            )
                .run_if(in_state(LevelState::Loading)),
        );
    }
}

pub fn setup(mut commands: Commands, handles: Res<AssetHandles>) {
    // We spawn our selected level
    commands.spawn(LdtkWorldBundle {
        ldtk_handle: handles.test_level.clone(),
        ..Default::default()
    });

    // This is what selects the level inside the ldtk file.
    commands.insert_resource(LevelSelection::index(0));

    commands.init_resource::<LevelWalls>();
    commands.init_resource::<LevelChests>();

    commands.init_resource::<Grid>();
}

pub fn get_level_size(
    mut level_walls: ResMut<LevelWalls>,
    mut grid: ResMut<Grid>,
    mut level_messages: MessageReader<LevelEvent>,
    ldtk_project_entities: Single<&LdtkProjectHandle>,
    ldtk_project_assets: Res<Assets<LdtkProject>>,
) {
    for level_event in level_messages.read() {
        if let LevelEvent::Spawned(level_iid) = level_event {
            info!("get_level_size saw LevelEvent::Spawned for the level");
            let ldtk_project = ldtk_project_assets
                .get(*ldtk_project_entities)
                .expect("LdtkProject should be loaded when level is spawned");
            let level = ldtk_project
                .get_raw_level_by_iid(level_iid.get())
                .expect("spawned level should exist in project");

            level_walls.level_width = level.px_wid / TILE_SIZE;
            level_walls.level_height = level.px_hei / TILE_SIZE;

            grid.width = level.px_wid / TILE_SIZE;
            grid.height = level.px_hei / TILE_SIZE;
        }
    }
}

pub fn cache_wall_locations(
    query: Query<(&GridCoords, &TileEnumTags), Added<TileEnumTags>>,
    mut level_walls: ResMut<LevelWalls>,
    mut grid: ResMut<Grid>,
) {
    for (coords, tag) in query {
        if tag.tags.contains(&"Wall".to_string()) {
            level_walls.wall_locations.insert(*coords);
            grid.items.insert(*coords, Item::Wall);
        }
    }
}

pub fn cache_items_location(
    mut grid: ResMut<Grid>,
    query: Query<
        (
            Entity,
            &GridCoords,
            Option<&ChestTag>,
            Option<&DoorTag>,
            Option<&StartingPointTag>,
            Option<&TrapType>,
        ),
        Added<GridCoords>,
    >,
) {
    for (entity, coord, chest, door, start, trap) in &query {
        if chest.is_some() {
            grid.items.insert(*coord, Item::Chest);
        } else if door.is_some() {
            grid.items.insert(*coord, Item::Door);
        } else if start.is_some() {
            grid.items.insert(*coord, Item::Dwarf);
        } else if let Some(trap) = trap {
            grid.items.insert(*coord, Item::Trap((entity, *trap)));
        }
    }
}

pub fn cache_chest_locations(
    mut level_chests: ResMut<LevelChests>,
    chests: Query<&GridCoords, Added<ChestTag>>,
) {
    for coords in chests {
        info!("Found chest at: {coords:?}");
        level_chests.chest_locations.insert(*coords);
    }
}

pub fn get_hand(
    mut commands: Commands,
    mut level_messages: MessageReader<LevelEvent>,
    ldtk_project_entities: Single<&LdtkProjectHandle>,
    ldtk_project_assets: Res<Assets<LdtkProject>>,
) {
    for level_event in level_messages.read() {
        if let LevelEvent::Spawned(level_iid) = level_event {
            info!("get_hand saw LevelEvent::Spawned for the level");
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

pub fn center_camera_to_level(
    mut level_messages: MessageReader<LevelEvent>,
    ldtk_project_entities: Single<&LdtkProjectHandle>,
    ldtk_project_assets: Res<Assets<LdtkProject>>,
    mut camera: Single<&mut Transform, With<Camera>>,
) {
    for level_event in level_messages.read() {
        if let LevelEvent::Spawned(level_iid) = level_event {
            info!("center_camera_to_level saw LevelEvent::Spawned for the level");
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

pub fn handle_level_loaded(
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

pub fn translate_grid_coords_entities(
    mut grid_coords_entities: Query<(&mut Transform, &GridCoords), Changed<GridCoords>>,
) {
    for (mut transform, grid_coords) in grid_coords_entities.iter_mut() {
        transform.translation =
            bevy_ecs_ldtk::utils::grid_coords_to_translation(*grid_coords, IVec2::splat(TILE_SIZE))
                .extend(transform.translation.z);
    }
}
