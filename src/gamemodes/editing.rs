use bevy::prelude::*;
use bevy_ecs_ldtk::{LdtkWorldBundle, LevelSelection};

use crate::SelectedLevel;

pub fn setup(mut commands: Commands, handles: Res<SelectedLevel>) {
    // We need a camera
    commands.spawn((
        Camera2d,
        Projection::Orthographic(OrthographicProjection {
            scale: 0.5,
            ..OrthographicProjection::default_2d()
        }),
        Transform::from_xyz(128.0, 128.0, 0.0),
    ));

    // We spawn our selected level
    commands.spawn(LdtkWorldBundle {
        ldtk_handle: handles.0.clone(),
        ..Default::default()
    });

    // This is what selects the level inside the ldtk file.
    commands.insert_resource(LevelSelection::index(0));
}

pub fn cleanup() {}

pub fn setup_ui() {}

pub fn cleanup_ui() {}
