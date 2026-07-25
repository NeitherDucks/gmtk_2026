use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::{Animation, AseAnimation, Aseprite};

use crate::{
    asset_loading::AssetHandles,
    gamemodes::{DwarfAction, DwarfCharacter, DwarfColor, DwarfResource, DwarfTool},
};

pub fn clone_dwarf_body_animation(
    dwarf: &DwarfCharacter,
    handles: &AssetHandles,
) -> Handle<Aseprite> {
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

pub fn clone_dwarf_parts_animation(
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

pub fn update_dwarf_body_animation(
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

pub fn update_dwarf_parts_animation(
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
