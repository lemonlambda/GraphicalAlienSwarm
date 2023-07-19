#![feature(macro_metavar_expr)]
#![deny(missing_docs)]

use bevy::diagnostic::Diagnostics;
use bevy::prelude::*;
use bevy_ecs_tilemap::TilemapPlugin;
use bevy_framepace::*;
use bevy_screen_diags::{FrameCounter, ScreenDiagsPlugin};
use std::fmt::Write;

mod clamped;

mod camera;
use camera::move_camera;

mod tiles;
use tiles::SetupTilemapPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugin(TilemapPlugin)
        .add_plugin(SetupTilemapPlugin)
        .add_plugin(FramepacePlugin)
        .add_plugin(ScreenDiagsPlugin)
        .add_startup_system(startup)
        .add_system(move_camera)
        .add_system(update_fps_counter)
        .run();
}

#[derive(Component)]
pub struct CameraComponent {
    pub held_down_mult: f32,
}

fn startup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut frame_settings: ResMut<FramepaceSettings>,
) {
    frame_settings.limiter = Limiter::Auto;

    commands.spawn((
        Camera2dBundle::default(),
        CameraComponent {
            held_down_mult: 1.0,
        },
    ));
    let font = asset_server.load("fonts/FiraCode-Regular.ttf");
    let style = TextStyle {
        font: font.clone(),
        font_size: 30.0,
        color: Color::WHITE,
    };
    commands.spawn(TextBundle::from_sections([TextSection::from_style(
        TextStyle {
            font: asset_server.load("fonts/FiraCode-Regular.ttf"),
            font_size: 20.0,
            color: Color::WHITE,
        },
    )]));
}

fn update_fps_counter(
    mut text: Query<&mut Text>,
    diagnostics: Res<Diagnostics>,
    frame_counter: Res<FrameCounter>,
) {
    let mut text = text.single_mut();
    let value = &mut text.sections[0].value;
    value.clear();
    write!(value, "{:.0}fps", frame_counter.0).unwrap();
}
