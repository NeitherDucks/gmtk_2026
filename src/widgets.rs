use bevy::{prelude::*, text::FontSourceTemplate};

use crate::{LevelState, entities::traptype::TrapType};

const ITEM_BUTTON_DISABLED: Rect = Rect {
    min: Vec2 { x: 192.0, y: 96.0 },
    max: Vec2 { x: 224.0, y: 128.0 },
};
const ITEM_BUTTON_NORMAL: Rect = Rect {
    min: Vec2 { x: 224.0, y: 96.0 },
    max: Vec2 { x: 256.0, y: 128.0 },
};
const ITEM_BUTTON_SELECTED: Rect = Rect {
    min: Vec2 { x: 256.0, y: 96.0 },
    max: Vec2 { x: 288.0, y: 128.0 },
};

#[derive(Debug, Component, Clone, Default)]
pub struct TrapAmount(u32);

pub fn fancy_pane() -> impl Scene {
    let slicer = TextureSlicer {
        border: BorderRect::all(32.0),
        center_scale_mode: SliceScaleMode::Stretch,
        sides_scale_mode: SliceScaleMode::Tile { stretch_value: 4.0 },
        max_corner_scale: 4.0,
    };

    let rect = Some(Rect {
        min: Vec2::ZERO,
        max: Vec2::splat(96.0),
    });

    bsn! {
        ImageNode {
            image: "ui/UI_Tileset.png",
            image_mode: NodeImageMode::Sliced(slicer),
            rect: rect,
        }
    }
}

pub fn menu_button(text: &str, font_size: f32, width: i32, height: i32) -> impl Scene {
    let slicer = TextureSlicer {
        border: BorderRect::all(11.0),
        center_scale_mode: SliceScaleMode::Stretch,
        sides_scale_mode: SliceScaleMode::Stretch,
        max_corner_scale: 4.0,
    };

    let button_rect = Some(Rect {
        min: Vec2 { x: 64.0, y: 104.0 },
        max: Vec2 { x: 128.0, y: 123.0 },
    });

    bsn! {
        #Button
        Node {
            width: px(width),
            height: px(height),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center
        }
        ImageNode {
            image: "ui/UI_Tileset.png",
            image_mode: NodeImageMode::Sliced(slicer),
            rect: button_rect,
        }
        Children [
            Text(text)
            TextFont {
                font: FontSourceTemplate::Family("Scriptorium"),
                font_size: FontSize::Px(font_size),
            }
        ]
        on(menu_enter)
        on(menu_leave)
    }
}

pub fn item_button(item: TrapType, amount: u32) -> impl Scene {
    let button_rect = Some(if amount > 0 {
        ITEM_BUTTON_NORMAL
    } else {
        ITEM_BUTTON_DISABLED
    });

    let icon_rect = Some(item.get_rect());

    let color = if amount > 0 {
        Color::WHITE
    } else {
        Color::linear_rgb(0.2, 0.2, 0.2)
    };

    let text = format!("{}", amount);

    bsn! {
        #Button
        TrapAmount(amount)
        Node {
            width: px(96),
            height: px(96),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center
        }
        ImageNode {
            image: "ui/UI_Tileset.png",
            rect: button_rect,
        }
        Children [
            Node {
                width: percent(100.0),
                height: percent(100.0),
                display: Display::Block,
            }
            Children [
                (
                    Node {
                        width: percent(100.0),
                        height: px(100.0),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                    }
                    ImageNode {
                        image: "ui/icons.png",
                        rect: icon_rect,
                        color: color,
                    }
                    UiTransform { scale: Vec2::splat(0.6) }
                    Children [
                        Node {
                            width: percent(100.0),
                            height: percent(100.0),
                            align_items: AlignItems::End,
                            justify_content: JustifyContent::End,
                        }
                        Children [
                            Text(text)
                            TextFont {
                                font: FontSourceTemplate::Family("Scriptorium"),
                                font_size: FontSize::Px(48.0),
                            }
                            UiTransform { translation: Vec2 {x: px(-8), y: px(8) } }
                        ]
                    ]
                ),
            ]
        ]
        on(item_enter)
        on(item_leave)
    }
}

pub fn play_button() -> impl Scene {
    let rect = Rect {
        min: Vec2 {
            x: 12.0 * 32.0,
            y: 0.0 * 32.0,
        },
        max: Vec2 {
            x: 13.0 * 32.0,
            y: 1.0 * 32.0,
        },
    };

    bsn! {
        #Button
        Node {
            width: px(96),
            height: px(96),
        }
        ImageNode {
            image: "ui/icons.png",
            rect: rect,
        }
        on(|_: On<Pointer<Click>>, mut next_state: ResMut<NextState<LevelState>>| {
            next_state.set(LevelState::Playing);
        })
    }
}

fn menu_enter(e: On<Pointer<Enter>>, mut query: Query<&mut ImageNode>) {
    if let Ok(mut image) = query.get_mut(e.entity) {
        image.rect = Some(Rect {
            min: Vec2 { x: 128.0, y: 104.0 },
            max: Vec2 { x: 192.0, y: 123.0 },
        });
    }
}

fn menu_leave(e: On<Pointer<Leave>>, mut query: Query<&mut ImageNode>) {
    if let Ok(mut image) = query.get_mut(e.entity) {
        image.rect = Some(Rect {
            min: Vec2 { x: 64.0, y: 104.0 },
            max: Vec2 { x: 128.0, y: 123.0 },
        });
    }
}

fn item_enter(e: On<Pointer<Enter>>, mut query: Query<(&mut ImageNode, &TrapAmount)>) {
    if let Ok((mut image, amount)) = query.get_mut(e.entity)
        && amount.0 > 0
    {
        image.rect = Some(ITEM_BUTTON_SELECTED);
    }
}

fn item_leave(e: On<Pointer<Leave>>, mut query: Query<(&mut ImageNode, &TrapAmount)>) {
    if let Ok((mut image, amount)) = query.get_mut(e.entity) {
        if amount.0 > 0 {
            image.rect = Some(ITEM_BUTTON_NORMAL);
        } else {
            image.rect = Some(ITEM_BUTTON_DISABLED);
        }
    }
}
