//! Anything regarding loading assets

use bevy::{asset::UntypedAssetId, prelude::*};
use bevy_aseprite_ultra::prelude::*;
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
            $(pub $name: $ty),+
        }

        impl AssetHandles {
            fn as_untyped(&self) -> Vec<UntypedAssetId> {
                vec![$((&self.$name).into()),+]
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
    ui: Handle<Image> = "ui/UI_Tileset.png",
    title: Handle<Image> = "ui/countvandwarfus.png",
    font: Handle<Font> = "ui/Scriptorium.ttf",
    icons: Handle<Image> = "ui/icons.png",
    test_level: LdtkProjectHandle = "test.ldtk",

    music: Handle<AudioSource> = "music/dwarves_music.wav",

    dwarf_body_blue_idle: Handle<Aseprite> = "dwarves/body/Blue/Idle.aseprite",
    dwarf_body_blue_moving: Handle<Aseprite> = "dwarves/body/Blue/Moving.aseprite",
    dwarf_body_blue_jump: Handle<Aseprite> = "dwarves/body/Blue/Jump.aseprite",
    dwarf_body_blue_lightlanding: Handle<Aseprite> = "dwarves/body/Blue/LightLanding.aseprite",
    dwarf_body_blue_heavylanding: Handle<Aseprite> = "dwarves/body/Blue/HeavyLanding.aseprite",
    dwarf_body_blue_standup: Handle<Aseprite> = "dwarves/body/Blue/StandUp.aseprite",
    dwarf_body_blue_shoveling: Handle<Aseprite> = "dwarves/body/Blue/Shoveling.aseprite",
    dwarf_body_blue_climbing: Handle<Aseprite> = "dwarves/body/Blue/Climbing.aseprite",
    dwarf_body_blue_light: Handle<Aseprite> = "dwarves/body/Blue/LightDynamite.aseprite",
    dwarf_body_blue_throw: Handle<Aseprite> = "dwarves/body/Blue/ThrowDynamite.aseprite",
    dwarf_body_blue_swing: Handle<Aseprite> = "dwarves/body/Blue/PickaxeSwing.aseprite",

    dwarf_body_red_idle: Handle<Aseprite> = "dwarves/body/Red/Idle.aseprite",
    dwarf_body_red_moving: Handle<Aseprite> = "dwarves/body/Red/Moving.aseprite",
    dwarf_body_red_jump: Handle<Aseprite> = "dwarves/body/Red/Jump.aseprite",
    dwarf_body_red_lightlanding: Handle<Aseprite> = "dwarves/body/Red/LightLanding.aseprite",
    dwarf_body_red_heavylanding: Handle<Aseprite> = "dwarves/body/Red/HeavyLanding.aseprite",
    dwarf_body_red_standup: Handle<Aseprite> = "dwarves/body/Red/StandUp.aseprite",
    dwarf_body_red_shoveling: Handle<Aseprite> = "dwarves/body/Red/Shoveling.aseprite",
    dwarf_body_red_climbing: Handle<Aseprite> = "dwarves/body/Red/Climbing.aseprite",
    dwarf_body_red_light: Handle<Aseprite> = "dwarves/body/Red/LightDynamite.aseprite",
    dwarf_body_red_throw: Handle<Aseprite> = "dwarves/body/Red/ThrowDynamite.aseprite",
    dwarf_body_red_swing: Handle<Aseprite> = "dwarves/body/Red/PickaxeSwing.aseprite",

    dwarf_body_yellow_idle: Handle<Aseprite> = "dwarves/body/Yellow/Idle.aseprite",
    dwarf_body_yellow_moving: Handle<Aseprite> = "dwarves/body/Yellow/Moving.aseprite",
    dwarf_body_yellow_jump: Handle<Aseprite> = "dwarves/body/Yellow/Jump.aseprite",
    dwarf_body_yellow_lightlanding: Handle<Aseprite> = "dwarves/body/Yellow/LightLanding.aseprite",
    dwarf_body_yellow_heavylanding: Handle<Aseprite> = "dwarves/body/Yellow/HeavyLanding.aseprite",
    dwarf_body_yellow_standup: Handle<Aseprite> = "dwarves/body/Yellow/StandUp.aseprite",
    dwarf_body_yellow_shoveling: Handle<Aseprite> = "dwarves/body/Yellow/Shoveling.aseprite",
    dwarf_body_yellow_climbing: Handle<Aseprite> = "dwarves/body/Yellow/Climbing.aseprite",
    dwarf_body_yellow_light: Handle<Aseprite> = "dwarves/body/Yellow/LightDynamite.aseprite",
    dwarf_body_yellow_throw: Handle<Aseprite> = "dwarves/body/Yellow/ThrowDynamite.aseprite",
    dwarf_body_yellow_swing: Handle<Aseprite> = "dwarves/body/Yellow/PickaxeSwing.aseprite",

    dwarf_body_purple_idle: Handle<Aseprite> = "dwarves/body/Purple/Idle.aseprite",
    dwarf_body_purple_moving: Handle<Aseprite> = "dwarves/body/Purple/Moving.aseprite",
    dwarf_body_purple_jump: Handle<Aseprite> = "dwarves/body/Purple/Jump.aseprite",
    dwarf_body_purple_lightlanding: Handle<Aseprite> = "dwarves/body/Purple/LightLanding.aseprite",
    dwarf_body_purple_heavylanding: Handle<Aseprite> = "dwarves/body/Purple/HeavyLanding.aseprite",
    dwarf_body_purple_standup: Handle<Aseprite> = "dwarves/body/Purple/StandUp.aseprite",
    dwarf_body_purple_shoveling: Handle<Aseprite> = "dwarves/body/Purple/Shoveling.aseprite",
    dwarf_body_purple_climbing: Handle<Aseprite> = "dwarves/body/Purple/Climbing.aseprite",
    dwarf_body_purple_light: Handle<Aseprite> = "dwarves/body/Purple/LightDynamite.aseprite",
    dwarf_body_purple_throw: Handle<Aseprite> = "dwarves/body/Purple/ThrowDynamite.aseprite",
    dwarf_body_purple_swing: Handle<Aseprite> = "dwarves/body/Purple/PickaxeSwing.aseprite",

    dwarf_parts_barehands_idle: Handle<Aseprite> = "dwarves/parts/Bare Hands/Idle.aseprite",
    dwarf_parts_barehands_moving: Handle<Aseprite> = "dwarves/parts/Bare Hands/Moving.aseprite",
    dwarf_parts_barehands_jump: Handle<Aseprite> = "dwarves/parts/Bare Hands/Jump.aseprite",
    dwarf_parts_barehands_lightlanding: Handle<Aseprite> = "dwarves/parts/Bare Hands/LightLanding.aseprite",
    dwarf_parts_barehands_heavylanding: Handle<Aseprite> = "dwarves/parts/Bare Hands/HeavyLanding.aseprite",
    dwarf_parts_barehands_standup: Handle<Aseprite> = "dwarves/parts/Bare Hands/StandUp.aseprite",
    dwarf_parts_barehands_climbing: Handle<Aseprite> = "dwarves/parts/Bare Hands/Climbing.aseprite",

    dwarf_parts_dynamite_idle: Handle<Aseprite> = "dwarves/parts/Dynamite/Idle.aseprite",
    dwarf_parts_dynamite_moving: Handle<Aseprite> = "dwarves/parts/Dynamite/Moving.aseprite",
    dwarf_parts_dynamite_jump: Handle<Aseprite> = "dwarves/parts/Dynamite/Jump.aseprite",
    dwarf_parts_dynamite_lightlanding: Handle<Aseprite> = "dwarves/parts/Dynamite/LightLanding.aseprite",
    dwarf_parts_dynamite_heavylanding: Handle<Aseprite> = "dwarves/parts/Dynamite/HeavyLanding.aseprite",
    dwarf_parts_dynamite_standup: Handle<Aseprite> = "dwarves/parts/Dynamite/StandUp.aseprite",
    dwarf_parts_dynamite_light: Handle<Aseprite> = "dwarves/parts/Dynamite/Light.aseprite",
    dwarf_parts_dynamite_throw: Handle<Aseprite> = "dwarves/parts/Dynamite/Throw.aseprite",

    dwarf_parts_shovel_gold_idle: Handle<Aseprite> = "dwarves/parts/Shovel/Gold/Idle.aseprite",
    dwarf_parts_shovel_gold_moving: Handle<Aseprite> = "dwarves/parts/Shovel/Gold/Moving.aseprite",
    dwarf_parts_shovel_gold_jump: Handle<Aseprite> = "dwarves/parts/Shovel/Gold/Jump.aseprite",
    dwarf_parts_shovel_gold_lightlanding: Handle<Aseprite> = "dwarves/parts/Shovel/Gold/LightLanding.aseprite",
    dwarf_parts_shovel_gold_heavylanding: Handle<Aseprite> = "dwarves/parts/Shovel/Gold/HeavyLanding.aseprite",
    dwarf_parts_shovel_gold_standup: Handle<Aseprite> = "dwarves/parts/Shovel/Gold/StandUp.aseprite",
    dwarf_parts_shovel_gold_shoveling: Handle<Aseprite> = "dwarves/parts/Shovel/Gold/Shoveling.aseprite",

    dwarf_parts_shovel_stone_idle: Handle<Aseprite> = "dwarves/parts/Shovel/Stone/Idle.aseprite",
    dwarf_parts_shovel_stone_moving: Handle<Aseprite> = "dwarves/parts/Shovel/Stone/Moving.aseprite",
    dwarf_parts_shovel_stone_jump: Handle<Aseprite> = "dwarves/parts/Shovel/Stone/Jump.aseprite",
    dwarf_parts_shovel_stone_lightlanding: Handle<Aseprite> = "dwarves/parts/Shovel/Stone/LightLanding.aseprite",
    dwarf_parts_shovel_stone_heavylanding: Handle<Aseprite> = "dwarves/parts/Shovel/Stone/HeavyLanding.aseprite",
    dwarf_parts_shovel_stone_standup: Handle<Aseprite> = "dwarves/parts/Shovel/Stone/StandUp.aseprite",
    dwarf_parts_shovel_stone_shoveling: Handle<Aseprite> = "dwarves/parts/Shovel/Stone/Shoveling.aseprite",

    dwarf_parts_shovel_iron_idle: Handle<Aseprite> = "dwarves/parts/Shovel/Iron/Idle.aseprite",
    dwarf_parts_shovel_iron_moving: Handle<Aseprite> = "dwarves/parts/Shovel/Iron/Moving.aseprite",
    dwarf_parts_shovel_iron_jump: Handle<Aseprite> = "dwarves/parts/Shovel/Iron/Jump.aseprite",
    dwarf_parts_shovel_iron_lightlanding: Handle<Aseprite> = "dwarves/parts/Shovel/Iron/LightLanding.aseprite",
    dwarf_parts_shovel_iron_heavylanding: Handle<Aseprite> = "dwarves/parts/Shovel/Iron/HeavyLanding.aseprite",
    dwarf_parts_shovel_iron_standup: Handle<Aseprite> = "dwarves/parts/Shovel/Iron/StandUp.aseprite",
    dwarf_parts_shovel_iron_shoveling: Handle<Aseprite> = "dwarves/parts/Shovel/Iron/Shoveling.aseprite",

    dwarf_parts_pickaxe_gold_idle: Handle<Aseprite> = "dwarves/parts/Pickaxe/Gold/Idle.aseprite",
    dwarf_parts_pickaxe_gold_moving: Handle<Aseprite> = "dwarves/parts/Pickaxe/Gold/Moving.aseprite",
    dwarf_parts_pickaxe_gold_jump: Handle<Aseprite> = "dwarves/parts/Pickaxe/Gold/Jump.aseprite",
    dwarf_parts_pickaxe_gold_lightlanding: Handle<Aseprite> = "dwarves/parts/Pickaxe/Gold/LightLanding.aseprite",
    dwarf_parts_pickaxe_gold_heavylanding: Handle<Aseprite> = "dwarves/parts/Pickaxe/Gold/HeavyLanding.aseprite",
    dwarf_parts_pickaxe_gold_standup: Handle<Aseprite> = "dwarves/parts/Pickaxe/Gold/StandUp.aseprite",
    dwarf_parts_pickaxe_gold_swing: Handle<Aseprite> = "dwarves/parts/Pickaxe/Gold/Swing.aseprite",

    dwarf_parts_pickaxe_stone_idle: Handle<Aseprite> = "dwarves/parts/Pickaxe/Stone/Idle.aseprite",
    dwarf_parts_pickaxe_stone_moving: Handle<Aseprite> = "dwarves/parts/Pickaxe/Stone/Moving.aseprite",
    dwarf_parts_pickaxe_stone_jump: Handle<Aseprite> = "dwarves/parts/Pickaxe/Stone/Jump.aseprite",
    dwarf_parts_pickaxe_stone_lightlanding: Handle<Aseprite> = "dwarves/parts/Pickaxe/Stone/LightLanding.aseprite",
    dwarf_parts_pickaxe_stone_heavylanding: Handle<Aseprite> = "dwarves/parts/Pickaxe/Stone/HeavyLanding.aseprite",
    dwarf_parts_pickaxe_stone_standup: Handle<Aseprite> = "dwarves/parts/Pickaxe/Stone/StandUp.aseprite",
    dwarf_parts_pickaxe_stone_swing: Handle<Aseprite> = "dwarves/parts/Pickaxe/Stone/Swing.aseprite",

    dwarf_parts_pickaxe_iron_idle: Handle<Aseprite> = "dwarves/parts/Pickaxe/Iron/Idle.aseprite",
    dwarf_parts_pickaxe_iron_moving: Handle<Aseprite> = "dwarves/parts/Pickaxe/Iron/Moving.aseprite",
    dwarf_parts_pickaxe_iron_jump: Handle<Aseprite> = "dwarves/parts/Pickaxe/Iron/Jump.aseprite",
    dwarf_parts_pickaxe_iron_lightlanding: Handle<Aseprite> = "dwarves/parts/Pickaxe/Iron/LightLanding.aseprite",
    dwarf_parts_pickaxe_iron_heavylanding: Handle<Aseprite> = "dwarves/parts/Pickaxe/Iron/HeavyLanding.aseprite",
    dwarf_parts_pickaxe_iron_standup: Handle<Aseprite> = "dwarves/parts/Pickaxe/Iron/StandUp.aseprite",
    dwarf_parts_pickaxe_iron_swing: Handle<Aseprite> = "dwarves/parts/Pickaxe/Iron/Swing.aseprite",
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
