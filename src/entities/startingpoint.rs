use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use crate::gamemodes::{DwarfColorComponent, DwarfDirectionComponent, DwarfToolComponent};

#[derive(Bundle, LdtkEntity, Default)]
pub struct StartingPoint {
    #[grid_coords]
    grid_coords: GridCoords,
    tag: StartingPointTag,
    #[with(DwarfColorComponent::from_field)]
    color: DwarfColorComponent,
    #[with(DwarfToolComponent::from_field)]
    tool: DwarfToolComponent,
    #[with(DwarfDirectionComponent::from_field)]
    direction: DwarfDirectionComponent,
}

#[derive(Default, Component)]
pub struct StartingPointTag;

pub struct StartingPointEntityPlugin;

impl Plugin for StartingPointEntityPlugin {
    fn build(&self, app: &mut App) {
        app.register_ldtk_entity::<StartingPoint>("StartingPoint");
    }
}
