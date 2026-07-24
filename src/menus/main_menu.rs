use bevy::prelude::*;

use crate::{
    GameState,
    menus::widgets::{fancy_pane, menu_button},
};

#[derive(Component, Default, Clone)]
pub struct MainMenuTag;

pub fn setup(mut commands: Commands) {
    commands.queue_spawn_scene(bsn! {
        MainMenuTag
        Node {
            width: percent(100),
            height: percent(100),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
        }
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
                    menu_button("New Game", 300, 75)
                    on(|_: On<Pointer<Click>>, mut next_state: ResMut<NextState<GameState>>| {
                        next_state.set(GameState::EditLevel);
                    })
                ),
                (
                    menu_button("Select Level", 250, 63)
                    on(|_: On<Pointer<Click>>, mut next_state: ResMut<NextState<GameState>>| {
                        next_state.set(GameState::ChooseLevel);
                    })
                ),
                (
                    menu_button("Settings", 250, 63)
                    on(|_: On<Pointer<Click>>, mut next_state: ResMut<NextState<GameState>>| {
                        next_state.set(GameState::Options);
                    })
                ),
                (
                    menu_button("Credits", 250, 63)
                    on(|_: On<Pointer<Click>>, mut next_state: ResMut<NextState<GameState>>| {
                        next_state.set(GameState::Credits);
                    })
                ),
                (
                    menu_button("Quit", 200, 51)
                    on(|_: On<Pointer<Click>>, mut exit: MessageWriter<AppExit>| {
                        exit.write(AppExit::Success);
                    })
                ),
            ]
        ]
    });
}

pub fn cleanup(mut commands: Commands, entity: Single<Entity, With<MainMenuTag>>) {
    commands.entity(*entity).despawn();
}
