use bevy::prelude::*;

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

pub fn menu_button(text: &str, width: i32, height: i32) -> impl Scene {
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
            Text(text),
        ]
        on(enter)
        on(leave)
    }
}

fn enter(e: On<Pointer<Enter>>, mut query: Query<&mut ImageNode>) {
    if let Ok(mut image) = query.get_mut(e.entity) {
        image.rect = Some(Rect {
            min: Vec2 { x: 128.0, y: 104.0 },
            max: Vec2 { x: 192.0, y: 123.0 },
        });
    }
}

fn leave(e: On<Pointer<Leave>>, mut query: Query<&mut ImageNode>) {
    if let Ok(mut image) = query.get_mut(e.entity) {
        image.rect = Some(Rect {
            min: Vec2 { x: 64.0, y: 104.0 },
            max: Vec2 { x: 128.0, y: 123.0 },
        });
    }
}
