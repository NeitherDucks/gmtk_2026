use bevy::prelude::App;
use bevy::prelude::Component;
use bevy::prelude::Plugin;

#[derive(Component, Clone, Eq, PartialEq, Hash, Debug, Default)]
pub struct DwarfActionComponent(pub DwarfAction);

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub enum DwarfAction {
    #[default]
    Idle,
    Moving,
    Jump,
    LightLanding,
    HeavyLanding,
    StandUp,
    Shoveling, // only with shovel
    Climbing,  // only bare hands
    Light,     // only dynamite; file for body action is LightDynamite
    Throw,     // only dynamite; file for body action is ThrowDynamite
    Swing,     // only pickaxe or multitool; file for body action is PickaxeSwing
}

pub struct DwarfActionPlugin;

impl Plugin for DwarfActionPlugin {
    fn build(&self, _app: &mut App) {}
}
