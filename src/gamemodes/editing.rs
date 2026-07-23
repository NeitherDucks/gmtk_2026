use bevy::{ecs::resource, prelude::*};
use bevy_ecs_ldtk::{LdtkWorldBundle, LevelSelection};
use bevy_aseprite_ultra::prelude::*;
use crate::asset_loading::AssetHandles;

pub fn setup(mut commands: Commands, handles: Res<AssetHandles>) {
    // We need a camera
    commands.spawn((
        Camera2d,
        Projection::Orthographic(OrthographicProjection {
            scale: 0.5,
            ..OrthographicProjection::default_2d()
        }),
        Transform::from_xyz(128.0, 128.0, 0.0),
    ));

    // We spawn our selected level
    commands.spawn(LdtkWorldBundle {
        ldtk_handle: handles.test_level.clone(),
        ..Default::default()
    });

    // This is what selects the level inside the ldtk file.
    commands.insert_resource(LevelSelection::index(0));

    let x = 1.0;
    let y = 5.0;

    let tx = x * 16.0 + 8.0;   // these formulas aren't correct yet
    let ty = y * 16.0 - 8.0;   // plus they should be (and probably are)
                                    // in a world_to_screen-type function

    const BODY_Z : f32 = 2.0_f32;
    const PARTS_Z : f32 = 3.0_f32;

    let body_color = DwarfColor::Blue;
    let body_action = DwarfAction::Moving;
    let parts_action = DwarfAction::Moving;
    let tool = DwarfTool::Shovel;
    let resource = DwarfResource::Gold;

    commands.spawn((
        Name::new("TestDwarfBody"),
        Sprite::default(),
        Transform::from_translation(Vec3::new(tx, ty, BODY_Z)),
        AseAnimation {
            animation: Animation::default(),
            aseprite: clone_dwarf_body_animation(body_color, body_action, &handles),
        },
    ));
    commands.spawn((
        Name::new("TestDwarfParts"),
        Sprite::default(),
        Transform::from_translation(Vec3::new(tx, ty, PARTS_Z)),
        AseAnimation {
            animation: Animation::default(),
            aseprite: clone_dwarf_parts_animation(body_action, tool, resource, &handles),
        },
    ));
}

#[derive(Clone, Copy, PartialEq)]
pub enum DwarfAction {
    Idle,
    Moving,
    Jump,
    LightLanding,
    HeavyLanding,
    StandUp,
    Shoveling,  // only with shovel
    Climbing,   // only bare hands
    Light,  // only dynamite; file for body action is LightDynamite
    Throw,  // only dynamite; file for body action is ThrowDynamite
    Swing,  // only pickaxe or multitool; file for body action is PickaxeSwing
}

#[derive(PartialEq)]
pub enum DwarfColor {
    Blue,
    Red,
    Yellow,
    Purple,
}

#[derive(PartialEq)]
pub enum DwarfResource {
    Stone,  // only with shovel, pickaxe, multitool
    Iron,   // only with shovel, pickaxe, multitool
    Gold,   // only with shovel, pickaxe
}

#[derive(PartialEq)]
pub enum DwarfTool {
    BareHands,
    MultiTool,
    Shovel,
    Pickaxe,
    Dynamite,
}

fn clone_dwarf_body_animation(body_color: DwarfColor, body_action: DwarfAction, handles: &AssetHandles) -> Handle<Aseprite> {
    match body_action {
        DwarfAction::Idle => return handles.dwarf_body_blue_idle.clone(),
        DwarfAction::Moving => return handles.dwarf_body_blue_moving.clone(),
        DwarfAction::Jump => return handles.dwarf_body_blue_jump.clone(),
        DwarfAction::LightLanding => return handles.dwarf_body_blue_lightlanding.clone(),
        DwarfAction::HeavyLanding => return handles.dwarf_body_blue_heavylanding.clone(),
        DwarfAction::StandUp => return handles.dwarf_body_blue_standup.clone(),
        DwarfAction::Climbing => return handles.dwarf_body_blue_climbing.clone(),
        DwarfAction::Shoveling => return handles.dwarf_body_blue_shoveling.clone(),
        DwarfAction::Light => return handles.dwarf_body_blue_light.clone(),
        DwarfAction::Throw => return handles.dwarf_body_blue_throw.clone(),
        DwarfAction::Swing => return handles.dwarf_body_blue_swing.clone(),
    }
}

fn clone_dwarf_parts_animation(body_action: DwarfAction, tool: DwarfTool, resource: DwarfResource, handles: &AssetHandles) -> Handle<Aseprite> {
    match body_action {
        DwarfAction::Idle => {
            match tool {
                DwarfTool::BareHands => return handles.dwarf_parts_barehands_idle.clone(),
                DwarfTool::Shovel => {
                    match resource {
                        DwarfResource::Gold => return handles.dwarf_parts_shovel_gold_idle.clone(),
                        DwarfResource::Iron => return handles.dwarf_parts_shovel_iron_idle.clone(),
                        DwarfResource::Stone => return handles.dwarf_parts_shovel_stone_idle.clone(),
                    }
                },
                DwarfTool::Pickaxe |
                DwarfTool::MultiTool => {
                    match resource {
                        DwarfResource::Gold => return handles.dwarf_parts_pickaxe_gold_idle.clone(),
                        DwarfResource::Iron => return handles.dwarf_parts_pickaxe_iron_idle.clone(),
                        DwarfResource::Stone => return handles.dwarf_parts_pickaxe_stone_idle.clone(),
                    }
                },
                DwarfTool::Dynamite => return handles.dwarf_parts_dynamite_idle.clone(),
            }
        },
        DwarfAction::Moving => {
            match tool {
                DwarfTool::BareHands => return handles.dwarf_parts_barehands_moving.clone(),
                DwarfTool::Shovel => {
                    match resource {
                        DwarfResource::Gold => return handles.dwarf_parts_shovel_gold_moving.clone(),
                        DwarfResource::Iron => return handles.dwarf_parts_shovel_iron_moving.clone(),
                        DwarfResource::Stone => return handles.dwarf_parts_shovel_stone_moving.clone(),
                    }
                },
                DwarfTool::Pickaxe |
                DwarfTool::MultiTool => {
                    match resource {
                        DwarfResource::Gold => return handles.dwarf_parts_pickaxe_gold_moving.clone(),
                        DwarfResource::Iron => return handles.dwarf_parts_pickaxe_iron_moving.clone(),
                        DwarfResource::Stone => return handles.dwarf_parts_pickaxe_stone_moving.clone(),
                    }
                },
                DwarfTool::Dynamite => return handles.dwarf_parts_dynamite_moving.clone(),
            }
        },
        DwarfAction::Jump => {
            match tool {
                DwarfTool::BareHands => return handles.dwarf_parts_barehands_jump.clone(),
                DwarfTool::Shovel => {
                    match resource {
                        DwarfResource::Gold => return handles.dwarf_parts_shovel_gold_jump.clone(),
                        DwarfResource::Iron => return handles.dwarf_parts_shovel_iron_jump.clone(),
                        DwarfResource::Stone => return handles.dwarf_parts_shovel_stone_jump.clone(),
                    }
                },
                DwarfTool::Pickaxe |
                DwarfTool::MultiTool => {
                    match resource {
                        DwarfResource::Gold => return handles.dwarf_parts_pickaxe_gold_jump.clone(),
                        DwarfResource::Iron => return handles.dwarf_parts_pickaxe_iron_jump.clone(),
                        DwarfResource::Stone => return handles.dwarf_parts_pickaxe_stone_jump.clone(),
                    }
                },
                DwarfTool::Dynamite => return handles.dwarf_parts_dynamite_jump.clone(),
            }
        },
        DwarfAction::LightLanding => {
            match tool {
                DwarfTool::BareHands => return handles.dwarf_parts_barehands_lightlanding.clone(),
                DwarfTool::Shovel => {
                    match resource {
                        DwarfResource::Gold => return handles.dwarf_parts_shovel_gold_lightlanding.clone(),
                        DwarfResource::Iron => return handles.dwarf_parts_shovel_iron_lightlanding.clone(),
                        DwarfResource::Stone => return handles.dwarf_parts_shovel_stone_lightlanding.clone(),
                    }
                },
                DwarfTool::Pickaxe |
                DwarfTool::MultiTool => {
                    match resource {
                        DwarfResource::Gold => return handles.dwarf_parts_pickaxe_gold_lightlanding.clone(),
                        DwarfResource::Iron => return handles.dwarf_parts_pickaxe_iron_lightlanding.clone(),
                        DwarfResource::Stone => return handles.dwarf_parts_pickaxe_stone_lightlanding.clone(),
                    }
                },
                DwarfTool::Dynamite => return handles.dwarf_parts_dynamite_lightlanding.clone(),
            }
        },
        DwarfAction::HeavyLanding => {
            match tool {
                DwarfTool::BareHands => return handles.dwarf_parts_barehands_heavylanding.clone(),
                DwarfTool::Shovel => {
                    match resource {
                        DwarfResource::Gold => return handles.dwarf_parts_shovel_gold_heavylanding.clone(),
                        DwarfResource::Iron => return handles.dwarf_parts_shovel_iron_heavylanding.clone(),
                        DwarfResource::Stone => return handles.dwarf_parts_shovel_stone_heavylanding.clone(),
                    }
                },
                DwarfTool::Pickaxe |
                DwarfTool::MultiTool => {
                    match resource {
                        DwarfResource::Gold => return handles.dwarf_parts_pickaxe_gold_heavylanding.clone(),
                        DwarfResource::Iron => return handles.dwarf_parts_pickaxe_iron_heavylanding.clone(),
                        DwarfResource::Stone => return handles.dwarf_parts_pickaxe_stone_heavylanding.clone(),
                    }
                },
                DwarfTool::Dynamite => return handles.dwarf_parts_dynamite_heavylanding.clone(),
            }
        },
        DwarfAction::StandUp => {
            match tool {
                DwarfTool::BareHands => return handles.dwarf_parts_barehands_standup.clone(),
                DwarfTool::Shovel => {
                    match resource {
                        DwarfResource::Gold => return handles.dwarf_parts_shovel_gold_standup.clone(),
                        DwarfResource::Iron => return handles.dwarf_parts_shovel_iron_standup.clone(),
                        DwarfResource::Stone => return handles.dwarf_parts_shovel_stone_standup.clone(),
                    }
                },
                DwarfTool::MultiTool |
                DwarfTool::Pickaxe => {
                    match resource {
                        DwarfResource::Gold => return handles.dwarf_parts_pickaxe_gold_standup.clone(),
                        DwarfResource::Iron => return handles.dwarf_parts_pickaxe_iron_standup.clone(),
                        DwarfResource::Stone => return handles.dwarf_parts_pickaxe_stone_standup.clone(),
                    }
                },
                DwarfTool::Dynamite => return handles.dwarf_parts_dynamite_standup.clone(),
            }
        },
        DwarfAction::Shoveling => {
            match tool {
                DwarfTool::Shovel => {
                    match resource {
                        DwarfResource::Gold => return handles.dwarf_parts_shovel_gold_shoveling.clone(),
                        DwarfResource::Iron => return handles.dwarf_parts_shovel_iron_shoveling.clone(),
                        DwarfResource::Stone => return handles.dwarf_parts_shovel_stone_shoveling.clone(),
                    }
                },
                DwarfTool::BareHands | 
                DwarfTool::MultiTool |
                DwarfTool::Pickaxe |
                DwarfTool::Dynamite => {
                    unimplemented!();
                },
            }
        },
        DwarfAction::Climbing => {
            match tool {
                DwarfTool::BareHands => return handles.dwarf_parts_barehands_climbing.clone(),
                _ => unimplemented!(),
            }
        },
        DwarfAction::Light => {
            match tool {
                DwarfTool::Dynamite => return handles.dwarf_parts_dynamite_light.clone(),
                _ => unimplemented!(),
            }
        },
        DwarfAction::Throw => {
            match tool {
                DwarfTool::Dynamite => return handles.dwarf_parts_dynamite_throw.clone(),
                _ => unimplemented!(),
            }
        },
        DwarfAction::Swing => {
            match tool {
                DwarfTool::Pickaxe |
                DwarfTool::MultiTool => {
                    match resource {
                        DwarfResource::Gold => return handles.dwarf_parts_pickaxe_gold_swing.clone(),
                        DwarfResource::Iron => return handles.dwarf_parts_pickaxe_iron_swing.clone(),
                        DwarfResource::Stone => return handles.dwarf_parts_pickaxe_stone_swing.clone(),
                    }
                },
                _ => unimplemented!(),
            }
        }
    };
}

pub fn cleanup() {}

pub fn setup_ui() {}

pub fn cleanup_ui() {}
