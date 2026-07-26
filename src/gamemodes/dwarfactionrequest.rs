use crate::gamemodes::dwarfaction::DwarfAction;
use crate::gamemodes::dwarfcolor::DwarfColor;
use crate::gamemodes::dwarfdirection::DwarfDirection;
use crate::gamemodes::dwarftool::DwarfTool;

#[derive(Clone, Copy, Debug)]
pub enum DwarfActionRequest {
    MoveForward,
    ChangeTool(DwarfTool),
    ChangeDirection(DwarfDirection),
    TakeAction(DwarfAction),
    ChangeColor(DwarfColor),
}
