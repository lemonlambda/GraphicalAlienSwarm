#![feature(exclusive_range_pattern)]
#![feature(macro_metavar_expr)]

use bevy::diagnostic::Diagnostics;
use bevy::prelude::*;
use bevy_ecs_tilemap::TilemapPlugin;
use bevy_framepace::*;
use bevy_screen_diags::{extract_fps, ScreenDiagsPlugin};

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
    commands.spawn(TextBundle {
        text: Text::from_section("0fps", style).with_alignment(TextAlignment::Left),
        ..Default::default()
    });
}

fn update_fps_counter(mut text: Query<&mut Text>, diagnostics: Res<Diagnostics>) {
    let mut text = text.single_mut();
    if let Some(fps) = extract_fps(&diagnostics) {
        text.sections[0].value = format!("{}fps", fps.round());
    }
}
