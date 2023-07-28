mod random_tile;
use random_tile::*;

mod layers;
mod load_tiles;

use crate::tiles::layers::LayerManager;
use crate::tiles::LayerType;
use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use bitfield_derive::BitFields;
use seq_macro::seq;

pub struct SetupTilemapPlugin;

impl Plugin for SetupTilemapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, startup);
    }
}

/// TODO: Find out what the hell this is for.
#[repr(u8)]
#[derive(Clone, Copy)]
#[allow(dead_code)]
pub enum TileTypes {
    Air,
    Dirt,
    Rock,
}

/// Bit Field for Autotiling
#[derive(Default, Copy, Clone, BitFields, Component)]
#[allow(dead_code)]
pub struct TileType {
    #[bitfield(n @ "1:0" as bool "North")]
    #[bitfield(e @ "2:1" as bool "East")]
    #[bitfield(s @ "3:2" as bool "South")]
    #[bitfield(w @ "4:3" as bool "West")]
    #[bitfield(up @ "10:4" as u8 "Up")]
    #[bitfield(right @ "17:10" as u8 "Right")]
    #[bitfield(down @ "24:17" as u8 "Down")]
    #[bitfield(left @ "31:24" as u8 "Left")]
    _tt: u32,
    idx: u16,
    tile_id: TileId,
}

seq!(N in 1..=15 {
    /// Representative Struct for TileId's to make
    /// it easier to to pick a tilewithout knowing the Id before hand
    #[repr(u8)]
    #[non_exhaustive]
    #[derive(Copy, Clone)]
    #[allow(dead_code)]
    pub enum TileId {
        Air,
        Dirt,
        #(Rock~N,)*
    }
});

impl Default for TileId {
    fn default() -> Self {
        Self::Air
    }
}

fn startup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    array_texture_loader: Res<ArrayTextureLoader>,
) {
    let mut layer_manager = LayerManager::new((128, 128), &asset_server, &array_texture_loader); // Testing out 3 layers
    layer_manager
        .new_layer(LayerType::Surface, &mut commands)
        .new_layer(LayerType::Surface, &mut commands)
        .new_layer(LayerType::Surface, &mut commands);
}
