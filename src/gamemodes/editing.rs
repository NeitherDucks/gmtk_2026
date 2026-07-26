use crate::{
    LevelState,
    asset_loading::AssetHandles,
    entities::Trap,
    gamemodes::{Grid, Item},
    gamemodes::hand::Hand,
    entities::TrapType,
    entities::TrapTag,
};
use bevy::{prelude::*, window::PrimaryWindow};
use bevy_ecs_ldtk::GridCoords;

const GRID_SIZE: i32 = 16;

pub struct EditingGameModePlugin;

impl Plugin for EditingGameModePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MouseGridPosition>();
        app.add_systems(OnEnter(LevelState::Editing), setup);
        app.add_systems(
            Update,
            (
                update_mouse_pos,
                update_added_trap,
                update_removed_trap,
                spawn_trap_ghost.run_if(
                    resource_exists::<PlacingTrap>.and_then(resource_changed::<PlacingTrap>),
                ),
                update_inputs,
                (update_trap_ghost).run_if(resource_exists::<PlacingTrap>),
                cleanup_trap_ghost.run_if(resource_removed::<PlacingTrap>),
            )
                .run_if(in_state(LevelState::Editing)),
        );
    }
}

#[derive(Debug, Component)]
pub struct PlacedTrap;

#[derive(Debug, Resource)]
pub struct PlacingTrap(pub TrapType);

#[derive(Debug, Component)]
pub struct TrapGhostTag;

#[derive(Debug, Resource, Default)]
pub struct MouseGridPosition(Option<GridCoords>);

pub fn setup() {}

pub fn spawn_trap_ghost(
    mut commands: Commands,
    placements: Query<Entity, With<TrapGhostTag>>,
    new_trap: If<Res<PlacingTrap>>,
    asset_handles: Res<AssetHandles>,
) {
    // Remove any previous placements
    for entity in placements {
        commands.entity(entity).despawn();
    }

    // Spawn new placement
    commands.spawn((
        Name::new("TrapGhost"),
        TrapGhostTag,
        sprite_from_trap_type(new_trap.0.0, &asset_handles),
        new_trap.0.0,
        Visibility::Hidden,
    ));
}

fn update_mouse_pos(
    mut mouse: ResMut<MouseGridPosition>,
    window: Single<&Window, With<PrimaryWindow>>,
    camera: Single<(&Camera, &GlobalTransform)>,
) {
    if let Some(cursor) = window
        .cursor_position()
        .and_then(|c| camera.0.viewport_to_world_2d(camera.1, c).ok())
    {
        let coords =
            bevy_ecs_ldtk::utils::translation_to_grid_coords(cursor, IVec2::splat(GRID_SIZE));

        mouse.0 = Some(coords);
    } else {
        mouse.0 = None;
    }
}

pub fn update_added_trap(
    mut grid: ResMut<Grid>,
    query: Query<(Entity, &GridCoords, &TrapType), Added<GridCoords>>,
) {
    for (entity, coord, trap) in &query {
        grid.items.insert(*coord, Item::Trap((entity, *trap)));
    }
}

pub fn update_removed_trap(mut grid: ResMut<Grid>, mut query: RemovedComponents<GridCoords>) {
    query.read().for_each(|entity| {
        grid.items.retain(|_, v| {
            if let Item::Trap((e, _)) = v
                && e == &entity
            {
                false
            } else {
                true
            }
        });
    });
}

pub fn update_trap_ghost(
    mut ghost: Single<(&mut Transform, &mut Visibility, &mut Sprite), With<TrapGhostTag>>,
    mouse: Res<MouseGridPosition>,
    grid: Res<Grid>,
) {
    if let Some(coords) = mouse.0 {
        if grid.is_collision(&coords) {
            ghost.2.color = Color::linear_rgb(1.0, 0.0, 0.0);
        } else {
            ghost.2.color = Color::WHITE;
        }

        let position =
            bevy_ecs_ldtk::utils::grid_coords_to_translation(coords, IVec2::splat(GRID_SIZE));

        ghost.0.translation = position.extend(15.0);
        *ghost.1 = Visibility::Inherited;
    } else {
        *ghost.1 = Visibility::Hidden;
    }
}

pub fn update_inputs(
    mut commands: Commands,
    traps: Query<Entity, With<PlacedTrap>>,
    keys: Res<ButtonInput<KeyCode>>,
    mouse: Res<ButtonInput<MouseButton>>,
    coord: Res<MouseGridPosition>,
    trap: Option<Res<PlacingTrap>>,
    grid: Res<Grid>,
    handles: Res<AssetHandles>,
    mut hand: ResMut<Hand>,
) {
    let Some(trap) = trap else {
        // If we are not placeing a trap
        // and the mouse is somewhere sensible
        // and there is something below the mouse
        // and it's a trap
        // and we placed it
        // and it fits in the hand
        // Big if
        if mouse.just_pressed(MouseButton::Right)
            && let Some(coord) = coord.0
            && let Some(item) = grid.items.get(&coord)
            && let Item::Trap((entity, trap)) = item
            && traps.contains(*entity)
            && hand.increment(trap).is_some()
        {
            // remove the trap
            commands.entity(*entity).despawn();
        };

        return;
    };

    // Escape or Right click cancel placement
    if keys.just_pressed(KeyCode::Escape) || mouse.just_pressed(MouseButton::Right) {
        commands.remove_resource::<PlacingTrap>();
    // If we left click
    } else if mouse.just_released(MouseButton::Left) {
        // Grab the coords
        let Some(coord) = coord.0 else {
            // If there is no coords (outside play area or window), we cancel placement
            commands.remove_resource::<PlacingTrap>();
            return;
        };

        // If we are outside level
        if grid.is_outside_level(&coord) {
            // cancel placement
            commands.remove_resource::<PlacingTrap>();
        // finally if we are not colliding
        } else if !grid.is_collision(&coord) {
            // Convert to world position
            let position =
                bevy_ecs_ldtk::utils::grid_coords_to_translation(coord, IVec2::splat(GRID_SIZE));

            // Remove one trap from the hand
            let Some(remainder) = hand.decrement(&trap.0) else {
                // Couldn't find the trap in hand, should never happen.
                commands.remove_resource::<PlacingTrap>();
                return;
            };

            // Spawn the trap
            commands.spawn((
                Trap {
                    sprite: sprite_from_trap_type(trap.0, &handles),
                    grid_coords: coord,
                    tag: TrapTag,
                    trap: trap.0,
                },
                PlacedTrap,
                Transform::from_translation(position.extend(4.0)),
            ));

            // If we don't have any more trap of that type, stop placing
            if remainder == 0 {
                commands.remove_resource::<PlacingTrap>();
            }
        }
    }
}

pub fn cleanup_trap_ghost(mut commands: Commands, ghosts: Query<Entity, With<TrapGhostTag>>) {
    // Remove any previous ghosts
    for entity in ghosts {
        commands.entity(entity).despawn();
    }
}

fn sprite_from_trap_type(trap: TrapType, handles: &Res<AssetHandles>) -> Sprite {
    Sprite {
        image: handles.icons.clone(),
        rect: Some(trap.get_rect()),
        ..Default::default()
    }
}
