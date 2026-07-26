use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use crate::entities::lock::*;

#[derive(Bundle, LdtkEntity, Default)]
pub struct Door {
    #[sprite_sheet]
    sprite: Sprite,
    #[grid_coords]
    grid_coords: GridCoords,
    tag: DoorTag,
    #[with(DoorType::from_field)]
    door_type: DoorType,
    #[with(Lock::from_field)]
    locked: Lock,
}

#[derive(Component, Default)]
pub struct DoorTag;

#[derive(Component, Default, Reflect, PartialEq, Eq)]
pub enum DoorType {
    #[default]
    HorizontalWallLeftSquare,
    HorizontalWallRightSquare,
    HorizontalWallLeftRounded,
    HorizontalWallRightRounded,
    VerticalWallUp,
    VerticalWallDown,
    LeftWallUp,
    LeftWallDown,
    RightWallUp,
    RightWallDown,
    Floor,
}

impl DoorType {
    pub fn from_field(entity_instance: &EntityInstance) -> Self {
        match entity_instance
            .get_enum_field("DoorType")
            .map(String::as_str)
        {
            Ok("HorizontalWallLeftSquare") => Self::HorizontalWallLeftSquare,
            Ok("HorizontalWallRightSquare") => Self::HorizontalWallRightSquare,
            Ok("HorizontalWallLeftRounded") => Self::HorizontalWallLeftRounded,
            Ok("HorizontalWallRightRounded") => Self::HorizontalWallRightRounded,
            Ok("VerticalWallUp") => Self::VerticalWallUp,
            Ok("VerticalWallDown") => Self::VerticalWallDown,
            Ok("LeftWallUp") => Self::LeftWallUp,
            Ok("LeftWallDown") => Self::LeftWallDown,
            Ok("RightWallUp") => Self::RightWallUp,
            Ok("RightWallDown") => Self::RightWallDown,
            Ok("Floor") => Self::Floor,
            _ => Self::default(),
        }
    }
}

pub struct DoorEntityPlugin;

impl Plugin for DoorEntityPlugin {
    fn build(&self, app: &mut App) {
        app.register_ldtk_entity::<Door>("Door");
    }
}
