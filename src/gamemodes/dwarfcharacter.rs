use bevy::prelude::*;
use bevy_ecs_ldtk::GridCoords;
use crate::gamemodes::dwarfcolor::DwarfColor;
use crate::gamemodes::dwarfaction::DwarfAction;
use crate::gamemodes::dwarfdirection::DwarfDirection;
use crate::gamemodes::dwarftool::DwarfTool;
use crate::gamemodes::dwarfresource::DwarfResource;

#[derive(Resource, Clone)]
pub struct DwarfCharacter {
    pub grid_coords: GridCoords,
    pub color: DwarfColor,
    pub action: DwarfAction,
    pub direction: DwarfDirection,
    pub tool: DwarfTool,
    pub resource: DwarfResource,
    pub body: Entity,
    pub parts: Entity,
    pub move_distance: f32, // distance moved in current Moving action
}

pub struct DwarfCharacterPlugin;

impl Plugin for DwarfCharacterPlugin {
    fn build(&self, _app: &mut App) {
    }
}
