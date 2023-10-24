//! Graphical Alien Swarm a game about being in space
//! and living and doing stuff, not sure yet

#![feature(macro_metavar_expr, stmt_expr_attributes, specialization)]
#![warn(missing_docs)]

// Use a custom Tiny Allocator
use tcmalloc::TCMalloc;

#[global_allocator]
static GLOBAL: TCMalloc = TCMalloc;

use bevy::diagnostic::DiagnosticsStore;
use bevy::prelude::*;

use bevy::render::camera::RenderTarget;
use bevy_ecs_tilemap::TilemapPlugin;
use bevy_framepace::*;
use bevy_magic_light_2d::gi::resource::ComputedTargetSizes;
use bevy_magic_light_2d::prelude::*;
use bevy_missing_texture::MissingTexturePlugin;
use bevy_screen_diags::FrameRate;
use bevy_screen_diags::ScreenDiagsPlugin;
use bytesize::ByteSize;
use lazy_static::lazy_static;
use std::fmt::Write;
use sysinfo::SystemExt;

mod camera;
mod clamped;
mod plugin_management;
mod tiles;
// mod tiles_v2;
use plugin_management::GamePlugins;

fn main() {
    App::new().add_plugins(GamePlugins).run();
}

/// A struct to get track the camera's "velocity"
#[derive(Component)]
pub struct CameraComponent {
    /// Multiplier to the speed that the camera is moving at
    pub held_down_mult: f32,
}

fn change_size(mut targets_sizes: ResMut<ComputedTargetSizes>) {
    targets_sizes.primary_target_usize = (1920, 1080).into();
}

fn startup_main(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut frame_settings: ResMut<FramepaceSettings>,
    post_processing_target: Res<PostProcessingTarget>,
) {
    frame_settings.limiter = Limiter::Auto;

    let render_target = post_processing_target
        .handles
        .as_ref()
        .expect("No post processing target")
        .1
        .clone();

    commands
        .spawn((
            Camera2dBundle {
                camera: Camera {
                    hdr: true,
                    target: RenderTarget::Image(render_target),
                    ..Default::default()
                },
                ..Default::default()
            },
            CameraComponent {
                held_down_mult: 1.0,
            },
            FloorCamera,
        ))
        .insert(SpriteCamera);
    let font = asset_server.load("fonts/FiraCode-Regular.ttf");
    let _style = TextStyle {
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

lazy_static! {
    static ref SYSTEM: sysinfo::System = sysinfo::System::new_all();
}

fn update_fps_counter(
    mut text: Query<&mut Text>,
    _diagnostics: Res<DiagnosticsStore>,
    frame_counter: Res<FrameRate>, // WARN: This is broken until https://github.com/jomala/bevy_screen_diags/pull/19 gets merged
) {
    let mut text = text.single_mut();
    let value = &mut text.sections[0].value;
    value.clear();
    // Get the memory usage of the program
    let (physical, r#virtual_old) = if let Some(usage) = memory_stats::memory_stats() {
        (
            usage.physical_mem as u64 >> 10,
            usage.virtual_mem as u64 >> 10,
        )
    } else {
        (0, 0)
    };

    let ram_total = SYSTEM.total_memory();
    let swap_total = SYSTEM.total_swap();
    let physical = ByteSize::kb(physical);
    let r#virtual = ByteSize::kb(r#virtual_old);

    write!(
        value,
        "{:.0}fps\nMem: {} / {} {:.2}%\nSwap: {} / {} | {:.2}%",
        frame_counter.0,
        physical.to_string_as(true),
        ByteSize(ram_total).to_string_as(true),
        (physical.as_u64() as f32 / ram_total as f32) * 100.0,
        r#virtual.to_string_as(true),
        ByteSize(swap_total).to_string_as(true),
        (r#virtual.as_u64() as f32 / swap_total as f32) * 100.0
    )
    .unwrap();
}

fn spawn_lights(mut commands: Commands) {
    let occluder_entity = commands
        .spawn((
            Transform::from_translation(Vec3::new(0., 0., 0.)),
            GlobalTransform::default(),
            Visibility::Visible,
            ComputedVisibility::default(),
            LightOccluder2D {
                h_size: Vec2::new(40.0, 20.0),
            },
        ))
        .id();

    let spawn_light = |cmd: &mut Commands,
                       x: f32,
                       y: f32,
                       name: &'static str,
                       light_source: OmniLightSource2D| {
        return cmd
            .spawn(Name::new(name))
            .insert(light_source)
            .insert(SpatialBundle {
                transform: Transform {
                    translation: Vec3::new(x, y, 0.0),
                    ..default()
                },
                ..default()
            })
            .id();
    };

    let mut lights = vec![];

    lights.push(spawn_light(
        &mut commands,
        -128.,
        -128.,
        "left",
        OmniLightSource2D {
            intensity: 1.0,
            color: Color::rgb_u8(255, 0, 0),
            falloff: Vec3::new(1.5, 10.0, 0.005),
            ..default()
        },
    ));

    commands
        .spawn(SpatialBundle::default())
        .insert(Name::new("lights"))
        .push_children(&lights);
}
