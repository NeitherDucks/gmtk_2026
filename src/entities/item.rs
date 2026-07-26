use bevy::prelude::*;

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
