use bevy::{prelude::*, text::FontSourceTemplate};

use crate::{
    asset_loading::{AssetHandles, CreditsAsset, CreditsAssetItem, CreditsAssetSection},
    widgets::{fancy_pane, fullscreen_node},
};

#[derive(Component, Debug, Default, Clone)]
pub struct CreditsMenuTag;

pub fn setup(
    mut commands: Commands,
    credits: Res<Assets<CreditsAsset>>,
    handles: Res<AssetHandles>,
) {
    let credits = credits
        .get(handles.credits.id())
        .expect("Credits should be loaded.");

    let credits_ui_elements = credits
        .0
        .iter()
        .map(|CreditsAssetSection { name, items }| {
            let items = items
                .iter()
                .map(|CreditsAssetItem { item, url }| {
                    let url = url.clone();

                    bsn! {
                        Node {}
                        Text(item)
                        TextFont {
                            font: FontSourceTemplate::Family("Scriptorium"),
                            font_size: FontSize::Px(36.0),
                        }
                        on(move |_: On<Pointer<Click>>| {
                            let url = url.clone();
                            // TODO: Open the url on click
                            dbg!(url);
                        })
                    }
                })
                .collect::<Vec<_>>();

            bsn! {
                Node {
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    // row_gap: px(6),
                }
                Children [
                    (
                        Node {}
                        Text(name)
                        TextFont {
                            font: FontSourceTemplate::Family("Scriptorium"),
                            font_size: FontSize::Px(48.0),
                        }
                    ),
                    { items }
                ]
            }
        })
        .collect::<Vec<_>>();

    commands.queue_spawn_scene(bsn! {
        #OptionMenu
        CreditsMenuTag
        fullscreen_node()
        Children [
            Node {
                width: px(378*2 + 160),
                height: px(740),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                row_gap: px(24),
            }
            fancy_pane()
            Children [
                (
                    Node { }
                    Text("Credits")
                    TextFont {
                        font: FontSourceTemplate::Family("Scriptorium"),
                        font_size: FontSize::Px(72.0),
                    }
                ),
                { credits_ui_elements }
            ]
        ]
    });
}

pub fn cleanup(mut commands: Commands, query: Single<Entity, With<CreditsMenuTag>>) {
    commands.entity(*query).despawn();
}
