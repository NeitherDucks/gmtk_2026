use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use crate::entities::item::Item;

/// Bundle to use for the "Chest" LdtkEntity
#[derive(Bundle, LdtkEntity, Default)]
pub struct Chest {
    #[sprite_sheet]
    sprite: Sprite,
    #[grid_coords]
    grid_coords: GridCoords,
    tag: ChestTag,
    #[with(ChestLoot::from_field)]
    loot: ChestLoot,
}

#[derive(Debug, Default, Component, Reflect)]
pub struct ChestLoot(pub Vec<Item>);

impl ChestLoot {
    pub fn from_field(entity_instance: &EntityInstance) -> Self {
        if let Ok(loot) = entity_instance.get_maybe_enums_field("Loot") {
            ChestLoot(loot.iter().flatten().map(|v| v.into()).collect())
        } else {
            ChestLoot(Vec::new())
        }
    }
}

#[derive(Default, Component)]
pub struct ChestTag;

pub struct ChestEntityPlugin;

impl Plugin for ChestEntityPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<ChestLoot>()
            .register_ldtk_entity::<Chest>("Chest");
    }
}
