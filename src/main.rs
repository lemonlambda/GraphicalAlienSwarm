#![feature(exclusive_range_pattern)]

use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use rand::Rng;

mod tileset_consts;
use tileset_consts::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(TilemapPlugin)
        .add_startup_system(startup)
        .run();
}

fn startup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    array_texture_loader: Res<ArrayTextureLoader>
) {
    let mut rng = rand::thread_rng();
    commands.spawn(Camera2dBundle::default());

    let texture_handle: Handle<Image> = asset_server.load("TileSheet.png");

    let map_size = TilemapSize { x: 128, y: 128 };

    let tilemap_entity = commands.spawn_empty().id();
    let mut tile_storage = TileStorage::empty(map_size);

    for x in 0..map_size.x {
        for y in 0..map_size.y {
            let tile_pos = TilePos { x, y };
            let tile_entity = commands
                .spawn(TileBundle {
                    position: tile_pos,
                    texture_index: TileTextureIndex(match rng.gen_range(0..=50) {
                        10..=17 => SMALL_ROCKS[rng.gen_range(0..4)],
                        18..=22 => MEDIUM_ROCKS[rng.gen_range(0..4)],
                        23..=25 => LARGE_ROCKS[rng.gen_range(0..4)],
                        _ => JUST_DIRT
                    }),
                    tilemap_id: TilemapId(tilemap_entity),
                    ..Default::default()
                })
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
        texture: TilemapTexture::Single(texture_handle),
        tile_size,
        transform: get_tilemap_center_transform(&map_size, &grid_size, &map_type, 0.0),
       ..Default::default() 
    });

    array_texture_loader.add(TilemapArrayTexture {
        texture: TilemapTexture::Single(asset_server.load("tiles.png")),
        tile_size,
        ..Default::default()
    });
}