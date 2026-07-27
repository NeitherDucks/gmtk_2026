use bevy::prelude::App;
use bevy::prelude::Component;
use bevy::prelude::Plugin;
use bevy_ecs_ldtk::EntityInstance;
use bevy_ecs_ldtk::prelude::ldtk::ldtk_fields::*;

#[derive(Component, Clone, Copy, PartialEq, Debug, Default)]
pub struct DwarfDirectionComponent(pub DwarfDirection);

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub enum DwarfDirection {
    Up,
    Down,
    Left,
    #[default]
    Right,
}

impl DwarfDirectionComponent {
    pub fn from_field(entity_instance: &EntityInstance) -> Self {
        match entity_instance
            .get_enum_field("DwarfColor") // TODO: should be DwarfDirection
            .map(String::as_str)
        {
            Ok("Up") => DwarfDirectionComponent(DwarfDirection::Up),
            Ok("Down") => DwarfDirectionComponent(DwarfDirection::Down),
            Ok("Left") => DwarfDirectionComponent(DwarfDirection::Left),
            Ok("Right") => DwarfDirectionComponent(DwarfDirection::Right),
            _ => DwarfDirectionComponent(DwarfDirection::default()),
        }
    }
}

pub struct DwarfDirectionPlugin;

impl Plugin for DwarfDirectionPlugin {
    fn build(&self, _app: &mut App) {}
}
