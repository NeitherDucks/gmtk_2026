use bevy::prelude::*;

pub mod chest;
pub mod door;
mod item;
mod lock;
pub mod startingpoint;
pub mod trap;
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
