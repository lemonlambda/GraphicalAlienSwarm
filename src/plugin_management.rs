use bevy::app::PluginGroupBuilder;
use bevy::ecs::schedule::ScheduleLabel;
use bevy::prelude::*;
use bevy::utils::label::DynEq;
use bevy_ecs_tilemap::TilemapPlugin;
use bevy_framepace::*;
use bevy_magic_light_2d::prelude::BevyMagicLight2DPlugin;
use bevy_missing_texture::MissingTexturePlugin;
use bevy_screen_diags::ScreenDiagsPlugin;
use std::hash::Hasher;

use crate::camera::move_camera;
use crate::tiles::SetupTilemapPlugin;
use crate::update_fps_counter;
use crate::{change_size, spawn_lights, startup_main};

pub struct GamePlugins;
impl Plugin for GamePlugins {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
            .add_plugins((ExternalPluginGroup, CorePluginGroup));
    }
}

/// Core plugins are plugins that I have made in the actual game itself
struct CorePluginGroup;
impl PluginGroup for CorePluginGroup {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(CoreSystems)
            .add(SetupTilemapPlugin)
    }
}

struct CoreSystems;
impl Plugin for CoreSystems {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, change_size)
            .add_systems(Startup, (startup_main, spawn_lights).chain())
            .add_systems(Update, (move_camera, update_fps_counter));
    }
}

/// External plugins are plugins from external libraries or bevy itself
struct ExternalPluginGroup;
impl PluginGroup for ExternalPluginGroup {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(MissingTexturePlugin::new("textures/missing_texture.png"))
            .add(TilemapPlugin)
            .add(ScreenDiagsPlugin)
            .add(FramepacePlugin)
            .add(BevyMagicLight2DPlugin)
    }
}
