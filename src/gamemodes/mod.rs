use std::collections::{HashMap, HashSet, VecDeque};

use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use crate::entities::traptype::TrapType;

pub mod dwarf;
mod editing;
mod editing_ui;
mod loading;
mod playing;
mod playing_ui;

pub struct GameModesPlugin;

impl Plugin for GameModesPlugin {
    fn build(&self, app: &mut App) {
        app //
            .register_type::<DwarfColor>()
            .register_type::<DwarfTool>()
            .insert_resource(LdtkSettings {
                int_grid_rendering: IntGridRendering::Invisible,
                set_clear_color: SetClearColor::FromLevelBackground,
                ..Default::default()
            })
            .add_plugins((
                loading::LoadingGameModePlugin,
                editing::EditingGameModePlugin,
                editing_ui::EditingUIGameModePlugin,
                playing::PlayingGameModePlugin,
                playing_ui::PlayingUIGameModePlugin,
            ));
    }
}

#[derive(Default, Component)]
pub struct GoalTag;

#[derive(Debug)]
pub enum Item {
    Wall,
    Chest,
    Trap((Entity, TrapType)),
    Dwarf,
    Door,
}

#[derive(Default, Resource, Debug)]
pub struct Grid {
    items: HashMap<GridCoords, Item>,
    width: i32,
    height: i32,
}

impl Grid {
    pub fn is_outside_level(&self, grid_coords: &GridCoords) -> bool {
        grid_coords.x < 0
            || grid_coords.y < 0
            || grid_coords.x >= self.width
            || grid_coords.y >= self.height
    }

    pub fn is_collision(&self, grid_coords: &GridCoords) -> bool {
        self.is_outside_level(grid_coords) || self.items.contains_key(grid_coords)
    }
}

#[derive(Default, Resource, Debug)]
pub struct LevelWalls {
    wall_locations: HashSet<GridCoords>,
    level_width: i32,
    level_height: i32,
}

impl LevelWalls {
    fn in_wall(&self, grid_coords: &GridCoords) -> bool {
        grid_coords.x < 0
            || grid_coords.y < 0
            || grid_coords.x >= self.level_width
            || grid_coords.y >= self.level_height
            || self.wall_locations.contains(grid_coords)
    }
}

#[derive(Default, Resource, Debug)]
pub struct LevelChests {
    chest_locations: HashSet<GridCoords>,
}

impl LevelChests {
    pub fn at_item(&self, grid_coords: &GridCoords) -> bool {
        self.chest_locations.contains(grid_coords)
    }
}

#[derive(Component, Clone, Copy, PartialEq, Debug, Default)]
pub enum DwarfDirection {
    Up,
    Down,
    Left,
    #[default]
    Right,
}

impl DwarfDirection {
    pub fn from_field(entity_instance: &EntityInstance) -> Self {
        match entity_instance
            .get_enum_field("DwarfColor")
            .map(String::as_str)
        {
            Ok("Up") => Self::Up,
            Ok("Down") => Self::Down,
            Ok("Left") => Self::Left,
            Ok("Right") => Self::Right,
            _ => Self::default(),
        }
    }
}

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

#[derive(Component, Clone, Copy, Eq, PartialEq, Hash, Debug, Default, Reflect)]
pub enum DwarfColor {
    #[default]
    Blue,
    Red,
    Yellow,
    Purple,
}

impl DwarfColor {
    pub fn from_field(entity_instance: &EntityInstance) -> Self {
        match entity_instance
            .get_enum_field("DwarfColor")
            .map(String::as_str)
        {
            Ok("Blue") => DwarfColor::Blue,
            Ok("Red") => DwarfColor::Red,
            Ok("Yellow") => DwarfColor::Yellow,
            Ok("Purple") => DwarfColor::Purple,
            _ => DwarfColor::Blue,
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum DwarfResource {
    Stone, // only with shovel, pickaxe, multitool
    Iron,  // only with shovel, pickaxe, multitool
    Gold,  // only with shovel, pickaxe
}

#[derive(Clone, Copy, Component, PartialEq, Eq, Debug, Default, Reflect)]
pub enum DwarfTool {
    #[default]
    BareHands,
    MultiTool,
    Shovel,
    Pickaxe,
    Dynamite,
}

impl DwarfTool {
    pub fn from_field(entity_instance: &EntityInstance) -> Self {
        match entity_instance
            .get_enum_field("DwarfTool")
            .map(String::as_str)
        {
            Ok("BareHands") => DwarfTool::BareHands,
            Ok("MultiTool") => DwarfTool::MultiTool,
            Ok("Shovel") => DwarfTool::Shovel,
            Ok("Pickaxe") => DwarfTool::Pickaxe,
            Ok("Dynamite") => DwarfTool::Dynamite,
            _ => DwarfTool::BareHands,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum DwarfActionRequest {
    MoveForward,
    ChangeTool(DwarfTool),
    ChangeDirection(DwarfDirection),
    TakeAction(DwarfAction),
    ChangeColor(DwarfColor),
}

#[derive(Resource, Default)]
pub struct ActionsRequested(pub VecDeque<DwarfActionRequest>);

#[derive(Resource, Clone)]
pub struct DwarfCharacter {
    grid_coords: GridCoords,
    color: DwarfColor,
    action: DwarfAction,
    direction: DwarfDirection,
    tool: DwarfTool,
    resource: DwarfResource,
    body: Entity,
    parts: Entity,
    move_distance: f32, // distance moved in current Moving action
}

#[derive(Debug, Resource)]
pub struct Hand(Vec<(TrapType, u32)>);

impl Hand {
    pub fn increment(&mut self, trap: &TrapType) -> Option<u32> {
        for (hand_trap, amount) in &mut self.0 {
            if hand_trap == trap {
                *amount = amount.saturating_add(1);

                return Some(*amount);
            }
        }

        None
    }

    pub fn decrement(&mut self, trap: &TrapType) -> Option<u32> {
        for (hand_trap, amount) in &mut self.0 {
            if hand_trap == trap {
                *amount = amount.saturating_sub(1);

                return Some(*amount);
            }
        }

        None
    }
}
