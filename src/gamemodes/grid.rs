use std::collections::HashMap;
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use crate::gamemodes::Tile;
use crate::entities::{Lock, TrapType};
use crate::entities::{ChestTag, DoorTag, StartingPointTag, WallTag};


#[derive(Default, Resource, Debug)]
pub struct Grid {
    items: HashMap<GridCoords, Tile>,
    width: i32,
    height: i32,
}

impl Grid {
    pub fn set_size(&mut self, width: i32, height: i32) {
        self.width = width;
        self.height = height;
    }

    pub fn get_items(&self) -> &HashMap<GridCoords, Tile> {
        &self.items
    }
    pub fn get_items_mut(&mut self) -> &mut HashMap<GridCoords, Tile> {
        &mut self.items
    }
    pub fn is_outside_level(&self, grid_coords: &GridCoords) -> bool {
        grid_coords.x < 0
            || grid_coords.y < 0
            || grid_coords.x >= self.width
            || grid_coords.y >= self.height
    }

    pub fn is_collision(&self, grid_coords: &GridCoords) -> bool {
        self.is_outside_level(grid_coords)
            || match self.items.get(grid_coords) {
                Some(Tile::Wall)
                | Some(Tile::Chest)
                | Some(Tile::Dwarf)
                | Some(Tile::Door(Lock::Gold))
                | Some(Tile::Door(Lock::Silver)) => true,
                Some(Tile::Trap((_, trap))) => match trap {
                    TrapType::Nothing
                    | TrapType::Up
                    | TrapType::Left
                    | TrapType::Down
                    | TrapType::Right
                    | TrapType::Catapult => false,
                    TrapType::Rock => true,
                },
                Some(Tile::Door(Lock::Unlocked)) | None => false,
            }
    }

    pub fn is_passable(&self, grid_coords: &GridCoords) -> bool {
        !self.is_collision(grid_coords)
    }
}

pub fn update_added_trap(
    mut grid: ResMut<Grid>,
    query: Query<(Entity, &GridCoords, &TrapType), Added<GridCoords>>,
) {
    for (entity, coord, trap) in &query {
        grid.get_items_mut()
            .insert(*coord, Tile::Trap((entity, *trap)));
    }
}

pub fn update_removed_trap(mut grid: ResMut<Grid>, mut query: RemovedComponents<GridCoords>) {
    query.read().for_each(|entity| {
        grid.get_items_mut().retain(|_, v| {
            if let Tile::Trap((e, _)) = v
                && e == &entity
            {
                false
            } else {
                true
            }
        });
    });
}

/*
pub fn cache_wall_locations(
    query: Query<(&GridCoords, &TileEnumTags), Added<TileEnumTags>>,
    mut level_walls: ResMut<LevelWallCache>,
) {
    for (coords, tag) in query {
        level_walls.wall_locations.insert(*coords);
    }
}
*/

pub fn cache_item_locations(
    mut grid: ResMut<Grid>,
    query: Query<(
        Entity,
        &GridCoords,
        Option<&WallTag>,
        Option<&ChestTag>,
        Option<&DoorTag>,
        Option<&Lock>,
        Option<&StartingPointTag>,
        Option<&TrapType>,
    )>,
) {
    info!("cache_item_locations");
    for (entity, coord, wall, chest, door, lock, start, trap) in &query {
        if wall.is_some() {
            grid.items.insert(*coord, Tile::Wall);
        } else if chest.is_some() {
            grid.items.insert(*coord, Tile::Chest);
        } else if door.is_some() {
            grid.items
                .insert(*coord, Tile::Door(lock.copied().unwrap_or_default()));
        } else if start.is_some() {
            grid.items.insert(*coord, Tile::Dwarf);
        } else if let Some(trap) = trap {
            grid.items.insert(*coord, Tile::Trap((entity, *trap)));
        }
    }
}

/*
pub fn cache_chest_locations(
    mut level_chests: ResMut<LevelChests>,
    chests: Query<&GridCoords, With<ChestTag>>,
) {
    for coords in chests {
        info!("Found chest at: {coords:?}");
        level_chests.chest_locations.insert(*coords);
    }
}
*/

