mod random_tile;
use random_tile::*;

use bevy::asset::FileAssetIo;
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
    let texture_handle = seq!(N in 1..=15 {
        vec![
            "Air.png",
            "Dirt.png",
            #(concat!("Rock", N, ".png"),)*
        ]
    })
    .into_iter()
    .map(|x| asset_server.load(x))
    .collect::<Vec<_>>();
    let map_size = TilemapSize { x: 128, y: 128 };

    let tilemap_entity = commands.spawn_empty().id();
    let mut tile_storage = TileStorage::empty(map_size);

    for x in 0..map_size.x {
        for y in 0..map_size.y {
            let tile_pos = TilePos { x, y };
            let tile_entity = commands
                .spawn((
                    TileBundle {
                        position: tile_pos,
                        texture_index: TileTextureIndex(surface_tile() as u32),
                        tilemap_id: TilemapId(tilemap_entity),
                        ..Default::default()
                    },
                    TileType {
                        _tt: 0,
                        // Modified Cantor's Formula
                        idx: (((x + y) * (x + y + 1) + (x * y)) / 2) as u16,
                    },
                ))
                .id();
            tile_storage.set(&tile_pos, tile_entity);
        }
    }

    let tile_size = TilemapTileSize { x: 32.0, y: 32.0 };
    let grid_size = tile_size.into();
    let map_type = TilemapType::default();

    commands.entity(tilemap_entity).insert(TilemapBundle {
        grid_size,
        map_type,
        size: map_size,
        storage: tile_storage,
        texture: TilemapTexture::Vector(texture_handle.clone()),
        tile_size,
        transform: get_tilemap_center_transform(&map_size, &grid_size, &map_type, 0.0),
        ..Default::default()
    });

    array_texture_loader.add(TilemapArrayTexture {
        texture: TilemapTexture::Vector(texture_handle),
        tile_size,
        ..Default::default()
    });
}
