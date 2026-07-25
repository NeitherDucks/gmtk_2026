use bevy::prelude::*;

use crate::{
    LevelState,
    gamemodes::Hand,
    widgets::{item_button, play_button},
};

#[derive(Component, Default, Clone)]
pub struct EditingLevelTag;

#[derive(Component, Default, Clone)]
pub struct HandBarTag;

pub struct EditingUIGameModePlugin;

impl Plugin for EditingUIGameModePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(LevelState::Editing), setup_ui)
            .add_systems(
                Update,
                update_hand_ui.run_if(resource_exists::<Hand>.and_then(resource_changed::<Hand>)),
            );
    }
}

pub fn setup_ui(mut commands: Commands) {
    commands.queue_spawn_scene(bsn! {
        EditingLevelTag
        Node {
            width: percent(100),
            height: percent(100),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
        }
        Children [
            HandBarTag
            Node {
                width: percent(100),
                height: percent(95),
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::End,
                justify_content: JustifyContent::Center,
            }
            Children [
                play_button(),
            ]
        ]
    });
}

/// Rebuilds the hand ui when the hand resource is changed
pub fn update_hand_ui(
    mut commands: Commands,
    hand: Res<Hand>,
    hand_bar: Single<Entity, With<HandBarTag>>,
) {
    commands.entity(*hand_bar).despawn_children();

    let mut new_entities = Vec::new();
    for (trap, amount) in &hand.0 {
        new_entities.push(
            commands
                .queue_spawn_scene(bsn! {
                    item_button(trap.clone(), *amount)
                })
                .id(),
        );
    }

    new_entities.push(
        commands
            .queue_spawn_scene(bsn! {
                play_button()
            })
            .id(),
    );

    commands.entity(*hand_bar).add_children(&new_entities);
}

// pub fn cleanup_ui(mut commands: Commands, entity: Single<Entity, With<EditingLevelTag>>) {
//     commands.entity(*entity).despawn();
// }
