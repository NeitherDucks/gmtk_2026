use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

#[derive(Component, Debug, Default, Clone, Copy, PartialEq, Eq)]
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
            TrapType::Up => (8.0, 2.0),
            TrapType::Left => (11.0, 2.0),
            TrapType::Down => (9.0, 2.0),
            TrapType::Right => (10.0, 2.0),
            TrapType::Catapult => (0.0, 0.0),
            TrapType::Rock => (1.0, 0.0),
            TrapType::Nothing => (27.0, 0.0),
        }
    }

    pub fn get_rect(&self) -> Rect {
        let (x, y) = self.get_tileset_offset();

        Rect {
            min: Vec2 {
                x: x * 32.0,
                y: y * 32.0,
            },
            max: Vec2 {
                x: (x + 1.0) * 32.0,
                y: (y + 1.0) * 32.0,
            },
        }
    }
}
