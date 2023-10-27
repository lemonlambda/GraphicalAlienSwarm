use bevy::app::PluginGroupBuilder;
use bevy::prelude::*;
use bevy::time::common_conditions::*;
use bevy_ecs_tilemap::TilemapPlugin;
use bevy_framepace::*;
use bevy_missing_texture::MissingTexturePlugin;
use bevy_screen_diags::ScreenDiagsPlugin;

use crate::camera::{move_camera, move_layer_pos};
use crate::startup;
use crate::tiles::SetupTilemapPlugin;
use crate::update_fps_counter;

use std::time::Duration;

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
        app.add_systems(Startup, startup).add_systems(
            Update,
            (
                move_camera,
                update_fps_counter,
                move_layer_pos.run_if(on_timer(Duration::from_millis(500))),
            ),
        );
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
    }
}
