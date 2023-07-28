use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

use super::generate_layer;
use super::random_tile::LayerType;
use super::TileType;


pub struct LayerManager<'a> {
    layers: Vec<(TileStorage, Vec<TileType>)>,
    layer_idx: usize,
    max_z: f32,
    map_size: TilemapSize,
    asset_server: &'a AssetServer,
    array_texture_loader: &'a ArrayTextureLoader,
}

impl<'a> LayerManager<'a> {
    pub fn new(
        (x, y): (u32, u32),
        asset_server: &'a AssetServer,
        array_texture_loader: &'a ArrayTextureLoader,
    ) -> Self {
        Self {
            layers: vec![],
            layer_idx: 0,
            max_z: 0.0,
            map_size: TilemapSize { x, y },
            asset_server,
            array_texture_loader
        }
    }

    /// Creates a new layer in the LayerManager
    pub fn new_layer(&mut self, layer_type: LayerType, commands: &mut Commands) -> &mut Self {
        let mut layer = TileStorage::empty(self.map_size);
        let tile_types = generate_layer(
            layer_type,
            commands,
            self.asset_server,
            self.array_texture_loader,
            &mut layer,
            self.map_size.clone(),
            self.max_z,
        );
        self.max_z += 1.0;
        self.layers.push((layer, tile_types));
        self
    }
}
