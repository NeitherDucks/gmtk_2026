use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::{Animation, AseAnimation, Aseprite};

use crate::{
    asset_loading::AssetHandles,
    gamemodes::{DwarfAction, DwarfColor, DwarfResource, DwarfTool},
};

pub fn clone_dwarf_body_animation(
    dwarf_color: &DwarfColor,
    dwarf_action: &DwarfAction,
    handles: &AssetHandles,
) -> Handle<Aseprite> {
    match dwarf_color {
        DwarfColor::Blue => match dwarf_action {
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
        DwarfColor::Red => match dwarf_action {
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
        DwarfColor::Yellow => match dwarf_action {
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
        DwarfColor::Purple => match dwarf_action {
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

pub fn clone_dwarf_parts_animation(
    dwarf_action: &DwarfAction,
    dwarf_tool: &DwarfTool,
    dwarf_resource: &DwarfResource,
    handles: &AssetHandles,
) -> Handle<Aseprite> {
    match dwarf_action {
        DwarfAction::Idle => match dwarf_tool {
            DwarfTool::BareHands => handles.dwarf_parts_barehands_idle.clone(),
            DwarfTool::Shovel => match dwarf_resource {
                DwarfResource::Gold => handles.dwarf_parts_shovel_gold_idle.clone(),
                DwarfResource::Iron => handles.dwarf_parts_shovel_iron_idle.clone(),
                DwarfResource::Stone => handles.dwarf_parts_shovel_stone_idle.clone(),
            },
            DwarfTool::Pickaxe | DwarfTool::MultiTool => match dwarf_resource {
                DwarfResource::Gold => handles.dwarf_parts_pickaxe_gold_idle.clone(),
                DwarfResource::Iron => handles.dwarf_parts_pickaxe_iron_idle.clone(),
                DwarfResource::Stone => handles.dwarf_parts_pickaxe_stone_idle.clone(),
            },
            DwarfTool::Dynamite => handles.dwarf_parts_dynamite_idle.clone(),
        },
        DwarfAction::Moving => match dwarf_tool {
            DwarfTool::BareHands => handles.dwarf_parts_barehands_moving.clone(),
            DwarfTool::Shovel => match dwarf_resource {
                DwarfResource::Gold => handles.dwarf_parts_shovel_gold_moving.clone(),
                DwarfResource::Iron => handles.dwarf_parts_shovel_iron_moving.clone(),
                DwarfResource::Stone => handles.dwarf_parts_shovel_stone_moving.clone(),
            },
            DwarfTool::Pickaxe | DwarfTool::MultiTool => match dwarf_resource {
                DwarfResource::Gold => handles.dwarf_parts_pickaxe_gold_moving.clone(),
                DwarfResource::Iron => handles.dwarf_parts_pickaxe_iron_moving.clone(),
                DwarfResource::Stone => handles.dwarf_parts_pickaxe_stone_moving.clone(),
            },
            DwarfTool::Dynamite => handles.dwarf_parts_dynamite_moving.clone(),
        },
        DwarfAction::Jump => match dwarf_tool {
            DwarfTool::BareHands => handles.dwarf_parts_barehands_jump.clone(),
            DwarfTool::Shovel => match dwarf_resource {
                DwarfResource::Gold => handles.dwarf_parts_shovel_gold_jump.clone(),
                DwarfResource::Iron => handles.dwarf_parts_shovel_iron_jump.clone(),
                DwarfResource::Stone => handles.dwarf_parts_shovel_stone_jump.clone(),
            },
            DwarfTool::Pickaxe | DwarfTool::MultiTool => match dwarf_resource {
                DwarfResource::Gold => handles.dwarf_parts_pickaxe_gold_jump.clone(),
                DwarfResource::Iron => handles.dwarf_parts_pickaxe_iron_jump.clone(),
                DwarfResource::Stone => handles.dwarf_parts_pickaxe_stone_jump.clone(),
            },
            DwarfTool::Dynamite => handles.dwarf_parts_dynamite_jump.clone(),
        },
        DwarfAction::LightLanding => match dwarf_tool {
            DwarfTool::BareHands => handles.dwarf_parts_barehands_lightlanding.clone(),
            DwarfTool::Shovel => match dwarf_resource {
                DwarfResource::Gold => handles.dwarf_parts_shovel_gold_lightlanding.clone(),
                DwarfResource::Iron => handles.dwarf_parts_shovel_iron_lightlanding.clone(),
                DwarfResource::Stone => handles.dwarf_parts_shovel_stone_lightlanding.clone(),
            },
            DwarfTool::Pickaxe | DwarfTool::MultiTool => match dwarf_resource {
                DwarfResource::Gold => handles.dwarf_parts_pickaxe_gold_lightlanding.clone(),
                DwarfResource::Iron => handles.dwarf_parts_pickaxe_iron_lightlanding.clone(),
                DwarfResource::Stone => handles.dwarf_parts_pickaxe_stone_lightlanding.clone(),
            },
            DwarfTool::Dynamite => handles.dwarf_parts_dynamite_lightlanding.clone(),
        },
        DwarfAction::HeavyLanding => match dwarf_tool {
            DwarfTool::BareHands => handles.dwarf_parts_barehands_heavylanding.clone(),
            DwarfTool::Shovel => match dwarf_resource {
                DwarfResource::Gold => handles.dwarf_parts_shovel_gold_heavylanding.clone(),
                DwarfResource::Iron => handles.dwarf_parts_shovel_iron_heavylanding.clone(),
                DwarfResource::Stone => handles.dwarf_parts_shovel_stone_heavylanding.clone(),
            },
            DwarfTool::Pickaxe | DwarfTool::MultiTool => match dwarf_resource {
                DwarfResource::Gold => handles.dwarf_parts_pickaxe_gold_heavylanding.clone(),
                DwarfResource::Iron => handles.dwarf_parts_pickaxe_iron_heavylanding.clone(),
                DwarfResource::Stone => handles.dwarf_parts_pickaxe_stone_heavylanding.clone(),
            },
            DwarfTool::Dynamite => handles.dwarf_parts_dynamite_heavylanding.clone(),
        },
        DwarfAction::StandUp => match dwarf_tool {
            DwarfTool::BareHands => handles.dwarf_parts_barehands_standup.clone(),
            DwarfTool::Shovel => match dwarf_resource {
                DwarfResource::Gold => handles.dwarf_parts_shovel_gold_standup.clone(),
                DwarfResource::Iron => handles.dwarf_parts_shovel_iron_standup.clone(),
                DwarfResource::Stone => handles.dwarf_parts_shovel_stone_standup.clone(),
            },
            DwarfTool::MultiTool | DwarfTool::Pickaxe => match dwarf_resource {
                DwarfResource::Gold => handles.dwarf_parts_pickaxe_gold_standup.clone(),
                DwarfResource::Iron => handles.dwarf_parts_pickaxe_iron_standup.clone(),
                DwarfResource::Stone => handles.dwarf_parts_pickaxe_stone_standup.clone(),
            },
            DwarfTool::Dynamite => handles.dwarf_parts_dynamite_standup.clone(),
        },
        DwarfAction::Shoveling => match dwarf_tool {
            DwarfTool::Shovel => match dwarf_resource {
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
        DwarfAction::Climbing => match dwarf_tool {
            DwarfTool::BareHands => handles.dwarf_parts_barehands_climbing.clone(),
            _ => unimplemented!(),
        },
        DwarfAction::Light => match dwarf_tool {
            DwarfTool::Dynamite => handles.dwarf_parts_dynamite_light.clone(),
            _ => unimplemented!(),
        },
        DwarfAction::Throw => match dwarf_tool {
            DwarfTool::Dynamite => handles.dwarf_parts_dynamite_throw.clone(),
            _ => unimplemented!(),
        },
        DwarfAction::Swing => match dwarf_tool {
            DwarfTool::Pickaxe | DwarfTool::MultiTool => match dwarf_resource {
                DwarfResource::Gold => handles.dwarf_parts_pickaxe_gold_swing.clone(),
                DwarfResource::Iron => handles.dwarf_parts_pickaxe_iron_swing.clone(),
                DwarfResource::Stone => handles.dwarf_parts_pickaxe_stone_swing.clone(),
            },
            _ => unimplemented!(),
        },
    }
}

pub fn update_dwarf_body_animation(
    commands: &mut Commands,
    dwarf_body: Entity,
    dwarf_color: &DwarfColor,
    dwarf_action: &DwarfAction,
    handles: &AssetHandles,
) {
    let new_aseprite = clone_dwarf_body_animation(dwarf_color, dwarf_action, handles);
    commands.entity(dwarf_body).insert(AseAnimation {
        animation: Animation::default(),
        aseprite: new_aseprite,
    });
}

pub fn update_dwarf_parts_animation(
    commands: &mut Commands,
    dwarf_parts: Entity,
    dwarf_action: &DwarfAction,
    dwarf_tool: &DwarfTool,
    dwarf_resource: &DwarfResource,
    handles: &AssetHandles,
) {
    let new_aseprite =
        clone_dwarf_parts_animation(dwarf_action, dwarf_tool, dwarf_resource, handles);
    commands.entity(dwarf_parts).insert(AseAnimation {
        animation: Animation::default(),
        aseprite: new_aseprite,
    });
}
