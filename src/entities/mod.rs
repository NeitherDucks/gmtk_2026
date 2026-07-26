use bevy::prelude::*;

pub mod chest;
mod door;
mod item;
mod lock;
mod startingpoint;
mod trap;
pub mod traptype;
mod wall;

use chest::ChestEntityPlugin;
use door::DoorEntityPlugin;
use lock::LockEntityPlugin;
use startingpoint::StartingPointEntityPlugin;
use trap::TrapEntityPlugin;

pub struct EntitiesPlugin;

impl Plugin for EntitiesPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            ChestEntityPlugin,
            DoorEntityPlugin,
            TrapEntityPlugin,
            LockEntityPlugin,
            StartingPointEntityPlugin,
        ));
    }
}
