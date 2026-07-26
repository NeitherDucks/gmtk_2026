use bevy::prelude::App;
use bevy::prelude::Plugin;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum DwarfAction {
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
