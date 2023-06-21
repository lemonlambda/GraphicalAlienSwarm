#![feature(exclusive_range_pattern)]
#![feature(macro_metavar_expr)]

use bevy::prelude::*;
use bevy_ecs_tilemap::TilemapPlugin;

mod math_helpers;

mod camera;
use camera::move_camera;

mod tiles;
use tiles::SetupTilemapPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugin(TilemapPlugin)
        .add_plugin(SetupTilemapPlugin)
        .add_startup_system(startup)
        .add_system(move_camera)
        .run();
}

#[derive(Component)]
pub struct CameraComponent {
    pub held_down_mult: f32,
}

fn startup(mut commands: Commands, _asset_server: Res<AssetServer>) {
    commands.spawn((
        Camera2dBundle::default(),
        CameraComponent {
            held_down_mult: 1.0,
        },
    ));
}
