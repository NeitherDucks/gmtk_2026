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
        app //
            .register_type::<DwarfColor>()
            .register_type::<DwarfTool>()
            .register_type::<ChestLoot>()
            .register_type::<Lock>()
            .register_ldtk_entity::<Chest>("Chest")
            .register_ldtk_entity::<Door>("Door")
            .register_ldtk_entity::<StartingPoint>("StartingPoint")
            .register_ldtk_entity::<Trap>("Trap")
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

#[derive(Default, Resource, Debug)]
pub struct LevelChests {
    chest_locations: HashSet<GridCoords>,
    // level_width: i32,
    // level_height: i32,
}

impl LevelChests {
    pub fn at_item(&self, grid_coords: &GridCoords) -> bool {
        self.chest_locations.contains(grid_coords)
    }
}

#[derive(Bundle, LdtkEntity, Default)]
pub struct Door {
    #[sprite_sheet]
    sprite: Sprite,
    #[grid_coords]
    grid_coords: GridCoords,
    tag: DoorTag,
    #[with(DoorType::from_field)]
    door_type: DoorType,
    #[with(Lock::from_field)]
    locked: Lock,
}

#[derive(Component, Default)]
pub struct DoorTag;

#[derive(Bundle, LdtkEntity, Default)]
pub struct StartingPoint {
    #[grid_coords]
    grid_coords: GridCoords,
    tag: StarintPointTag,
    #[with(DwarfColor::from_field)]
    color: DwarfColor,
    #[with(DwarfTool::from_field)]
    tool: DwarfTool,
}

#[derive(Default, Component)]
pub struct StarintPointTag;

#[derive(Bundle, LdtkEntity, Default)]
pub struct Trap {
    #[sprite_sheet]
    sprite: Sprite,
    #[grid_coords]
    grid_coords: GridCoords,
    tag: TrapTag,
    #[with(TrapType::from_field)]
    trap: TrapType,
}

#[derive(Default, Component)]
pub struct TrapTag;

#[derive(Component, Default, Debug, PartialEq, Eq, Reflect)]
pub enum Item {
    #[default]
    None,
    SilverKey,
    GoldKey,
    SmallRedPotion,
    LargeRedPotion,
    SmallBluePotion,
    LargeBluePotion,
}

impl From<&String> for Item {
    fn from(value: &String) -> Self {
        match value.as_str() {
            "SilverKey" => Self::SilverKey,
            "GoldKey" => Self::GoldKey,
            "SmallRedPotion" => Self::SmallRedPotion,
            "LargeRedPotion" => Self::LargeRedPotion,
            "SmallBluePotion" => Self::SmallBluePotion,
            "LargeBluePotion" => Self::LargeBluePotion,
            _ => Self::None,
        }
    }
}

#[derive(Component, Default, Reflect, PartialEq, Eq)]
pub enum DoorType {
    #[default]
    HorizontalWallLeftSquare,
    HorizontalWallRightSquare,
    HorizontalWallLeftRounded,
    HorizontalWallRightRounded,
    VerticalWallUp,
    VerticalWallDown,
    LeftWallUp,
    LeftWallDown,
    RightWallUp,
    RightWallDown,
    Floor,
}

impl DoorType {
    pub fn from_field(entity_instance: &EntityInstance) -> Self {
        match entity_instance
            .get_enum_field("DoorType")
            .map(String::as_str)
        {
            Ok("HorizontalWallLeftSquare") => Self::HorizontalWallLeftSquare,
            Ok("HorizontalWallRightSquare") => Self::HorizontalWallRightSquare,
            Ok("HorizontalWallLeftRounded") => Self::HorizontalWallLeftRounded,
            Ok("HorizontalWallRightRounded") => Self::HorizontalWallRightRounded,
            Ok("VerticalWallUp") => Self::VerticalWallUp,
            Ok("VerticalWallDown") => Self::VerticalWallDown,
            Ok("LeftWallUp") => Self::LeftWallUp,
            Ok("LeftWallDown") => Self::LeftWallDown,
            Ok("RightWallUp") => Self::RightWallUp,
            Ok("RightWallDown") => Self::RightWallDown,
            Ok("Floor") => Self::Floor,
            _ => Self::default(),
        }
    }
}

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

#[derive(Default, Component)]
pub struct GoalTag;

#[derive(Default, Component)]
pub struct WallTag;

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

#[derive(Component, Debug, Default, Clone, PartialEq, Eq)]
pub enum TrapType {
    #[default]
    Nothing,
    Up,
    Left,
    Down,
    Right,
    Catapult,
    Rock,
}

impl From<&String> for TrapType {
    fn from(value: &String) -> Self {
        match value.as_str() {
            "Up" => TrapType::Up,
            "Down" => TrapType::Down,
            "Left" => TrapType::Left,
            "Right" => TrapType::Right,
            "Catapult" => TrapType::Catapult,
            "Rock" => TrapType::Rock,
            _ => TrapType::Nothing,
        }
    }
}

impl TrapType {
    fn from_field(entity_instance: &EntityInstance) -> Self {
        if let Ok(trap_name) = entity_instance.get_enum_field("DwarfTool") {
            trap_name.into()
        } else {
            TrapType::Nothing
        }
    }

    pub fn get_tileset_offset(&self) -> (f32, f32) {
        match self {
            TrapType::Up => (0.0, 2.0),
            TrapType::Left => (2.0, 2.0),
            TrapType::Down => (1.0, 2.0),
            TrapType::Right => (3.0, 2.0),
            TrapType::Catapult => (0.0, 0.0),
            TrapType::Rock => (1.0, 0.0),
            TrapType::Nothing => (27.0, 0.0),
        }
    }
}

#[derive(Debug, Resource)]
pub struct Hand(Vec<(TrapType, u32)>);
