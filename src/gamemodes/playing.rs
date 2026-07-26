use bevy::prelude::*;

use crate::{
    LevelState,
    asset_loading::AssetHandles,
    gamemodes::{
        ActionsRequested, DwarfAction, DwarfActionRequest, DwarfCharacter, DwarfColor,
        DwarfDirection, DwarfTool, LevelWalls,
        dwarf::{update_dwarf_body_animation, update_dwarf_parts_animation},
    },
    entities::GoalTag,
};
use bevy_ecs_ldtk::prelude::*;

const DWARF_MOVE_SPEED: f32 = 50.0;
const TILE_SIZE: i32 = 16;

pub struct PlayingGameModePlugin;

impl Plugin for PlayingGameModePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(LevelState::Playing), setup);
        app.add_systems(
            Update,
            (check_goal, (dev_input, process_action_request).chain())
                .run_if(in_state(LevelState::Playing)),
        );
    }
}

pub fn setup() {}

fn apply_movement(
    dwarf: &mut DwarfCharacter,
    target_direction: DwarfDirection,
    dt: f32,
    transform: &mut Transform,
    level_walls: &LevelWalls,
    apply_movement_fn: impl FnOnce(&mut Transform, f32),
) -> (bool, bool) {
    let mut direction_changed = false;
    let mut action_changed = false;
    // let mut able_to_move_forward = false;

    let grid_movement_direction = match target_direction {
        DwarfDirection::Up => GridCoords::new(0, 1),
        DwarfDirection::Down => GridCoords::new(0, -1),
        DwarfDirection::Left => GridCoords::new(-1, 0),
        DwarfDirection::Right => GridCoords::new(1, 0),
    };

    // can we move forward?
    let destination = dwarf.grid_coords + grid_movement_direction;
    //    println!(
    //        "now at {:?}. move {:?} to {:?}",
    //        dwarf.grid_coords, grid_movement_direction, destination
    //    );
    let able_to_move_forward = !level_walls.in_wall(&destination);

    // Start moving if Idle
    if dwarf.action == DwarfAction::Idle && able_to_move_forward {
        dwarf.action = DwarfAction::Moving;
        dwarf.direction = target_direction;
        dwarf.move_distance = 0.0;
        direction_changed = true;
        action_changed = true;
    }

    // Keep moving if Moving and direction matches
    if dwarf.action == DwarfAction::Moving && dwarf.direction == target_direction {
        let distance_this_frame = DWARF_MOVE_SPEED * dt;
        dwarf.move_distance += distance_this_frame;
        if dwarf.direction == DwarfDirection::Left {
            transform.scale.x = -1.;
        } else {
            transform.scale.x = 1.;
        }

        if dwarf.move_distance >= TILE_SIZE as f32 {
            // Finish Moving exactly one tile's distance
            apply_movement_fn(
                transform,
                TILE_SIZE as f32 - (dwarf.move_distance - distance_this_frame),
            );
            dwarf.action = DwarfAction::Idle;
            dwarf.move_distance = 0.0;
            dwarf.grid_coords = destination;
            action_changed = true;
        } else {
            apply_movement_fn(transform, distance_this_frame);
        }
    }

    (direction_changed, action_changed)
}

pub fn process_action_request(
    mut commands: Commands,
    mut dwarf: If<ResMut<DwarfCharacter>>,
    mut actions_requested: ResMut<ActionsRequested>,
    handles: Res<AssetHandles>,
    time: Res<Time>,
    mut query: Query<&mut Transform>,
    level_walls: Res<LevelWalls>,
) {
    let dt = time.delta_secs();

    if let Some(request) = actions_requested.0.pop_front() {
        // start carrying it out
        let mut direction_changed = false;
        let mut body_action_changed = false;
        let mut tool_changed = false;
        let mut color_changed = false;
        let mut moved = false;

        match request {
            DwarfActionRequest::MoveForward => {
                let current_direction = dwarf.direction;
                if let Ok(mut transform) = query.get_mut(dwarf.body) {
                    let (dir_changed, action_changed) = apply_movement(
                        &mut dwarf,
                        current_direction,
                        dt,
                        &mut transform,
                        &level_walls,
                        |t, dist| match current_direction {
                            DwarfDirection::Up => t.translation.y += dist,
                            DwarfDirection::Down => t.translation.y -= dist,
                            DwarfDirection::Left => t.translation.x -= dist,
                            DwarfDirection::Right => t.translation.x += dist,
                        },
                    );
                    if dir_changed {
                        direction_changed = true;
                    }
                    if action_changed {
                        body_action_changed = true;
                    }
                    moved = true;
                }
            }
            DwarfActionRequest::ChangeTool(new_tool) => {
                if dwarf.tool != new_tool {
                    dwarf.tool = new_tool;
                    tool_changed = true;
                }
            }
            DwarfActionRequest::ChangeDirection(new_direction) => {
                if dwarf.direction != new_direction {
                    dwarf.direction = new_direction;
                    direction_changed = true;
                }
            }
            DwarfActionRequest::TakeAction(new_action) => {
                if dwarf.action != new_action {
                    dwarf.action = new_action;
                    body_action_changed = true;
                }
            }
            DwarfActionRequest::ChangeColor(new_color) => {
                if dwarf.color != new_color {
                    dwarf.color = new_color;
                    color_changed = true;
                }
            }
        }

        if direction_changed || body_action_changed || color_changed {
            update_dwarf_body_animation(&mut commands, &dwarf, &handles);
        }
        if direction_changed || tool_changed {
            update_dwarf_parts_animation(&mut commands, &dwarf, &handles);
        }

        // Keep parts transform in sync with body transform
        if moved {
            let body_pos = query.get(dwarf.body).ok().map(|t| t.translation).unwrap();
            if let Ok(mut parts_transform) = query.get_mut(dwarf.parts) {
                parts_transform.translation = body_pos + Vec3::new(0.0, 0.0, 1.0); // keep parts z one greater than body
            }
        }
    }
}

pub fn dev_input(
    input: Res<ButtonInput<KeyCode>>,
    dwarf: If<Res<DwarfCharacter>>,
    mut actions_requested: ResMut<ActionsRequested>,
    mut next_state: ResMut<NextState<LevelState>>,
) {
    if input.just_pressed(KeyCode::KeyC) {
        let next_color = match dwarf.color {
            DwarfColor::Blue => DwarfColor::Purple,
            DwarfColor::Purple => DwarfColor::Red,
            DwarfColor::Red => DwarfColor::Yellow,
            DwarfColor::Yellow => DwarfColor::Blue,
        };
        actions_requested
            .0
            .push_back(DwarfActionRequest::ChangeColor(next_color));
    } else if input.just_pressed(KeyCode::KeyT) {
        let next_tool = match dwarf.tool {
            DwarfTool::BareHands => DwarfTool::Shovel,
            DwarfTool::Shovel => DwarfTool::Dynamite,
            DwarfTool::Dynamite => DwarfTool::MultiTool,
            DwarfTool::MultiTool => DwarfTool::Pickaxe,
            DwarfTool::Pickaxe => DwarfTool::BareHands,
        };
        actions_requested
            .0
            .push_back(DwarfActionRequest::ChangeTool(next_tool));
    } else if input.pressed(KeyCode::KeyW) {
        actions_requested
            .0
            .push_back(DwarfActionRequest::ChangeDirection(DwarfDirection::Up));
        actions_requested
            .0
            .push_back(DwarfActionRequest::MoveForward);
    } else if input.pressed(KeyCode::KeyA) {
        actions_requested
            .0
            .push_back(DwarfActionRequest::ChangeDirection(DwarfDirection::Left));
        actions_requested
            .0
            .push_back(DwarfActionRequest::MoveForward);
    } else if input.pressed(KeyCode::KeyS) {
        actions_requested
            .0
            .push_back(DwarfActionRequest::ChangeDirection(DwarfDirection::Down));
        actions_requested
            .0
            .push_back(DwarfActionRequest::MoveForward);
    } else if input.pressed(KeyCode::KeyD) {
        actions_requested
            .0
            .push_back(DwarfActionRequest::ChangeDirection(DwarfDirection::Right));
        actions_requested
            .0
            .push_back(DwarfActionRequest::MoveForward);
    } else if input.just_pressed(KeyCode::KeyX) {
        // TODO: handle other actions, like Shoveling
        let next_action = DwarfAction::Idle;
        actions_requested
            .0
            .push_back(DwarfActionRequest::TakeAction(next_action));
    } else if input.just_pressed(KeyCode::Escape) {
        next_state.set(LevelState::Editing);
    }
}

pub fn check_goal(dwarf: Res<DwarfCharacter>, goals: Query<&GridCoords, With<GoalTag>>) {
    if goals
        .iter()
        .any(|goal_grid_coords| &dwarf.grid_coords == goal_grid_coords)
    {
        println!("found goal at dwarf.grid_coords {:?}", dwarf.grid_coords);
    }
}
