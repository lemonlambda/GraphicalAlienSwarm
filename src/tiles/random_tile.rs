use crate::tiles::TileType;
use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use rand::Rng;
use seq_macro::seq;

seq!(N in 1..=15 {
    #[repr(u8)]
    #[non_exhaustive]
    pub enum TileId {
        Air,
        Dirt,
        #(Rock~N,)*
    }
});

macro_rules! probable {
    ($($prob:literal => $expr:expr),+) => {{
        let mut rng = rand::thread_rng();
        let rand = rng.gen_range(1..=100);
        let ranges = {
            let mut temp = vec![];
            let mut count = 0;
            $(
                count += 1;
                temp.push(count..(count + $prob));
                count += $prob;
            )+
            temp
        };
        $(
            if ranges[${index()}].contains(&rand) {
                return $expr;
            }
        )+
        return TileId::Air;
    }}
}

pub fn surface_tile() -> TileId {
    probable!(
        15 => TileId::Air,
        15 => TileId::Rock1,
        70 => TileId::Dirt
    )
}

#[non_exhaustive]
pub enum LayerType {
    Surface,
}

/// Generates a layer for the game, might be expanded in the future
pub fn generate_layer(
    layer_type: LayerType,
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    array_texture_loader: &Res<ArrayTextureLoader>,
    tile_storage: &mut TileStorage,
    map_size: TilemapSize,
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

    let tilemap_entity = commands.spawn_empty().id();

    for x in 0..map_size.x {
        for y in 0..map_size.y {
            let tile_pos = TilePos { x, y };
            let tile_entity = commands
                .spawn((
                    TileBundle {
                        position: tile_pos,
                        texture_index: TileTextureIndex(match layer_type {
                            LayerType::Surface => surface_tile(),
                            _ => surface_tile(),
                        } as u32),
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
        storage: tile_storage.clone(),
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
