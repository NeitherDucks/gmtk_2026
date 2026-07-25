use std::collections::{HashSet, VecDeque};

use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

pub mod dwarf;
mod editing;
mod editing_ui;
mod loading;
mod playing;
mod playing_ui;

pub struct GameModesPlugin;

impl Plugin for GameModesPlugin {
    fn build(&self, app: &mut App) {
        app.register_ldtk_entity::<Chest>("Chest")
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

/// Bundle to use for the "Chest" LdtkEntity
#[derive(Bundle, LdtkEntity, Default)]
pub struct Chest {
    #[sprite_sheet]
    sprite: Sprite,
    #[grid_coords]
    grid_coords: GridCoords,
}

#[derive(Default, Component)]
pub struct GoalTag;

#[derive(Default, Component)]
pub struct WallTag;

#[derive(Default, Bundle, LdtkIntCell)]
struct WallBundle {
    wall: WallTag,
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

#[derive(Default, Component)]
pub struct ChestTag;

#[derive(Default, Bundle, LdtkIntCell)]
struct ChestBundle {
    chest: Chest,
}

#[derive(Default, Resource, Debug)]
pub struct LevelChests {
    chest_locations: HashSet<GridCoords>,
    level_width: i32,
    level_height: i32,
}

impl LevelChests {
    fn at_item(&self, grid_coords: &GridCoords) -> bool {
        self.chest_locations.contains(grid_coords)
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum DwarfDirection {
    Up,
    Down,
    Left,
    Right,
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

#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
pub enum DwarfColor {
    Blue,
    Red,
    Yellow,
    Purple,
}

#[derive(Clone, Copy, PartialEq)]
pub enum DwarfResource {
    Stone, // only with shovel, pickaxe, multitool
    Iron,  // only with shovel, pickaxe, multitool
    Gold,  // only with shovel, pickaxe
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum DwarfTool {
    BareHands,
    MultiTool,
    Shovel,
    Pickaxe,
    Dynamite,
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Traps {
    Up,
    Left,
    Down,
    Right,
    Catapult,
    Rock,
}

impl TryFrom<String> for Traps {
    type Error = &'static str;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "Up" => Ok(Traps::Up),
            "Down" => Ok(Traps::Down),
            "Left" => Ok(Traps::Left),
            "Right" => Ok(Traps::Right),
            "Catapult" => Ok(Traps::Catapult),
            "Rock" => Ok(Traps::Rock),
            _ => Err("Unknown trap"),
        }
    }
}

#[derive(Debug, Resource)]
pub struct Hand(Vec<(Traps, u32)>);
