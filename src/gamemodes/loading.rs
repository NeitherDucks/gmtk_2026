#![allow(unused)]

use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::{Animation, AseAnimation};
use bevy_ecs_ldtk::ldtk::loaded_level::*;
use bevy_ecs_ldtk::prelude::*;
use std::collections::HashSet;

use crate::{
    LevelState,
    asset_loading::AssetHandles,
    gamemodes::{
        ActionsRequested, ChestTag, DwarfAction, DwarfCharacter, DwarfColor, DwarfDirection,
        DwarfResource, DwarfTool, Hand, LevelChests, LevelWalls, TrapType, WallTag,
        dwarf::{clone_dwarf_body_animation, clone_dwarf_parts_animation},
    },
};

const TILE_SIZE: i32 = 16;

pub struct LoadingGameModePlugin;

impl Plugin for LoadingGameModePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(LevelState::Loading), setup)
            .add_systems(
                Update,
                (
                    //process_map_extras,
                    get_level_size,
                    cache_wall_locations,
                    cache_chest_locations,
                    center_camera_to_level,
                    get_hand,
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
    commands.init_resource::<ActionsRequested>();

    spawn_initial_dwarf(commands.reborrow(), &handles);
}

fn spawn_initial_dwarf(mut commands: Commands, handles: &AssetHandles) {
    let grid_coords: GridCoords = GridCoords { x: 1, y: 5 };

    // these formulas aren't correct yet plus they should be (and probably are) in a world_to_screen-type function
    let tx = grid_coords.x * TILE_SIZE + TILE_SIZE / 2;
    let ty = grid_coords.y * TILE_SIZE - TILE_SIZE / 2;

    const BODY_Z: f32 = 10.0_f32;
    const PARTS_Z: f32 = 11.0_f32;

    let body_color = DwarfColor::Blue;
    let body_action = DwarfAction::Idle;
    let direction = DwarfDirection::Right;
    let tool = DwarfTool::BareHands;
    let resource = DwarfResource::Gold;

    let dwarf_body_entity = commands
        .spawn((
            Name::new("Body"),
            Sprite::default(),
            Transform::from_translation(Vec3::new(tx as f32, ty as f32, BODY_Z)),
        ))
        .id();
    let dwarf_parts_entity = commands
        .spawn((
            Name::new("Parts"),
            Sprite::default(),
            Transform::from_translation(Vec3::new(tx as f32, ty as f32, PARTS_Z)),
        ))
        .id();

    let dwarf = DwarfCharacter {
        grid_coords,
        action: body_action,
        direction,
        color: body_color,
        resource,
        tool,
        body: dwarf_body_entity,
        parts: dwarf_parts_entity,
        move_distance: 0.0,
    };

    // Add animations based on the dwarf's initial state
    commands.entity(dwarf_body_entity).insert(AseAnimation {
        animation: Animation::default(),
        aseprite: clone_dwarf_body_animation(&dwarf, handles),
    });
    commands.entity(dwarf_parts_entity).insert(AseAnimation {
        animation: Animation::default(),
        aseprite: clone_dwarf_parts_animation(&dwarf, handles),
    });

    commands.insert_resource(dwarf);
}

// I'm not sure of the differences between how to register/spawn-bundle-from tiles or placed entities, but for
// entities one would use the LdtkEntity registration on a bundle,
// e.g., Bundle (InteractibleEntity, ExactEntityName("Chest"), Chest(contents=RedPotion LdtkEntity), Lock(unlock_entity=SilverKey LdtkEntity))
// as in the field_instances example.

// Components:
// InteractibleEntity -- indicates something a dwarf would interact with
// ExactEntityName(namehash=hash("Silver Key")) -- or some way to get the entity's "type"
// Tool(type: DwarfTool) -- marks that the interactible entity is a type of tool
// DirectionChanger(dir: DwarfDirection) -- marks that the interactible entity is a direction-changer arrow
// Chest(contents: Option<EntityID>) -- marks that the interactible entity is a chest (with possible contents of an entity)
// Lock(unlock_entity: e.g., SILVER_KEY) -- marks that the Chest/Door is secured by a certain unlock_entity

// Other:
// register (invisible) Wall entities; cache them like before
// registered DwarfStart LdtkEntity (marker component) bundle, with DwarfColor, DwarfDirection, and DwarfTool components
// registered Door LdtkEntity which may also have a Lock component on it

pub fn get_level_size(
    mut level_walls: ResMut<LevelWalls>,
    mut level_messages: MessageReader<LevelEvent>,
    walls: Query<&GridCoords, With<WallTag>>,
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

            level_walls.level_width = level.px_wid / TILE_SIZE;
            level_walls.level_height = level.px_hei / TILE_SIZE;
        }
    }
}

pub fn cache_wall_locations(
    query: Query<(Entity, &GridCoords, &TileEnumTags), Added<TileEnumTags>>,
    mut level_walls: ResMut<LevelWalls>,
) {
    for (entity, coords, tag) in query {
        if tag.tags.contains(&"Wall".to_string()) {
            level_walls.wall_locations.insert(*coords);
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
            let ldtk_project = ldtk_project_assets
                .get(*ldtk_project_entities)
                .expect("LdtkProject should be loaded when level is spawned");
            let level = ldtk_project
                .get_raw_level_by_iid(level_iid.get())
                .expect("spawned level should exist in project");

            let mut traps = Vec::<TrapType>::new();
            let mut amounts = Vec::<u32>::new();

            for field in &level.field_instances {
                match &field.value {
                    FieldValue::Enums(values) => {
                        traps = values
                            .iter()
                            .cloned()
                            .filter_map(|v| v.map(|v| (&v).into()))
                            .collect();
                    }
                    FieldValue::Ints(values) => {
                        amounts = values
                            .iter()
                            .cloned()
                            .filter_map(|v| v.map(|v| v as u32))
                            .collect();
                    }
                    _ => {}
                }
            }

            let hand = traps.into_iter().zip(amounts).collect();
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
