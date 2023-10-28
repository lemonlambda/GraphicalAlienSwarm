use bevy::ecs::query::WorldQuery;
use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use bevy_missing_texture::ReplaceIfMissing;

use super::generate_layer;
use super::random_tile::LayerType;
use super::TileType;

#[derive(Clone, Resource)]
pub struct LayerManager {
    layers: Vec<(TileStorage, Vec<TileType>, Entity, f32)>,
    layer_idx: usize,
    max_z: f32,
    current_z: f32,
    map_size: TilemapSize,
}

impl LayerManager {
    pub fn new((x, y): (u32, u32)) -> Self {
        Self {
            layers: vec![],
            layer_idx: 0,
            max_z: 0.0,
            current_z: 0.0,
            map_size: TilemapSize { x, y },
        }
    }

    /// Creates a new layer in the LayerManager
    pub fn new_layer(
        &mut self,
        layer_type: LayerType,
        commands: &mut Commands,
        if_missing: &mut ReplaceIfMissing,
        asset_server: &AssetServer,
        array_texture_loader: &ArrayTextureLoader,
    ) -> &mut Self {
        let mut layer = TileStorage::empty(self.map_size);
        let (tile_types, tile_entity) = generate_layer(
            layer_type,
            commands,
            if_missing,
            asset_server,
            array_texture_loader,
            &mut layer,
            self.map_size.clone(),
            self.max_z,
        );
        self.layers
            .push((layer, tile_types, tile_entity, self.max_z));
        self.max_z += 1.0;
        self.current_z += 1.0;
        self
    }

    /// Move down a layer
    pub fn move_down(&mut self, commands: &mut Commands) -> &mut Self {
        info!("{}", self.layers.len());
        // Filter out the layer to then make it go away
        let layer = self
            .layers
            .iter()
            .filter(|(_, _, _, z)| {
                info!("{z}, {}", self.current_z - 1.0);
                *z == self.current_z
            })
            .collect::<Vec<_>>();

        if layer.len() == 0 {
            return self;
        }
        self.current_z -= 1.0;
        let layer = layer[0];

        let mut tile_entity_commands = commands.get_entity(layer.2).unwrap();
        tile_entity_commands.add(|entity, world: &mut World| {
            let mut real_entity = world.get_entity_mut(entity).unwrap();
            let mut visibilty = real_entity.get_mut::<Visibility>().unwrap();
            *visibilty = Visibility::Hidden;
        });

        self
    }
}
