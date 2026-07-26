use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use crate::gamemodes::{DwarfColor, DwarfDirection, DwarfTool};

#[derive(Bundle, LdtkEntity, Default)]
pub struct StartingPoint {
    #[grid_coords]
    grid_coords: GridCoords,
    tag: StartingPointTag,
    #[with(DwarfColor::from_field)]
    color: DwarfColor,
    #[with(DwarfTool::from_field)]
    tool: DwarfTool,
    #[with(DwarfDirection::from_field)]
    direction: DwarfDirection,
}

#[derive(Default, Component)]
pub struct StartingPointTag;

pub struct StartingPointEntityPlugin;

impl Plugin for StartingPointEntityPlugin {
    fn build(&self, app: &mut App) {
        app.register_ldtk_entity::<StartingPoint>("StartingPoint");
    }
}
