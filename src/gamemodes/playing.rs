use std::collections::VecDeque;

use bevy::prelude::*;

use crate::LevelState;
use crate::asset_loading::AssetHandles;
use crate::entities::{GoalTag, StartingPointTag};
use crate::gamemodes::dwarf::{update_dwarf_body_animation, update_dwarf_parts_animation};
use crate::gamemodes::{
    DwarfAction, DwarfActionComponent, DwarfActionRequest, DwarfCharacter, DwarfColor,
    DwarfColorComponent, DwarfDirection, DwarfDirectionComponent, DwarfResource,
    DwarfResourceComponent, DwarfTool, DwarfToolComponent, Requests,
};

use bevy_ecs_ldtk::prelude::*;

const DWARF_MOVE_SPEED: f32 = 48.0;
const TILE_SIZE: i32 = 16;

pub struct PlayingGameModePlugin;

impl Plugin for PlayingGameModePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(LevelState::Playing), (setup, spawn_dwarves));
        app.add_systems(Update, check_goal.run_if(in_state(LevelState::Playing)));
        app.add_systems(
            Update,
            ((dev_input, process_action_request).chain()).run_if(in_state(LevelState::Playing)),
        );
        app.add_systems(OnExit(LevelState::Playing), cleanup);
    }
}

fn setup() {}

pub fn spawn_dwarves(
    mut commands: Commands,
    starting_points: Query<
        (
            &GridCoords,
            &DwarfColorComponent,
            &DwarfToolComponent,
            &DwarfDirectionComponent,
        ),
        With<StartingPointTag>,
    >,
) {
    // go through StartingSpots in the level, add a Dwarf for each one with ActionRequest(DwarfActionRequest::MoveForward) added to their deque
    for (coords, dwarf_color, dwarf_tool, dwarf_direction) in starting_points.iter() {
        const BODY_Z: f32 = 10.0_f32;

        let (tx, ty) = (
            coords.x * TILE_SIZE + TILE_SIZE / 2,
            coords.y * TILE_SIZE + TILE_SIZE / 2,
        );

        println!("put a dwarf at {}, {}", coords.x, coords.y);
        let body_action = DwarfAction::Idle;
        let resource = DwarfResource::Gold;

        let body_id = commands.spawn((Sprite::default(),)).id();
        let parts_id = commands
            .spawn((
                Sprite::default(),
                Transform::from_translation(Vec3::new(0., 0., 1.)),
            ))
            .id();
        let character_id = commands
            .spawn((
                DwarfCharacter {
                    grid_coords: *coords,
                    body: body_id,
                    parts: parts_id,
                    move_distance: 0.,
                },
                Transform::from_translation(Vec3::new(tx as f32, ty as f32, BODY_Z)),
                DwarfActionComponent(body_action),
                DwarfColorComponent(dwarf_color.0),
                DwarfToolComponent(dwarf_tool.0),
                DwarfDirectionComponent(dwarf_direction.0),
                DwarfResourceComponent(resource),
                Requests(VecDeque::from([
                    // test set of initial requests to complete when spawned
                    DwarfActionRequest::ChangeColor(DwarfColor::Purple),
                    DwarfActionRequest::ChangeTool(DwarfTool::Pickaxe),
                    DwarfActionRequest::ChangeTool(DwarfTool::Dynamite),
                    DwarfActionRequest::ChangeDirection(DwarfDirection::Left),
                    DwarfActionRequest::MoveForward,
                    DwarfActionRequest::ChangeColor(DwarfColor::Red),
                ])),
            ))
            .id();
        commands
            .entity(character_id)
            .add_children(&[body_id, parts_id]);
        //break;  // hack for just 1 dwarf
    }
}

#[derive(Component)]
struct ActionCompleted;

// for each DwarfCharacter and Request component, progress current action request; if complete current action, pop next request and start it
fn process_action_request(
    mut commands: Commands,
    mut query: Query<(
        Entity,
        &mut DwarfCharacter,
        &mut Transform,
        &mut Requests,
        &DwarfColorComponent,
        &mut DwarfActionComponent,
        &DwarfDirectionComponent,
        &DwarfResourceComponent,
        Option<&ActionCompleted>,
    )>,
    handles: Res<AssetHandles>,
    time: Res<Time>,
) {
    let dt = time.delta_secs();

    for (
        e,
        mut dwarf,
        mut current_transform,
        mut reqs,
        current_color,
        mut current_action,
        current_direction,
        current_resource,
        completed_opt,
    ) in query.iter_mut()
    {
        let mut completed = completed_opt.is_some();
        if let Some(first_request) = reqs.0.front() {
            // have requests to do
            if !completed {
                //println!("{:?} not complete for {:?}", first_request, dwarf);

                // advance first_request; if completed, add ActionCompleted component
                // TODO: decide if request is for same color/direction/tool, do we do anything? presently, yes
                //let request = first_request;
                match first_request {
                    DwarfActionRequest::ChangeColor(new_color) => {
                        completed = true;
                        commands.entity(e).insert(DwarfColorComponent(*new_color));
                        update_dwarf_body_animation(
                            &mut commands,
                            dwarf.body,
                            new_color,
                            &current_action.0,
                            &handles,
                        );
                    }
                    DwarfActionRequest::ChangeDirection(new_direction) => {
                        completed = true;
                        commands
                            .entity(e)
                            .insert(DwarfDirectionComponent(*new_direction));
                    }
                    DwarfActionRequest::ChangeTool(new_tool) => {
                        completed = true;
                        commands.entity(e).insert(DwarfToolComponent(*new_tool));
                        // TODO: need current action and resource
                        update_dwarf_parts_animation(
                            &mut commands,
                            dwarf.parts,
                            &current_action.0,
                            new_tool,
                            &current_resource.0,
                            &handles,
                        );
                    }
                    DwarfActionRequest::MoveForward => {
                        println!(
                            "MoveForward completed={completed} {:?} {:?}",
                            dwarf, current_action.0
                        );
                        if DwarfAction::Idle == current_action.0 {
                            println!("move to Moving from Idle");
                            current_action.0 = DwarfAction::Moving;
                            dwarf.move_distance = 0.0;
                            update_dwarf_body_animation(
                                &mut commands,
                                dwarf.body,
                                &current_color.0,
                                &DwarfAction::Moving,
                                &handles,
                            );
                        }
                        let grid_movement_direction = match current_direction.0 {
                            DwarfDirection::Up => GridCoords::new(0, 1),
                            DwarfDirection::Down => GridCoords::new(0, -1),
                            DwarfDirection::Left => GridCoords::new(-1, 0),
                            DwarfDirection::Right => GridCoords::new(1, 0),
                        };
                        let destination = dwarf.grid_coords + grid_movement_direction;
                        let can_move_forward = true; // TODO: check walls, doors, etc.
                        if DwarfAction::Moving == current_action.0 && can_move_forward {
                            println!("can move foward {:?}", dwarf);
                            let distance_this_frame = DWARF_MOVE_SPEED * dt;
                            dwarf.move_distance += distance_this_frame;
                            match current_direction.0 {
                                DwarfDirection::Up => {
                                    current_transform.translation.y += distance_this_frame
                                }
                                DwarfDirection::Down => {
                                    current_transform.translation.y -= distance_this_frame
                                }
                                DwarfDirection::Right => {
                                    current_transform.translation.x += distance_this_frame
                                }
                                DwarfDirection::Left => {
                                    current_transform.translation.x -= distance_this_frame
                                }
                            };
                            if current_direction.0 == DwarfDirection::Left {
                                current_transform.scale.x = -1.;
                            } else {
                                current_transform.scale.x = 1.;
                            }
                        }
                        completed = dwarf.move_distance >= TILE_SIZE as f32;
                        if completed {
                            println!("completed");
                            dwarf.grid_coords = destination;
                            dwarf.move_distance = 0.;
                        }
                        update_dwarf_body_animation(
                            &mut commands,
                            dwarf.body,
                            &current_color.0,
                            if completed {
                                &DwarfAction::Idle
                            } else {
                                &DwarfAction::Moving
                            },
                            &handles,
                        );
                    }
                    DwarfActionRequest::TakeAction(_new_action) => {
                        completed = false;
                    }
                };

                if completed {
                    commands.entity(e).insert(ActionCompleted);
                    // reset progress to start of actions
                    dwarf.move_distance = 0.0;
                }
            } else {
                //println!("{:?} completed {:?}", first_request, dwarf);
                commands.entity(e).remove::<ActionCompleted>();
                // advance to next request
                reqs.0.pop_front();
            }
        }
    }
}

pub fn dev_input(
    input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(
        Entity,
        &DwarfColorComponent,
        &DwarfActionComponent,
        &DwarfToolComponent,
        &mut Requests,
    )>,
    mut next_state: ResMut<NextState<LevelState>>,
) {
    for (_e, dwarf_color, _dwarf_action, dwarf_tool, mut reqs) in query.iter_mut() {
        if input.just_pressed(KeyCode::KeyC) {
            let next_color = match dwarf_color.0 {
                DwarfColor::Blue => DwarfColor::Purple,
                DwarfColor::Purple => DwarfColor::Red,
                DwarfColor::Red => DwarfColor::Yellow,
                DwarfColor::Yellow => DwarfColor::Blue,
            };
            reqs.0
                .push_back(DwarfActionRequest::ChangeColor(next_color));
        } else if input.just_pressed(KeyCode::KeyT) {
            let next_tool = match dwarf_tool.0 {
                DwarfTool::BareHands => DwarfTool::Shovel,
                DwarfTool::Shovel => DwarfTool::Dynamite,
                DwarfTool::Dynamite => DwarfTool::MultiTool,
                DwarfTool::MultiTool => DwarfTool::Pickaxe,
                DwarfTool::Pickaxe => DwarfTool::BareHands,
            };
            reqs.0.push_back(DwarfActionRequest::ChangeTool(next_tool));
        } else if input.pressed(KeyCode::KeyA) {
            reqs.0
                .push_back(DwarfActionRequest::ChangeDirection(DwarfDirection::Left));
            reqs.0.push_back(DwarfActionRequest::MoveForward);
        } else if input.pressed(KeyCode::KeyD) {
            reqs.0
                .push_back(DwarfActionRequest::ChangeDirection(DwarfDirection::Right));
            reqs.0.push_back(DwarfActionRequest::MoveForward);
        } else if input.just_pressed(KeyCode::KeyX) {
            // TODO: handle other actions, like Shoveling
            let next_action = DwarfAction::Idle;
            reqs.0
                .push_back(DwarfActionRequest::TakeAction(next_action));
        }
    }
    if input.just_pressed(KeyCode::Escape) {
        next_state.set(LevelState::Editing);
    }
}

pub fn check_goal(
    _dwarf: Query<(&DwarfCharacter, &GridCoords)>,
    _goals: Query<&GridCoords, With<GoalTag>>,
) {
    // look through dwarfs to see if their GridCoords are the same as a GoalTag's
}

fn cleanup(mut commands: Commands, dwarves_query: Query<Entity, With<DwarfCharacter>>) {
    for entity in dwarves_query {
        commands.entity(entity).despawn();
    }
}
