use bevy::prelude::App;
use bevy::prelude::Component;
use bevy::prelude::Plugin;
use bevy::prelude::Reflect;
use bevy_ecs_ldtk::EntityInstance;
use bevy_ecs_ldtk::prelude::ldtk::ldtk_fields::*;

#[derive(Component, Clone, Copy, Eq, PartialEq, Hash, Debug, Default, Reflect)]
pub enum DwarfColor {
    #[default]
    Blue,
    Red,
    Yellow,
    Purple,
}

impl DwarfColor {
    pub fn from_field(entity_instance: &EntityInstance) -> Self {
        match entity_instance
            .get_enum_field("DwarfColor")
            .map(String::as_str)
        {
            Ok("Blue") => DwarfColor::Blue,
            Ok("Red") => DwarfColor::Red,
            Ok("Yellow") => DwarfColor::Yellow,
            Ok("Purple") => DwarfColor::Purple,
            _ => DwarfColor::Blue,
        }
    }
}

pub struct DwarfColorPlugin;

impl Plugin for DwarfColorPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<DwarfColor>();
    }
}
