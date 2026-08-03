use bevy::{audio::Volume, prelude::*};

use crate::{
    GameState,
    asset_loading::AssetHandles,
    widgets::{fancy_pane, fullscreen_node, menu_button},
};

#[derive(Component, Default, Clone)]
pub struct MainMenuTag;

#[derive(Component, Default, Clone)]
pub struct MainMenuMusicTag;

pub fn setup(mut commands: Commands, handles: Res<AssetHandles>) {
    commands.queue_spawn_scene(bsn! {
        MainMenuTag
        fullscreen_node()
        Children [
            Node {
                width: px(378*2 + 160),
                height: px(740),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                row_gap: px(12),
            }
            fancy_pane()
            Children [
                (
                    Node {
                        width: px(378*2),
                        height: px(66*2),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                    }
                    ImageNode {
                        image: "ui/countvandwarfus.png",
                    }
                ),
                (
                  Node {
                      height: px(64),
                  }
                ),
                (
                    menu_button("New Game", 36.0, 300, 75)
                    on(|_: On<Pointer<Click>>, mut next_state: ResMut<NextState<GameState>>| {
                        next_state.set(GameState::PlayLevel);
                    })
                ),
                (
                    menu_button("Select Level", 24.0, 250, 63)
                    on(|_: On<Pointer<Click>>, mut next_state: ResMut<NextState<GameState>>| {
                        next_state.set(GameState::ChooseLevel);
                    })
                ),
                (
                    menu_button("Settings", 24.0, 250, 63)
                    on(|_: On<Pointer<Click>>, mut next_state: ResMut<NextState<GameState>>| {
                        next_state.set(GameState::Options);
                    })
                ),
                (
                    menu_button("Credits", 24.0, 250, 63)
                    on(|_: On<Pointer<Click>>, mut next_state: ResMut<NextState<GameState>>| {
                        next_state.set(GameState::Credits);
                    })
                ),
                (
                    menu_button("Quit", 24.0, 200, 51)
                    on(|_: On<Pointer<Click>>, mut exit: MessageWriter<AppExit>| {
                        exit.write(AppExit::Success);
                    })
                ),
            ]
        ]
    });

    commands.spawn((
        MainMenuMusicTag,
        AudioPlayer::new(handles.music.clone()),
        PlaybackSettings {
            mode: bevy::audio::PlaybackMode::Loop,
            volume: Volume::Decibels(-2.0),
            ..Default::default()
        },
    ));
}

pub fn cleanup(mut commands: Commands, entity: Single<Entity, With<MainMenuTag>>) {
    commands.entity(*entity).despawn();
}
