#![feature(exclusive_range_pattern)]

use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use rand::Rng;

mod camera;
use camera::move_camera;

mod tileset_consts;
use tileset_consts::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(
            ImagePlugin::default_nearest()
        ))
        .add_plugin(TilemapPlugin)
        .add_startup_system(startup)
        .add_system(move_camera)
        .run();
}

#[derive(Component)]
pub struct CameraComponent {
    pub held_down_mult: f32
}

fn startup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    array_texture_loader: Res<ArrayTextureLoader>
) {
    let mut rng = rand::thread_rng();
    commands.spawn((Camera2dBundle::default(), CameraComponent {
        held_down_mult: 1.0
    }));
}