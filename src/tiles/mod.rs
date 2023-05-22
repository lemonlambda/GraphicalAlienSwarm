mod random_tile;
use random_tile::*;

use crate::tiles::LayerType;
use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use bitfield_derive::BitFields;
use seq_macro::seq;

pub struct SetupTilemapPlugin;

impl Plugin for SetupTilemapPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(startup);
    }
}

#[repr(u8)]
pub enum TileTypes {
    Air,
    Dirt,
    Rock,
}

#[derive(Default, BitFields, Component)]
struct TileType {
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
}

fn startup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    array_texture_loader: Res<ArrayTextureLoader>,
) {
    let map_size = TilemapSize { x: 128, y: 128 };
    let mut layer1 = TileStorage::empty(map_size);
    let mut layer2 = TileStorage::empty(map_size);
    generate_layer(
        LayerType::Surface,
        &mut commands,
        &asset_server,
        &array_texture_loader,
        &mut layer1,
        map_size.clone(),
    );
    generate_layer(
        LayerType::Surface,
        &mut commands,
        &asset_server,
        &array_texture_loader,
        &mut layer2,
        map_size,
    );
}
