#![allow(unused)]

use crate::asset_loading::AssetHandles;
use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::*;
use bevy_ecs_ldtk::{LdtkWorldBundle, LevelSelection};

#[derive(Resource, Clone)]
pub struct DwarfCharacter {
    color: DwarfColor,
    action: DwarfAction,
    direction: DwarfDirection,
    tool: DwarfTool,
    resource: DwarfResource,
    body: Entity,
    parts: Entity,
}

pub fn setup(mut commands: Commands, handles: Res<AssetHandles>) {
    // We spawn our selected level
    commands.spawn(LdtkWorldBundle {
        ldtk_handle: handles.test_level.clone(),
        ..Default::default()
    });

    // This is what selects the level inside the ldtk file.
    commands.insert_resource(LevelSelection::index(0));

    spawn_initial_dwarf(commands.reborrow(), &handles);
}

fn spawn_initial_dwarf(mut commands: Commands, handles: &AssetHandles) {
    let x = 1.0;
    let y = 5.0;

    // these formulas aren't correct yet plus they should be (and probably are) in a world_to_screen-type function
    let tx = x * 16.0 + 8.0;
    let ty = y * 16.0 - 8.0;

    const BODY_Z: f32 = 2.0_f32;
    const PARTS_Z: f32 = 3.0_f32;

    let body_color = DwarfColor::Blue;
    let body_action = DwarfAction::Idle;
    let direction = DwarfDirection::Left;
    let tool = DwarfTool::BareHands;
    let resource = DwarfResource::Gold;

    let dwarf_body_entity = commands
        .spawn((
            Name::new("TestDwarfBody"),
            Sprite::default(),
            Transform::from_translation(Vec3::new(tx, ty, BODY_Z)),
        ))
        .id();
    let dwarf_parts_entity = commands
        .spawn((
            Name::new("TestDwarfParts"),
            Sprite::default(),
            Transform::from_translation(Vec3::new(tx, ty, PARTS_Z)),
        ))
        .id();
    let dwarf = DwarfCharacter {
        action: body_action,
        direction,
        color: body_color,
        resource,
        tool,
        body: dwarf_body_entity,
        parts: dwarf_parts_entity,
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

#[derive(Clone, Copy, PartialEq)]
pub enum DwarfDirection {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Copy, PartialEq)]
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

#[derive(Clone, Copy, PartialEq)]
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

#[derive(Clone, Copy, PartialEq)]
pub enum DwarfTool {
    BareHands,
    MultiTool,
    Shovel,
    Pickaxe,
    Dynamite,
}

fn clone_dwarf_body_animation(
    dwarf: &DwarfCharacter,
    handles: &AssetHandles,
) -> Handle<Aseprite> {
    match dwarf.action {
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
    }
}

fn clone_dwarf_parts_animation(
    dwarf: &DwarfCharacter,
    handles: &AssetHandles,
) -> Handle<Aseprite> {
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

pub fn setup_ui() {}

pub fn cleanup_ui() {}

fn update_dwarf_body_animation(
    commands: &mut Commands,
    dwarf: &DwarfCharacter,
    handles: &AssetHandles,
) {
    let mut temp_dwarf = dwarf.clone();
    temp_dwarf.color = DwarfColor::Blue;
    let new_aseprite = clone_dwarf_body_animation(&temp_dwarf, handles);
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

pub fn dev_input(
    mut commands: Commands,
    input: Res<ButtonInput<KeyCode>>,
    mut dwarf: If<ResMut<DwarfCharacter>>,
    handles: Res<AssetHandles>,
) {
    let mut direction_changed = false;
    let mut body_action_changed = false;
    let mut tool_changed = false;
    if input.just_pressed(KeyCode::KeyT) {
        // change tool
        dwarf.tool = match dwarf.tool {
            DwarfTool::BareHands => DwarfTool::Shovel,
            DwarfTool::Shovel => DwarfTool::Dynamite,
            DwarfTool::Dynamite => DwarfTool::MultiTool,
            DwarfTool::MultiTool => DwarfTool::Pickaxe,
            DwarfTool::Pickaxe => DwarfTool::BareHands,
        };
        tool_changed = true;
    }
    if input.pressed(KeyCode::KeyW) {
        // if not going Direction::Up, go that way
        if dwarf.direction != DwarfDirection::Up {
            dwarf.direction = DwarfDirection::Up;
            direction_changed = true;
        }

        // if not Moving change to Moving
        if dwarf.action != DwarfAction::Moving {
            body_action_changed = true;
        }
    }
    if input.pressed(KeyCode::KeyA) {
        // if not going Direction::Up, go that way
        if dwarf.direction != DwarfDirection::Left {
            dwarf.direction = DwarfDirection::Left;
            direction_changed = true;
        }

        // if not Moving change to Moving
        if dwarf.action != DwarfAction::Moving {
            body_action_changed = true;
        }
    }
    if input.pressed(KeyCode::KeyS) {
        // if not going Direction::Up, go that way
        if dwarf.direction != DwarfDirection::Down {
            dwarf.direction = DwarfDirection::Down;
            direction_changed = true;
        }

        // if not Moving change to Moving
        if dwarf.action != DwarfAction::Moving {
            body_action_changed = true;
        }
    }
    if input.pressed(KeyCode::KeyD) {
        // if not going Direction::Up, go that way
        if dwarf.direction != DwarfDirection::Right {
            dwarf.direction = DwarfDirection::Right;
            direction_changed = true;
        }

        // if not Moving change to Moving
        if dwarf.action != DwarfAction::Moving {
            body_action_changed = true;
        }
    }
    if direction_changed || body_action_changed {
        update_dwarf_body_animation(
            &mut commands,
            &mut dwarf,
            &handles,
        );
    }
    if direction_changed || tool_changed {
        update_dwarf_parts_animation(
            &mut commands,
            &mut dwarf,
            &handles,
        );
    }
}
