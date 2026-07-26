use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

#[derive(Component, Default, Reflect, PartialEq, Eq)]
pub enum Lock {
    #[default]
    Unlocked,
    Silver,
    Gold,
}

impl Lock {
    pub fn from_field(entity_instance: &EntityInstance) -> Self {
        match entity_instance.get_enum_field("Lock").map(String::as_str) {
            Ok("Silver") => Self::Silver,
            Ok("Gold") => Self::Gold,
            _ => Self::Unlocked,
        }
    }
}

pub struct LockEntityPlugin;

impl Plugin for LockEntityPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Lock>();
    }
}
