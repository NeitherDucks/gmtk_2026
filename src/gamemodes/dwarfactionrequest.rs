use crate::gamemodes::dwarfaction::DwarfAction;
use crate::gamemodes::dwarfcolor::DwarfColor;
use crate::gamemodes::dwarfdirection::DwarfDirection;
use crate::gamemodes::dwarftool::DwarfTool;

#[derive(Clone, Copy, Debug)]
pub enum DwarfActionRequest {
    MoveForward,
    #[expect(unused)]
    ChangeTool(DwarfTool),
    ChangeDirection(DwarfDirection),
    #[expect(unused)]
    TakeAction(DwarfAction),
    #[expect(unused)]
    ChangeColor(DwarfColor),
}
