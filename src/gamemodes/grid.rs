use crate::entities::{Lock, TrapType};
use crate::gamemodes::Tile;
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use std::collections::HashMap;

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

    pub fn get_tile(&self, grid_coords: &GridCoords) -> Option<&Tile> {
        self.items.get(grid_coords)
    }

    pub fn add_to(&mut self, grid_coords: GridCoords, tile: Tile) {
        self.items.insert(grid_coords, tile);
    }

    pub fn keep(&mut self, entity: &Entity) {
        self.items.retain(|_, v| {
            if let Tile::Trap((e, _)) = v
                && e == entity
            {
                false
            } else {
                true
            }
        });
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
