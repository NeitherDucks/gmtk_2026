// Support configuring Bevy lints within code.
#![cfg_attr(bevy_lint, feature(register_tool), register_tool(bevy))]
// Disable console on Windows for non-dev builds.
#![cfg_attr(not(feature = "dev"), windows_subsystem = "windows")]

pub mod asset_loading;
pub mod gamemodes;
pub mod menus;

use bevy::{asset::AssetMetaCheck, prelude::*};
use bevy_aseprite_ultra::prelude::*;
use bevy_ecs_ldtk::LdtkProjectHandle;

use crate::asset_loading::AssetHandles;

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, States)]
pub enum GameState {
    #[default]
    LoadingAssets,
    MainMenu,
    Options,
    Credits,
    ChooseLevel,
    EditLevel,
    PlayLevel,
    LevelFailure,
    LevelSuccess,
    NextLevel,
    WonGame,
}

/// Level to load once we go into `GameState::EditLevel`.
/// Currently takes a handle to a ldtk level, but could be the index of a level inside the project.
#[derive(Resource)]
pub struct SelectedLevel(pub LdtkProjectHandle);

fn main() -> AppExit {
    let mut app = App::new();

    // Add Bevy plugins.
    app.add_plugins((
        DefaultPlugins
            .set(AssetPlugin {
                // Wasm builds will check for meta files (that don't exist) if this isn't set.
                // This causes errors and even panics on web build on itch.
                // See https://github.com/bevyengine/bevy_github_ci_template/issues/48.
                meta_check: AssetMetaCheck::Never,
                file_path: "assets".to_string(),
                ..default()
            })
            .set(WindowPlugin {
                primary_window: Window {
                    title: "GMTK 2026".to_string(),
                    fit_canvas_to_parent: true,
                    ..default()
                }
                .into(),
                ..default()
            })
            .set(ImagePlugin::default_nearest()),
        bevy_framepace::FramepacePlugin,
        bevy_rand::prelude::EntropyPlugin::<bevy_prng::ChaCha20Rng>::default(),
        bevy_ecs_ldtk::prelude::LdtkPlugin,
        AsepriteUltraPlugin,
    ));

    #[cfg(feature = "dev")]
    app.add_plugins((
        bevy_inspector_egui::bevy_egui::EguiPlugin::default(),
        bevy_inspector_egui::quick::WorldInspectorPlugin::default(),
    ));

    app.init_state::<GameState>()
        .add_plugins((
            asset_loading::LoadingPlugin,
            menus::MenuPlugin,
            gamemodes::GameModesPlugin,
        ))
        .add_systems(OnExit(GameState::LoadingAssets), post_load_setup);

    #[cfg(feature = "dev")]
    app.add_systems(
        Update,
        (bevy::dev_tools::states::log_transitions::<GameState>,),
    );

    app.run()
}

/// Anything that needs to be done after loading, so stuff that would rely on handles for example.
fn post_load_setup(mut commands: Commands, handles: Res<AssetHandles>) {
    // Insert the starting level.
    commands.insert_resource(SelectedLevel(handles.test_level.clone()));
}
