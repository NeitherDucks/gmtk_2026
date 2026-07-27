use bevy::prelude::App;
use bevy::prelude::Component;
use bevy::prelude::Plugin;

#[derive(Component, Clone, Copy, PartialEq, Debug, Default)]
pub struct DwarfResourceComponent(pub DwarfResource);

#[derive(Component, Clone, Copy, PartialEq, Debug, Default)]
pub enum DwarfResource {
    #[default]
    Stone, // only with shovel, pickaxe, multitool
    Iron, // only with shovel, pickaxe, multitool
    Gold, // only with shovel, pickaxe
}

pub struct DwarfResourcePlugin;

impl Plugin for DwarfResourcePlugin {
    fn build(&self, _app: &mut App) {}
}
