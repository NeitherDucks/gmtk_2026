#![allow(unused)]

use crate::{asset_loading::AssetHandles, menus::widgets::item_button};
use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use std::collections::{HashSet, VecDeque};

const DWARF_MOVE_SPEED: f32 = 50.0;
const TILE_SIZE: i32 = 16;

#[derive(Default, Component)]
pub struct Goal;

#[derive(Default, Component)]
pub struct Wall;

#[derive(Default, Bundle, LdtkIntCell)]
struct WallBundle {
    wall: Wall,
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

#[derive(Default, Component)]
pub struct Chest;

#[derive(Default, Bundle, LdtkIntCell)]
struct ChestBundle {
    chest: Chest,
}

#[derive(Default, Resource, Debug)]
pub struct LevelChests {
    chest_locations: HashSet<GridCoords>,
    level_width: i32,
    level_height: i32,
}

impl LevelChests {
    fn at_item(&self, grid_coords: &GridCoords) -> bool {
        self.chest_locations.contains(grid_coords)
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum DwarfDirection {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum DwarfAction {
    Idle,
    Moving,
    Jump,
    LightLanding,
    HeavyLanding,
    StandUp,
    Shoveling, // only with shovel
    Climbing,  // only bare hands
    Light,     // only dynamite; file for body action is LightDynamite
    Throw,     // only dynamite; file for body action is ThrowDynamite
    Swing,     // only pickaxe or multitool; file for body action is PickaxeSwing
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum DwarfColor {
    Blue,
    Red,
    Yellow,
    Purple,
}

#[derive(Clone, Copy, PartialEq)]
pub enum DwarfResource {
    Stone, // only with shovel, pickaxe, multitool
    Iron,  // only with shovel, pickaxe, multitool
    Gold,  // only with shovel, pickaxe
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum DwarfTool {
    BareHands,
    MultiTool,
    Shovel,
    Pickaxe,
    Dynamite,
}

#[derive(Clone, Copy, Debug)]
pub enum DwarfActionRequest {
    MoveForward,
    ChangeTool(DwarfTool),
    ChangeDirection(DwarfDirection),
    TakeAction(DwarfAction),
    ChangeColor(DwarfColor),
}

#[derive(Resource, Default)]
pub struct ActionsRequested(pub VecDeque<DwarfActionRequest>);

#[derive(Resource, Clone)]
pub struct DwarfCharacter {
    grid_coords: GridCoords,
    color: DwarfColor,
    action: DwarfAction,
    direction: DwarfDirection,
    tool: DwarfTool,
    resource: DwarfResource,
    body: Entity,
    parts: Entity,
    move_distance: f32, // distance moved in current Moving action
}

#[derive(Component, Default, Clone)]
pub struct EditingLevelTag;

#[derive(Clone, PartialEq, Eq)]
pub enum Traps {
    Up,
    Left,
    Down,
    Right,
    Catapult,
    Rocks,
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

fn clone_dwarf_body_animation(dwarf: &DwarfCharacter, handles: &AssetHandles) -> Handle<Aseprite> {
    match dwarf.color {
        DwarfColor::Blue => match dwarf.action {
            DwarfAction::Idle => handles.dwarf_body_blue_idle.clone(),
            DwarfAction::Moving => handles.dwarf_body_blue_moving.clone(),
            DwarfAction::Jump => handles.dwarf_body_blue_jump.clone(),
            DwarfAction::LightLanding => handles.dwarf_body_blue_lightlanding.clone(),
            DwarfAction::HeavyLanding => handles.dwarf_body_blue_heavylanding.clone(),
            DwarfAction::StandUp => handles.dwarf_body_blue_standup.clone(),
            DwarfAction::Climbing => handles.dwarf_body_blue_climbing.clone(),
            DwarfAction::Shoveling => handles.dwarf_body_blue_shoveling.clone(),
            DwarfAction::Light => handles.dwarf_body_blue_light.clone(),
            DwarfAction::Throw => handles.dwarf_body_blue_throw.clone(),
            DwarfAction::Swing => handles.dwarf_body_blue_swing.clone(),
        },
        DwarfColor::Red => match dwarf.action {
            DwarfAction::Idle => handles.dwarf_body_red_idle.clone(),
            DwarfAction::Moving => handles.dwarf_body_red_moving.clone(),
            DwarfAction::Jump => handles.dwarf_body_red_jump.clone(),
            DwarfAction::LightLanding => handles.dwarf_body_red_lightlanding.clone(),
            DwarfAction::HeavyLanding => handles.dwarf_body_red_heavylanding.clone(),
            DwarfAction::StandUp => handles.dwarf_body_red_standup.clone(),
            DwarfAction::Climbing => handles.dwarf_body_red_climbing.clone(),
            DwarfAction::Shoveling => handles.dwarf_body_red_shoveling.clone(),
            DwarfAction::Light => handles.dwarf_body_red_light.clone(),
            DwarfAction::Throw => handles.dwarf_body_red_throw.clone(),
            DwarfAction::Swing => handles.dwarf_body_red_swing.clone(),
        },
        DwarfColor::Yellow => match dwarf.action {
            DwarfAction::Idle => handles.dwarf_body_yellow_idle.clone(),
            DwarfAction::Moving => handles.dwarf_body_yellow_moving.clone(),
            DwarfAction::Jump => handles.dwarf_body_yellow_jump.clone(),
            DwarfAction::LightLanding => handles.dwarf_body_yellow_lightlanding.clone(),
            DwarfAction::HeavyLanding => handles.dwarf_body_yellow_heavylanding.clone(),
            DwarfAction::StandUp => handles.dwarf_body_yellow_standup.clone(),
            DwarfAction::Climbing => handles.dwarf_body_yellow_climbing.clone(),
            DwarfAction::Shoveling => handles.dwarf_body_yellow_shoveling.clone(),
            DwarfAction::Light => handles.dwarf_body_yellow_light.clone(),
            DwarfAction::Throw => handles.dwarf_body_yellow_throw.clone(),
            DwarfAction::Swing => handles.dwarf_body_yellow_swing.clone(),
        },
        DwarfColor::Purple => match dwarf.action {
            DwarfAction::Idle => handles.dwarf_body_purple_idle.clone(),
            DwarfAction::Moving => handles.dwarf_body_purple_moving.clone(),
            DwarfAction::Jump => handles.dwarf_body_purple_jump.clone(),
            DwarfAction::LightLanding => handles.dwarf_body_purple_lightlanding.clone(),
            DwarfAction::HeavyLanding => handles.dwarf_body_purple_heavylanding.clone(),
            DwarfAction::StandUp => handles.dwarf_body_purple_standup.clone(),
            DwarfAction::Climbing => handles.dwarf_body_purple_climbing.clone(),
            DwarfAction::Shoveling => handles.dwarf_body_purple_shoveling.clone(),
            DwarfAction::Light => handles.dwarf_body_purple_light.clone(),
            DwarfAction::Throw => handles.dwarf_body_purple_throw.clone(),
            DwarfAction::Swing => handles.dwarf_body_purple_swing.clone(),
        },
    }
}

fn clone_dwarf_parts_animation(dwarf: &DwarfCharacter, handles: &AssetHandles) -> Handle<Aseprite> {
    match dwarf.action {
        DwarfAction::Idle => match dwarf.tool {
            DwarfTool::BareHands => handles.dwarf_parts_barehands_idle.clone(),
            DwarfTool::Shovel => match dwarf.resource {
                DwarfResource::Gold => handles.dwarf_parts_shovel_gold_idle.clone(),
                DwarfResource::Iron => handles.dwarf_parts_shovel_iron_idle.clone(),
                DwarfResource::Stone => handles.dwarf_parts_shovel_stone_idle.clone(),
            },
            DwarfTool::Pickaxe | DwarfTool::MultiTool => match dwarf.resource {
                DwarfResource::Gold => handles.dwarf_parts_pickaxe_gold_idle.clone(),
                DwarfResource::Iron => handles.dwarf_parts_pickaxe_iron_idle.clone(),
                DwarfResource::Stone => handles.dwarf_parts_pickaxe_stone_idle.clone(),
            },
            DwarfTool::Dynamite => handles.dwarf_parts_dynamite_idle.clone(),
        },
        DwarfAction::Moving => match dwarf.tool {
            DwarfTool::BareHands => handles.dwarf_parts_barehands_moving.clone(),
            DwarfTool::Shovel => match dwarf.resource {
                DwarfResource::Gold => handles.dwarf_parts_shovel_gold_moving.clone(),
                DwarfResource::Iron => handles.dwarf_parts_shovel_iron_moving.clone(),
                DwarfResource::Stone => handles.dwarf_parts_shovel_stone_moving.clone(),
            },
            DwarfTool::Pickaxe | DwarfTool::MultiTool => match dwarf.resource {
                DwarfResource::Gold => handles.dwarf_parts_pickaxe_gold_moving.clone(),
                DwarfResource::Iron => handles.dwarf_parts_pickaxe_iron_moving.clone(),
                DwarfResource::Stone => handles.dwarf_parts_pickaxe_stone_moving.clone(),
            },
            DwarfTool::Dynamite => handles.dwarf_parts_dynamite_moving.clone(),
        },
        DwarfAction::Jump => match dwarf.tool {
            DwarfTool::BareHands => handles.dwarf_parts_barehands_jump.clone(),
            DwarfTool::Shovel => match dwarf.resource {
                DwarfResource::Gold => handles.dwarf_parts_shovel_gold_jump.clone(),
                DwarfResource::Iron => handles.dwarf_parts_shovel_iron_jump.clone(),
                DwarfResource::Stone => handles.dwarf_parts_shovel_stone_jump.clone(),
            },
            DwarfTool::Pickaxe | DwarfTool::MultiTool => match dwarf.resource {
                DwarfResource::Gold => handles.dwarf_parts_pickaxe_gold_jump.clone(),
                DwarfResource::Iron => handles.dwarf_parts_pickaxe_iron_jump.clone(),
                DwarfResource::Stone => handles.dwarf_parts_pickaxe_stone_jump.clone(),
            },
            DwarfTool::Dynamite => handles.dwarf_parts_dynamite_jump.clone(),
        },
        DwarfAction::LightLanding => match dwarf.tool {
            DwarfTool::BareHands => handles.dwarf_parts_barehands_lightlanding.clone(),
            DwarfTool::Shovel => match dwarf.resource {
                DwarfResource::Gold => handles.dwarf_parts_shovel_gold_lightlanding.clone(),
                DwarfResource::Iron => handles.dwarf_parts_shovel_iron_lightlanding.clone(),
                DwarfResource::Stone => handles.dwarf_parts_shovel_stone_lightlanding.clone(),
            },
            DwarfTool::Pickaxe | DwarfTool::MultiTool => match dwarf.resource {
                DwarfResource::Gold => handles.dwarf_parts_pickaxe_gold_lightlanding.clone(),
                DwarfResource::Iron => handles.dwarf_parts_pickaxe_iron_lightlanding.clone(),
                DwarfResource::Stone => handles.dwarf_parts_pickaxe_stone_lightlanding.clone(),
            },
            DwarfTool::Dynamite => handles.dwarf_parts_dynamite_lightlanding.clone(),
        },
        DwarfAction::HeavyLanding => match dwarf.tool {
            DwarfTool::BareHands => handles.dwarf_parts_barehands_heavylanding.clone(),
            DwarfTool::Shovel => match dwarf.resource {
                DwarfResource::Gold => handles.dwarf_parts_shovel_gold_heavylanding.clone(),
                DwarfResource::Iron => handles.dwarf_parts_shovel_iron_heavylanding.clone(),
                DwarfResource::Stone => handles.dwarf_parts_shovel_stone_heavylanding.clone(),
            },
            DwarfTool::Pickaxe | DwarfTool::MultiTool => match dwarf.resource {
                DwarfResource::Gold => handles.dwarf_parts_pickaxe_gold_heavylanding.clone(),
                DwarfResource::Iron => handles.dwarf_parts_pickaxe_iron_heavylanding.clone(),
                DwarfResource::Stone => handles.dwarf_parts_pickaxe_stone_heavylanding.clone(),
            },
            DwarfTool::Dynamite => handles.dwarf_parts_dynamite_heavylanding.clone(),
        },
        DwarfAction::StandUp => match dwarf.tool {
            DwarfTool::BareHands => handles.dwarf_parts_barehands_standup.clone(),
            DwarfTool::Shovel => match dwarf.resource {
                DwarfResource::Gold => handles.dwarf_parts_shovel_gold_standup.clone(),
                DwarfResource::Iron => handles.dwarf_parts_shovel_iron_standup.clone(),
                DwarfResource::Stone => handles.dwarf_parts_shovel_stone_standup.clone(),
            },
            DwarfTool::MultiTool | DwarfTool::Pickaxe => match dwarf.resource {
                DwarfResource::Gold => handles.dwarf_parts_pickaxe_gold_standup.clone(),
                DwarfResource::Iron => handles.dwarf_parts_pickaxe_iron_standup.clone(),
                DwarfResource::Stone => handles.dwarf_parts_pickaxe_stone_standup.clone(),
            },
            DwarfTool::Dynamite => handles.dwarf_parts_dynamite_standup.clone(),
        },
        DwarfAction::Shoveling => match dwarf.tool {
            DwarfTool::Shovel => match dwarf.resource {
                DwarfResource::Gold => handles.dwarf_parts_shovel_gold_shoveling.clone(),
                DwarfResource::Iron => handles.dwarf_parts_shovel_iron_shoveling.clone(),
                DwarfResource::Stone => handles.dwarf_parts_shovel_stone_shoveling.clone(),
            },
            DwarfTool::BareHands
            | DwarfTool::MultiTool
            | DwarfTool::Pickaxe
            | DwarfTool::Dynamite => {
                unimplemented!();
            }
        },
        DwarfAction::Climbing => match dwarf.tool {
            DwarfTool::BareHands => handles.dwarf_parts_barehands_climbing.clone(),
            _ => unimplemented!(),
        },
        DwarfAction::Light => match dwarf.tool {
            DwarfTool::Dynamite => handles.dwarf_parts_dynamite_light.clone(),
            _ => unimplemented!(),
        },
        DwarfAction::Throw => match dwarf.tool {
            DwarfTool::Dynamite => handles.dwarf_parts_dynamite_throw.clone(),
            _ => unimplemented!(),
        },
        DwarfAction::Swing => match dwarf.tool {
            DwarfTool::Pickaxe | DwarfTool::MultiTool => match dwarf.resource {
                DwarfResource::Gold => handles.dwarf_parts_pickaxe_gold_swing.clone(),
                DwarfResource::Iron => handles.dwarf_parts_pickaxe_iron_swing.clone(),
                DwarfResource::Stone => handles.dwarf_parts_pickaxe_stone_swing.clone(),
            },
            _ => unimplemented!(),
        },
    }
}

pub fn cleanup() {}

pub fn setup_ui(mut commands: Commands) {
    commands.queue_spawn_scene(bsn! {
        EditingLevelTag
        Node {
            width: percent(100),
            height: percent(100),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
        }
        Children [
            Node {
                width: percent(100),
                height: percent(95),
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::End,
                justify_content: JustifyContent::Center,
            }
            Children [
                item_button(Traps::Up, 2),
                item_button(Traps::Down, 3),
                item_button(Traps::Left, 1),
                item_button(Traps::Right, 5),
                item_button(Traps::Catapult, 1),
                item_button(Traps::Rocks, 0),
            ]
        ]
    });
}

pub fn cleanup_ui(mut commands: Commands, entity: Single<Entity, With<EditingLevelTag>>) {
    commands.entity(*entity).despawn();
}

fn update_dwarf_body_animation(
    commands: &mut Commands,
    dwarf: &DwarfCharacter,
    handles: &AssetHandles,
) {
    let new_aseprite = clone_dwarf_body_animation(dwarf, handles);
    commands.entity(dwarf.body).insert(AseAnimation {
        animation: Animation::default(),
        aseprite: new_aseprite,
    });
}

fn update_dwarf_parts_animation(
    commands: &mut Commands,
    dwarf: &DwarfCharacter,
    handles: &AssetHandles,
) {
    let new_aseprite = clone_dwarf_parts_animation(dwarf, handles);
    commands.entity(dwarf.parts).insert(AseAnimation {
        animation: Animation::default(),
        aseprite: new_aseprite,
    });
}

fn apply_movement(
    dwarf: &mut DwarfCharacter,
    target_direction: DwarfDirection,
    dt: f32,
    transform: &mut Transform,
    level_walls: &LevelWalls,
    apply_movement_fn: impl FnOnce(&mut Transform, f32),
) -> (bool, bool) {
    let mut direction_changed = false;
    let mut action_changed = false;
    let mut able_to_move_forward = false;

    let grid_movement_direction = match target_direction {
        DwarfDirection::Up => GridCoords::new(0, 1),
        DwarfDirection::Down => GridCoords::new(0, -1),
        DwarfDirection::Left => GridCoords::new(-1, 0),
        DwarfDirection::Right => GridCoords::new(1, 0),
    };

    // can we move forward?
    let destination = dwarf.grid_coords + grid_movement_direction;
    println!(
        "now at {:?}. move {:?} to {:?}",
        dwarf.grid_coords, grid_movement_direction, destination
    );
    able_to_move_forward = !level_walls.in_wall(&destination);

    // Start moving if Idle
    if dwarf.action == DwarfAction::Idle && able_to_move_forward {
        dwarf.action = DwarfAction::Moving;
        dwarf.direction = target_direction;
        dwarf.move_distance = 0.0;
        direction_changed = true;
        action_changed = true;
    }

    // Keep moving if Moving and direction matches
    if dwarf.action == DwarfAction::Moving && dwarf.direction == target_direction {
        let distance_this_frame = DWARF_MOVE_SPEED * dt;
        dwarf.move_distance += distance_this_frame;
        if dwarf.direction == DwarfDirection::Left {
            transform.scale.x = -1.;
        } else {
            transform.scale.x = 1.;
        }

        if dwarf.move_distance >= TILE_SIZE as f32 {
            // Finish Moving exactly one tile's distance
            apply_movement_fn(
                transform,
                TILE_SIZE as f32 - (dwarf.move_distance - distance_this_frame),
            );
            dwarf.action = DwarfAction::Idle;
            dwarf.move_distance = 0.0;
            dwarf.grid_coords = destination;
            action_changed = true;
        } else {
            apply_movement_fn(transform, distance_this_frame);
        }
    }

    (direction_changed, action_changed)
}

pub fn process_action_request(
    mut commands: Commands,
    mut dwarf: If<ResMut<DwarfCharacter>>,
    mut actions_requested: ResMut<ActionsRequested>,
    handles: Res<AssetHandles>,
    time: Res<Time>,
    mut query: Query<&mut Transform>,
    level_walls: Res<LevelWalls>,
) {
    let dt = time.delta_secs();

    if let Some(request) = actions_requested.0.pop_front() {
        // start carrying it out
        let mut direction_changed = false;
        let mut body_action_changed = false;
        let mut tool_changed = false;
        let mut color_changed = false;
        let mut moved = false;

        match request {
            DwarfActionRequest::MoveForward => {
                let current_direction = dwarf.direction;
                if let Ok(mut transform) = query.get_mut(dwarf.body) {
                    let (dir_changed, action_changed) = apply_movement(
                        &mut dwarf,
                        current_direction,
                        dt,
                        &mut transform,
                        &level_walls,
                        |t, dist| match current_direction {
                            DwarfDirection::Up => t.translation.y += dist,
                            DwarfDirection::Down => t.translation.y -= dist,
                            DwarfDirection::Left => t.translation.x -= dist,
                            DwarfDirection::Right => t.translation.x += dist,
                        },
                    );
                    if dir_changed {
                        direction_changed = true;
                    }
                    if action_changed {
                        body_action_changed = true;
                    }
                    moved = true;
                }
            }
            DwarfActionRequest::ChangeTool(new_tool) => {
                if dwarf.tool != new_tool {
                    dwarf.tool = new_tool;
                    tool_changed = true;
                }
            }
            DwarfActionRequest::ChangeDirection(new_direction) => {
                if dwarf.direction != new_direction {
                    dwarf.direction = new_direction;
                    direction_changed = true;
                }
            }
            DwarfActionRequest::TakeAction(new_action) => {
                if dwarf.action != new_action {
                    dwarf.action = new_action;
                    body_action_changed = true;
                }
            }
            DwarfActionRequest::ChangeColor(new_color) => {
                if dwarf.color != new_color {
                    dwarf.color = new_color;
                    color_changed = true;
                }
            }
        }

        if direction_changed || body_action_changed || color_changed {
            update_dwarf_body_animation(&mut commands, &dwarf, &handles);
        }
        if direction_changed || tool_changed {
            update_dwarf_parts_animation(&mut commands, &dwarf, &handles);
        }

        // Keep parts transform in sync with body transform
        if moved {
            let body_pos = query.get(dwarf.body).ok().map(|t| t.translation).unwrap();
            if let Ok(mut parts_transform) = query.get_mut(dwarf.parts) {
                parts_transform.translation = body_pos + Vec3::new(0.0, 0.0, 1.0); // keep parts z one greater than body
            }
        }
    }
}

pub fn dev_input(
    input: Res<ButtonInput<KeyCode>>,
    mut dwarf: If<ResMut<DwarfCharacter>>,
    mut actions_requested: ResMut<ActionsRequested>,
) {
    if input.just_pressed(KeyCode::KeyC) {
        let next_color = match dwarf.color {
            DwarfColor::Blue => DwarfColor::Purple,
            DwarfColor::Purple => DwarfColor::Red,
            DwarfColor::Red => DwarfColor::Yellow,
            DwarfColor::Yellow => DwarfColor::Blue,
        };
        actions_requested
            .0
            .push_back(DwarfActionRequest::ChangeColor(next_color));
    } else if input.just_pressed(KeyCode::KeyT) {
        let next_tool = match dwarf.tool {
            DwarfTool::BareHands => DwarfTool::Shovel,
            DwarfTool::Shovel => DwarfTool::Dynamite,
            DwarfTool::Dynamite => DwarfTool::MultiTool,
            DwarfTool::MultiTool => DwarfTool::Pickaxe,
            DwarfTool::Pickaxe => DwarfTool::BareHands,
        };
        actions_requested
            .0
            .push_back(DwarfActionRequest::ChangeTool(next_tool));
    } else if input.pressed(KeyCode::KeyW) {
        actions_requested
            .0
            .push_back(DwarfActionRequest::ChangeDirection(DwarfDirection::Up));
        actions_requested
            .0
            .push_back(DwarfActionRequest::MoveForward);
    } else if input.pressed(KeyCode::KeyA) {
        actions_requested
            .0
            .push_back(DwarfActionRequest::ChangeDirection(DwarfDirection::Left));
        actions_requested
            .0
            .push_back(DwarfActionRequest::MoveForward);
    } else if input.pressed(KeyCode::KeyS) {
        actions_requested
            .0
            .push_back(DwarfActionRequest::ChangeDirection(DwarfDirection::Down));
        actions_requested
            .0
            .push_back(DwarfActionRequest::MoveForward);
    } else if input.pressed(KeyCode::KeyD) {
        actions_requested
            .0
            .push_back(DwarfActionRequest::ChangeDirection(DwarfDirection::Right));
        actions_requested
            .0
            .push_back(DwarfActionRequest::MoveForward);
    } else if input.just_pressed(KeyCode::KeyX) {
        // TODO: handle other actions, like Shoveling
        let next_action = DwarfAction::Idle;
        actions_requested
            .0
            .push_back(DwarfActionRequest::TakeAction(next_action));
    }
}

pub fn cache_wall_locations(
    mut level_walls: ResMut<LevelWalls>,
    mut level_messages: MessageReader<LevelEvent>,
    walls: Query<&GridCoords, With<Wall>>,
    ldtk_project_entities: Query<&LdtkProjectHandle>,
    ldtk_project_assets: Res<Assets<LdtkProject>>,
) -> Result {
    for level_event in level_messages.read() {
        if let LevelEvent::Spawned(level_iid) = level_event {
            let ldtk_project = ldtk_project_assets
                .get(ldtk_project_entities.single()?)
                .expect("LdtkProject should be loaded when level is spawned");
            let level = ldtk_project
                .get_raw_level_by_iid(level_iid.get())
                .expect("spawned level should exist in project");

            let wall_locations = walls.iter().copied().collect();

            let new_level_walls = LevelWalls {
                wall_locations,
                level_width: level.px_wid / TILE_SIZE,
                level_height: level.px_hei / TILE_SIZE,
            };
            println!("{:?}", new_level_walls);

            *level_walls = new_level_walls;
        }
    }
    Ok(())
}

pub fn cache_chest_locations(
    mut level_chests: ResMut<LevelChests>,
    mut level_messages: MessageReader<LevelEvent>,
    chests: Query<&GridCoords, With<Chest>>,
    ldtk_project_entities: Query<&LdtkProjectHandle>,
    ldtk_project_assets: Res<Assets<LdtkProject>>,
) -> Result {
    for level_event in level_messages.read() {
        if let LevelEvent::Spawned(level_iid) = level_event {
            let ldtk_project = ldtk_project_assets
                .get(ldtk_project_entities.single()?)
                .expect("LdtkProject should be loaded when level is spawned");
            let level = ldtk_project
                .get_raw_level_by_iid(level_iid.get())
                .expect("spawned level should exist in project");

            let chest_locations = chests.iter().copied().collect();

            let new_level_chests = LevelChests {
                chest_locations,
                level_width: level.px_wid / TILE_SIZE,
                level_height: level.px_hei / TILE_SIZE,
            };
            println!("{:?}", new_level_chests);

            *level_chests = new_level_chests;
        }
    }
    Ok(())
}

pub fn check_goal(dwarf: Res<DwarfCharacter>, goals: Query<&GridCoords, With<Goal>>) {
    if goals
        .iter()
        .any(|(goal_grid_coords)| &dwarf.grid_coords == goal_grid_coords)
    {
        println!("found goal at dwarf.grid_coords {:?}", dwarf.grid_coords);
    }
}

pub fn translate_grid_coords_entities(
    mut grid_coords_entities: Query<(Entity, &mut Transform, &GridCoords), Changed<GridCoords>>,
) {
    for (e, mut transform, grid_coords) in grid_coords_entities.iter_mut() {
        transform.translation =
            bevy_ecs_ldtk::utils::grid_coords_to_translation(*grid_coords, IVec2::splat(TILE_SIZE))
                .extend(transform.translation.z);
        println!("entity {:?} at {:?}", e, grid_coords);
    }
}
