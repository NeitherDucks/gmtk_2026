use bevy::prelude::*;

mod chest;
mod door;
mod item;
mod lock;
mod startingpoint;
mod trap;
mod traptype;
mod wall;
mod goal;

pub use chest::ChestTag;
pub use door::DoorTag;
pub use startingpoint::StartingPointTag;
pub use trap::{Trap, TrapTag};
pub use traptype::TrapType;
pub use goal::GoalTag;

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
