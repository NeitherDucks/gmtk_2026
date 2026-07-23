//! Anything regarding loading assets

use bevy::{asset::UntypedAssetId, prelude::*};
use bevy_ecs_ldtk::LdtkProjectHandle;

use crate::GameState;

pub struct LoadingPlugin;

impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::LoadingAssets),
            (setup_loading_screen, load_assets).chain(),
        )
        .add_systems(
            OnExit(GameState::LoadingAssets),
            (cleanup_loading_screen, cleaup_loading_resource),
        )
        .add_systems(
            Update,
            (check_assets, update_loading_screen)
                .chain()
                .run_if(in_state(GameState::LoadingAssets)),
        );
    }
}

/// Macro to enforce `as_untyped` and `FromWorld` correctness
macro_rules! asset_handles {
    ($($name:ident: $ty:ty = $path:literal),+$(,)*) => {
        #[derive(Resource)]
        pub struct AssetHandles {
            pub $($name: $ty),+
        }

        impl AssetHandles {
            fn as_untyped(&self) -> Vec<UntypedAssetId> {
                vec![$(self.$name.clone().into()),+]
            }
        }

        impl FromWorld for AssetHandles {
            fn from_world(world: &mut World) -> Self {
                let assets = world.resource::<AssetServer>();
                Self {
                    $($name: assets.load($path).into()),*
                }
            }
        }
    };
}

asset_handles! {
    test_level: LdtkProjectHandle = "test.ldtk",
}

#[derive(Resource)]
struct LoadingState(f32);

/// Setup the loading screen
fn setup_loading_screen() {
    // TODO
}

/// Update the loading screen value
fn update_loading_screen() {
    // TODO
}

/// Remove the loading screen
fn cleanup_loading_screen() {
    // TODO
}

/// Queue our assets to load when the game starts
fn load_assets(mut commands: Commands) {
    commands.insert_resource(LoadingState(0.0));
    commands.init_resource::<AssetHandles>();
}

/// Check if our assets are fully loaded before continuing
fn check_assets(
    handles: Res<AssetHandles>,
    asset_server: Res<AssetServer>,
    mut loading_state: ResMut<LoadingState>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    let untyped = handles.as_untyped();
    let count = untyped.len();

    let mut loaded_handles = 0;
    for handle in untyped {
        loaded_handles += asset_server.is_loaded_with_dependencies(handle) as usize;
    }

    if loaded_handles == count {
        next_state.set(GameState::MainMenu);
    } else {
        let percent = loaded_handles as f32 / count as f32;
        loading_state.0 = percent;

        debug!("Loading assets: {:.0}%", percent * 100.0);
    }
}

/// Remove the `LoadingState` resource once we don't need it anymore
fn cleaup_loading_resource(mut commands: Commands) {
    commands.remove_resource::<LoadingState>();
}
