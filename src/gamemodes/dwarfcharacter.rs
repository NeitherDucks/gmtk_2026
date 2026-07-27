use bevy::prelude::App;
use bevy::prelude::Component;
use bevy::prelude::Entity;
use bevy::prelude::Plugin;
use bevy_ecs_ldtk::GridCoords;

#[derive(Component)]
pub struct DwarfBody;

#[derive(Component)]
pub struct DwarfParts;

#[derive(Component, Clone, Copy, Debug)]
pub struct DwarfCharacter {
    pub grid_coords: GridCoords,
    pub body: Entity,
    pub parts: Entity,
    pub move_distance: f32, // distance moved in current Moving action
}

pub struct DwarfCharacterPlugin;

impl Plugin for DwarfCharacterPlugin {
    fn build(&self, _app: &mut App) {}
}
