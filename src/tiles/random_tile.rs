use crate::tiles::TileId;
use crate::tiles::TileType;
use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use bevy_missing_texture::ReplaceIfMissing;
use rand::Rng;
use seq_macro::seq;

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
                #[allow(unused_assignments)]
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

#[derive(Component)]
pub struct TileImages {
    vec: Vec<Handle<Image>>,
}

/// Generates a layer for the game, might be expanded in the future
pub fn generate_layer(
    layer_type: LayerType,
    commands: &mut Commands,
    if_missing: &mut ReplaceIfMissing,
    asset_server: &AssetServer,
    array_texture_loader: &ArrayTextureLoader,
    tile_storage: &mut TileStorage,
    map_size: TilemapSize,
    z: f32,
) -> (Vec<TileType>, Entity) {
    // Create an empty Entity to use
    let tilemap_entity = commands.spawn_empty().id();
    let mut tile_types = vec![];

    for x in 0..map_size.x {
        for y in 0..map_size.y {
            let tile_pos = TilePos { x, y };
            // Generate a random `TileId`
            let tile_id = match layer_type {
                LayerType::Surface => surface_tile(),
            };

            let tile_entity = commands
                .spawn(TileBundle {
                    position: tile_pos,
                    texture_index: TileTextureIndex(tile_id as u32),
                    tilemap_id: TilemapId(tilemap_entity),
                    ..Default::default()
                })
                .id();
            tile_types.push(TileType {
                _tt: 0,
                // Cantor Pairing Function
                // Note: xor(brl(x), brr(y)) is faster but results in bigger numbers
                // Maybe rotate in the future?
                idx: ((x + y) * (x + y + 1) / 2 + y) as u16,
                tile_id,
            });
            tile_storage.set(&tile_pos, tile_entity);
        }
    }

    // Set up values to use in creating the tilemap
    let tile_size = TilemapTileSize { x: 32.0, y: 32.0 };
    let grid_size = tile_size.into();
    let map_type = TilemapType::default();

    // Get a list of all the game textures
    let texture_handle = seq!(N in 1..=15 {
        vec![
            "textures/tiles/Air.png",
            "textures/tiles/Dirt.png",
            #(concat!("textures/tiles/Rock", N, ".png"),)*
        ]
    })
    .into_iter()
    .map(|x| asset_server.load(x))
    .collect::<Vec<_>>();

    // Creates a tilemap
    let mut binding = commands.entity(tilemap_entity);
    let tilemap_entity = binding.insert(TilemapBundle {
        grid_size,
        map_type,
        size: map_size,
        storage: tile_storage.clone(),
        texture: TilemapTexture::Vector(texture_handle.clone()),
        tile_size,
        transform: get_tilemap_center_transform(&map_size, &grid_size, &map_type, z),
        visibility: Visibility::Visible,
        ..Default::default()
    });

    // Create a texture for the tile map
    array_texture_loader.add(TilemapArrayTexture {
        texture: TilemapTexture::Vector(texture_handle.clone()),
        tile_size,
        ..Default::default()
    });

    for handle in texture_handle.into_iter() {
        if_missing.push(handle);
    }

    return (tile_types, tilemap_entity.id());
}
