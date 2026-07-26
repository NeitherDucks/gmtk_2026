use crate::entities::traptype::TrapType;
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

#[derive(Bundle, LdtkEntity, Default)]
pub struct Trap {
    #[sprite_sheet]
    pub sprite: Sprite,
    #[grid_coords]
    pub grid_coords: GridCoords,
    pub tag: TrapTag,
    #[with(TrapType::from_field)]
    pub trap: TrapType,
}

#[derive(Default, Component)]
pub struct TrapTag;

pub struct TrapEntityPlugin;

impl Plugin for TrapEntityPlugin {
    fn build(&self, app: &mut App) {
        app.register_ldtk_entity::<Trap>("Trap");
    }
}
