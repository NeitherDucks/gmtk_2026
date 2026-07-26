use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

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
    pub fn from_field(entity_instance: &EntityInstance) -> Self {
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
