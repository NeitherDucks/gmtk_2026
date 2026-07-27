use bevy::prelude::Component;
use std::collections::VecDeque;

use crate::gamemodes::DwarfActionRequest;

#[derive(Component, Debug)]
pub struct Requests(pub VecDeque<DwarfActionRequest>);
