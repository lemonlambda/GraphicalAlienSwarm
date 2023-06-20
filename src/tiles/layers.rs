use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

use super::generate_layer;
use super::random_tile::LayerType;
use super::TileType;
use crate::tiles::TileId;

pub struct LayerManager {
    layers: Vec<(TileStorage, Vec<TileType>)>,
    map_size: TilemapSize,
}

impl LayerManager {
    pub fn new((x, y): (u32, u32)) -> Self {
        Self {
            layers: vec![],
            map_size: TilemapSize { x, y },
        }
    }

    /// Creates a new layer in the LayerManager
    pub fn new_layer(
        &mut self,
        layer_type: LayerType,
        commands: &mut Commands,
        asset_server: &Res<AssetServer>,
        array_texture_loader: &Res<ArrayTextureLoader>,
    ) {
        let mut layer = TileStorage::empty(self.map_size);
        let tile_types = generate_layer(
            layer_type,
            commands,
            &asset_server,
            &array_texture_loader,
            &mut layer,
            self.map_size.clone(),
        );
        self.layers.push((layer, tile_types));
    }
}
