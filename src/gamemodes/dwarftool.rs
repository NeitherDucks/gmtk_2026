use bevy::prelude::App;
use bevy::prelude::Component;
use bevy::prelude::Plugin;
use bevy::prelude::Reflect;
use bevy_ecs_ldtk::EntityInstance;
use bevy_ecs_ldtk::prelude::ldtk::ldtk_fields::*;

#[derive(Clone, Copy, Component, PartialEq, Eq, Debug, Default)]
pub struct DwarfToolComponent(pub DwarfTool);

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default, Reflect)]
pub enum DwarfTool {
    #[default]
    BareHands,
    MultiTool,
    Shovel,
    Pickaxe,
    Dynamite,
}

impl DwarfToolComponent {
    pub fn from_field(entity_instance: &EntityInstance) -> Self {
        match entity_instance
            .get_enum_field("DwarfTool")
            .map(String::as_str)
        {
            Ok("BareHands") => DwarfToolComponent(DwarfTool::BareHands),
            Ok("MultiTool") => DwarfToolComponent(DwarfTool::MultiTool),
            Ok("Shovel") => DwarfToolComponent(DwarfTool::Shovel),
            Ok("Pickaxe") => DwarfToolComponent(DwarfTool::Pickaxe),
            Ok("Dynamite") => DwarfToolComponent(DwarfTool::Dynamite),
            _ => DwarfToolComponent(DwarfTool::BareHands),
        }
    }
}

pub struct DwarfToolPlugin;

impl Plugin for DwarfToolPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<DwarfTool>();
    }
}
