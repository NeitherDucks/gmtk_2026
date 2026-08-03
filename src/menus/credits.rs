use bevy::{prelude::*, text::FontSourceTemplate};

use crate::{
    asset_loading::{AssetHandles, CreditsAsset, CreditsAssetItem, CreditsAssetSection},
    widgets::{fancy_pane, fullscreen_node},
};

// TODO: Open urls on click
// TODO: Hide text clipping outside of pane better
// TODO: Add return button

#[derive(Component, Debug, Default, Clone)]
pub struct CreditsMenuTag;

#[derive(Component, Debug, Default, Clone)]
pub struct CreditsScollTag;

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
                width: px(916),
                height: px(740),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                row_gap: px(24),
            }
            fancy_pane()
            Children [
                CreditsScollTag
                Node {
                    width: px(916),
                    height: px(660),
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    row_gap: px(24),
                    align_self: AlignSelf::Stretch,
                    overflow: Overflow::scroll_y(),
                    scrollbar_width: 0.0,
                }
                // Initial position of the scroll, "Credits" starts half way up.
                ScrollPosition( Vec2 { y: 500.0 } )
                Children [
                    (
                        Node {
                            // Put heavy margin to have blank space and hide the reset of the scroll
                            margin: UiRect {
                                top: px(660),
                            }
                        }
                        Text("Credits")
                        TextFont {
                            font: FontSourceTemplate::Family("Scriptorium"),
                            font_size: FontSize::Px(72.0),
                        }
                    ),
                    { credits_ui_elements }
                    // Put something invisible at the end to force the screen to clear before reset
                    (
                        Node {
                            margin: UiRect {
                                top: px(660),
                            }
                        }
                        Visibility::Hidden
                        Text("Kisses from around the world.")
                    ),
                ]
            ]
        ]
    });
}

pub fn update(
    query: Single<(&mut ScrollPosition, &Node, &ComputedNode), With<CreditsScollTag>>,
    time: Res<Time>,
) {
    const SCROLL_SPEED: f32 = 70.0;

    let (mut position, _node, computed) = query.into_inner();

    position.y += time.delta_secs() * SCROLL_SPEED;

    let max_offset = (computed.content_size() - computed.size()) * computed.inverse_scale_factor();
    if position.y >= max_offset.y {
        position.y = 0.0;
    }
}

pub fn cleanup(mut commands: Commands, query: Single<Entity, With<CreditsMenuTag>>) {
    commands.entity(*query).despawn();
}
