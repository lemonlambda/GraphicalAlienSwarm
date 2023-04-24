#![feature(exclusive_range_pattern)]

use bevy::prelude::*;
use rand::Rng;
use bevy_tileset::prelude::*;

mod camera;
use camera::move_camera;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(
            ImagePlugin::default_nearest()
        ))
        .add_plugin(TilesetPlugin::default())
        .add_startup_system(startup)
        .add_system(move_camera)
        .add_system(init_tiles)
        .run();
}

#[derive(Component)]
pub struct CameraComponent {
    pub held_down_mult: f32
}

fn startup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.spawn((Camera2dBundle {
        transform: Transform {
            translation: Vec3::new(0.0, 0.0, 999.0),
            ..Default::default()
        },
        ..Default::default()
    }, CameraComponent {
        held_down_mult: 1.0
    }));
}