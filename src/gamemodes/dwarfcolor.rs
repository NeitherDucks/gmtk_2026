use bevy::prelude::App;
use bevy::prelude::Component;
use bevy::prelude::Plugin;
use bevy::prelude::Reflect;
use bevy_ecs_ldtk::EntityInstance;
use bevy_ecs_ldtk::prelude::ldtk::ldtk_fields::*;

#[derive(Component, Clone, Eq, PartialEq, Hash, Debug, Default)]
pub struct DwarfColorComponent(pub DwarfColor);

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default, Reflect)]
pub enum DwarfColor {
    #[default]
    Blue,
    Red,
    Yellow,
    Purple,
}

impl DwarfColorComponent {
    pub fn from_field(entity_instance: &EntityInstance) -> Self {
        match entity_instance
            .get_enum_field("DwarfColor")
            .map(String::as_str)
        {
            Ok("Blue") => DwarfColorComponent(DwarfColor::Blue),
            Ok("Red") => DwarfColorComponent(DwarfColor::Red),
            Ok("Yellow") => DwarfColorComponent(DwarfColor::Yellow),
            Ok("Purple") => DwarfColorComponent(DwarfColor::Purple),
            _ => DwarfColorComponent(DwarfColor::Blue),
        }
    }
}

pub struct DwarfColorPlugin;

impl Plugin for DwarfColorPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<DwarfColor>();
    }
}
