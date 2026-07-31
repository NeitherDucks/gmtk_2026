use std::collections::VecDeque;

use bevy::prelude::*;

use crate::LevelState;
use crate::asset_loading::AssetHandles;
use crate::entities::{GoalTag, Lock, StartingPointTag, TrapType};
use crate::gamemodes::dwarf::{update_dwarf_body_animation, update_dwarf_parts_animation};
use crate::gamemodes::{
    DwarfAction, DwarfActionComponent, DwarfActionRequest, DwarfCharacter, DwarfColorComponent,
    DwarfDirection, DwarfDirectionComponent, DwarfResource, DwarfResourceComponent,
    DwarfToolComponent, Requests, Tile, grid::Grid,
};

use bevy_ecs_ldtk::prelude::*;

const DWARF_MOVE_SPEED: f32 = 48.0;
const GRID_SIZE: i32 = 16;
const TILE_SIZE: i32 = 16;

pub struct PlayingGameModePlugin;

impl Plugin for PlayingGameModePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(LevelState::Playing),
            (setup, spawn_dwarves, begin_action_generation).chain(),
        );
        app.add_systems(Update, check_goal.run_if(in_state(LevelState::Playing)));
        app.add_systems(
            Update,
            (
                dev_input,
                player_input,
                tick_action_generation_timer.run_if(resource_exists::<ActionGenerationTimer>),
                process_action_request,
            )
                .run_if(in_state(LevelState::Playing))
                .chain(),
        );
        app.add_systems(
            OnExit(LevelState::Playing),
            (end_action_generation, cleanup),
        );
    }
}

fn setup() {}

#[derive(Resource, Default)]
pub struct ActionGenerationTimer(Timer);

impl ActionGenerationTimer {
    pub fn new() -> Self {
        Self(Timer::from_seconds(1.5, TimerMode::Repeating))
    }
}

fn tick_action_generation_timer(
    time: Res<Time>,
    mut action_timer: ResMut<ActionGenerationTimer>,
    dwarves_query: Query<&mut Requests>,
) {
    action_timer.0.tick(time.delta());

    if action_timer.0.just_finished() {
        for mut reqs in dwarves_query {
            reqs.0.push_back(DwarfActionRequest::MoveForward);
        }
    }
}

fn begin_action_generation(mut commands: Commands) {
    commands.insert_resource(ActionGenerationTimer::new());
}

fn end_action_generation(mut commands: Commands) {
    commands.remove_resource::<ActionGenerationTimer>();
}

fn spawn_dwarves(
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
    handles: Res<AssetHandles>,
) {
    // go through StartingSpots in the level, add a Dwarf for each one with ActionRequest(DwarfActionRequest::MoveForward) added to their deque
    for (coords, dwarf_color, dwarf_tool, dwarf_direction) in starting_points.iter() {
        const BODY_Z: f32 = 10.0_f32;

        let t = bevy_ecs_ldtk::utils::grid_coords_to_translation(*coords, IVec2::splat(GRID_SIZE));

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
                Transform::from_translation(Vec3::new(t.x as f32, t.y as f32, BODY_Z)),
                DwarfActionComponent(body_action),
                DwarfColorComponent(dwarf_color.0),
                DwarfToolComponent(dwarf_tool.0),
                DwarfDirectionComponent(dwarf_direction.0),
                DwarfResourceComponent(resource),
                Requests(VecDeque::default()),
            ))
            .id();
        info!(
            "put dwarf {:?} at {}, {} = {:?}",
            character_id, coords.x, coords.y, t
        );
        commands
            .entity(character_id)
            .add_children(&[body_id, parts_id]);
        update_dwarf_body_animation(
            &mut commands,
            body_id,
            &dwarf_color.0,
            &body_action,
            &handles,
        );
        update_dwarf_parts_animation(
            &mut commands,
            parts_id,
            &body_action,
            &dwarf_tool.0,
            &resource,
            &handles,
        );
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
    grid: Res<Grid>,
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
                        if DwarfAction::Idle == current_action.0 {
                            info!("{:?} move to Moving from Idle", dwarf);
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
                        let mut cannot_move_forward = false;
                        let at_destination = grid.get_tile(&destination);
                        if let Some(dest_tile) = at_destination {
                            match dest_tile {
                                Tile::Wall => cannot_move_forward = true,
                                Tile::Chest => cannot_move_forward = true,
                                Tile::Dwarf => cannot_move_forward = true,
                                Tile::Door(lock) => {
                                    cannot_move_forward = match lock {
                                        Lock::Unlocked => false,
                                        Lock::Silver => true,
                                        Lock::Gold => true,
                                    }
                                }
                                Tile::Trap((_e, _traptype)) => {}
                            }
                        }

                        let can_move_forward =
                            !cannot_move_forward && grid.is_passable(&destination); // TODO: Check if Dwarf has key

                        if DwarfAction::Moving == current_action.0 && can_move_forward {
                            let distance_this_frame = DWARF_MOVE_SPEED * dt;
                            dwarf.move_distance += distance_this_frame;

                            match current_direction.0 {
                                DwarfDirection::Up => {
                                    current_transform.translation.y += distance_this_frame;
                                }
                                DwarfDirection::Down => {
                                    current_transform.translation.y -= distance_this_frame;
                                }
                                DwarfDirection::Right => {
                                    current_transform.translation.x += distance_this_frame;
                                }
                                DwarfDirection::Left => {
                                    current_transform.translation.x -= distance_this_frame;
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
                            info!("{:?} completed {:?}", dwarf, first_request);
                            dwarf.grid_coords = destination;
                            dwarf.move_distance = 0.;
                            // does something happen when we get here?
                            if let Some(Tile::Trap((_e, traptype))) = at_destination {
                                match traptype {
                                    TrapType::Left /*| TrapType::Up */ => {
                                        info!("{:?} turning left", dwarf);
                                        let new_direction = turn_left(current_direction.0);
                                        reqs.0.push_back(DwarfActionRequest::ChangeDirection(
                                            new_direction,
                                        ));
                                    }
                                    TrapType::Right /*| TrapType::Down */ => {
                                        info!("{:?} turning right", dwarf);
                                        let new_direction = turn_right(current_direction.0);
                                        reqs.0.push_back(DwarfActionRequest::ChangeDirection(
                                            new_direction,
                                        ));
                                    }
                                    _ => {}
                                }
                            }
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

fn turn_left(old_dir: DwarfDirection) -> DwarfDirection {
    match old_dir {
        DwarfDirection::Up => DwarfDirection::Right,
        DwarfDirection::Left => DwarfDirection::Down,
        DwarfDirection::Right => DwarfDirection::Up,
        DwarfDirection::Down => DwarfDirection::Left,
    }
}

fn turn_right(old_dir: DwarfDirection) -> DwarfDirection {
    match old_dir {
        DwarfDirection::Up => DwarfDirection::Left,
        DwarfDirection::Left => DwarfDirection::Up,
        DwarfDirection::Right => DwarfDirection::Down,
        DwarfDirection::Down => DwarfDirection::Right,
    }
}

fn dev_input(_input: Res<ButtonInput<KeyCode>>) {}

fn player_input(input: Res<ButtonInput<KeyCode>>, mut next_state: ResMut<NextState<LevelState>>) {
    if input.just_pressed(KeyCode::Escape) {
        next_state.set(LevelState::Editing);
    }
}

fn check_goal(
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
