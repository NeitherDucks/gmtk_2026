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

pub fn setup_ui(mut commands: Commands, hand: Res<Hand>) {
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
                { ui_from_hand(&hand) },
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

    commands
        .entity(*hand_bar)
        .queue_spawn_related_scenes::<Children>(
            bsn_list! { { ui_from_hand(&hand) }, play_button() },
        );
}

fn ui_from_hand(hand: &Res<Hand>) -> impl SceneList {
    hand.0
        .iter()
        .map(|(trap, amount)| {
            bsn! {
                item_button(trap.clone(), *amount)
            }
        })
        .collect::<Vec<_>>()
}

// pub fn cleanup_ui(mut commands: Commands, entity: Single<Entity, With<EditingLevelTag>>) {
//     commands.entity(*entity).despawn();
// }
