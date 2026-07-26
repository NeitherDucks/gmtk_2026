use bevy::prelude::App;
use bevy::prelude::Plugin;

#[derive(Clone, Copy, PartialEq)]
pub enum DwarfResource {
    Stone, // only with shovel, pickaxe, multitool
    Iron,  // only with shovel, pickaxe, multitool
    Gold,  // only with shovel, pickaxe
}

pub struct DwarfResourcePlugin;

impl Plugin for DwarfResourcePlugin {
    fn build(&self, _app: &mut App) {}
}
