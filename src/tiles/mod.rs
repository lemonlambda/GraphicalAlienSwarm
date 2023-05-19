use bevy::asset::FileAssetIo;
use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use bitfield_derive::BitFields;

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
    tile_type: u32,
}

fn startup(mut commands: Commands, asset_server: Res<AssetServer>) {}
