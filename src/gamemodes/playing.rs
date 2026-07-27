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
    dwarfcharacter::{DwarfBody, DwarfParts},
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
        const PARTS_Z: f32 = 11.0_f32;

        let (tx, ty) = (
            coords.x * TILE_SIZE + TILE_SIZE / 2,
            coords.y * TILE_SIZE + TILE_SIZE / 2,
        );

        let dwarf_body_entity = commands
            .spawn((
                DwarfBody,
                Sprite::default(),
                Transform::from_translation(Vec3::new(tx as f32, ty as f32, BODY_Z)),
            ))
            .id();
        let dwarf_parts_entity = commands
            .spawn((
                DwarfParts,
                Sprite::default(),
                Transform::from_translation(Vec3::new(tx as f32, ty as f32, PARTS_Z)),
            ))
            .id();

        println!("put a dwarf at {}, {}", coords.x, coords.y);
        let body_action = DwarfAction::Idle;
        let resource = DwarfResource::Gold;

        commands.spawn((
            DwarfCharacter {
                grid_coords: *coords,
                body: dwarf_body_entity,
                parts: dwarf_parts_entity,
                move_distance: default(),
            },
            DwarfActionComponent(body_action),
            DwarfColorComponent(dwarf_color.0),
            DwarfToolComponent(dwarf_tool.0),
            DwarfDirectionComponent(dwarf_direction.0),
            DwarfResourceComponent(resource),
            Requests(VecDeque::from([
                DwarfActionRequest::ChangeColor(DwarfColor::Purple),
                DwarfActionRequest::ChangeTool(DwarfTool::Pickaxe),
                DwarfActionRequest::ChangeTool(DwarfTool::Dynamite),
                DwarfActionRequest::ChangeDirection(DwarfDirection::Left),
                DwarfActionRequest::MoveForward,
                DwarfActionRequest::ChangeColor(DwarfColor::Red),
            ])),
        ));
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
        &mut Requests,
        &DwarfActionComponent,
        &DwarfResourceComponent,
        Option<&ActionCompleted>,
    )>,
    mut transforms_query: Query<(Entity, &mut Transform)>,
    handles: Res<AssetHandles>,
    time: Res<Time>,
) {
    let dt = time.delta_secs();

    for (e, mut dwarf, mut reqs, current_action, current_resource, completed_opt) in
        query.iter_mut()
    {
        let mut completed = completed_opt.is_some();
        if let Some(first_request) = reqs.0.front() {
            // have requests to do
            if !completed {
                println!("{:?} not complete for {:?}", first_request, dwarf);
                let mut _direction_changed = false;
                let mut _body_action_changed = false;
                let mut _tool_changed = false;
                let mut _color_changed = false;
                let mut moved = false;

                // advance first_request; if completed, add ActionCompleted component
                // TODO: decide if request is for same color/direction/tool, do we do anything? presently, yes
                let request = first_request;
                match request {
                    DwarfActionRequest::ChangeColor(new_color) => {
                        _color_changed = true;
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
                        _direction_changed = true;
                        completed = true;
                        commands
                            .entity(e)
                            .insert(DwarfDirectionComponent(*new_direction));
                    }
                    DwarfActionRequest::ChangeTool(new_tool) => {
                        _tool_changed = true;
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
                        moved = true;
                        dwarf.move_distance += dt * DWARF_MOVE_SPEED;

                        completed = dwarf.move_distance >= TILE_SIZE as f32; // TODO: for now
                    }
                    DwarfActionRequest::TakeAction(_new_action) => {
                        completed = false;
                    }
                };

                // Keep parts transform in sync with body transform
                if moved {
                    let mut body_transform: Option<Transform> = None;
                    if let Ok((_body_entity, found_body_transform)) =
                        transforms_query.get_mut(dwarf.body)
                    {
                        body_transform = Some((*found_body_transform).clone());
                    } else {
                        panic!("no dwarf body transform");
                    }
                    if let Ok((_part_entity, mut parts_transform)) =
                        transforms_query.get_mut(dwarf.parts)
                    {
                        parts_transform.translation =
                            body_transform.unwrap().translation + Vec3::new(0.0, 0.0, 1.0); // keep parts z one greater than body
                    } else {
                        panic!("no dwarf parts transform");
                    }
                }

                if completed {
                    commands.entity(e).insert(ActionCompleted);
                    // reset progress to start of actions
                    dwarf.move_distance = 0.0;
                }
            } else {
                println!("{:?} completed {:?}", first_request, dwarf);
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
