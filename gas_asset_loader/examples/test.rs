use bevy::prelude::*;
use gas_asset_loader::GASImageLoaderPlugin;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, GASImageLoaderPlugin))
        .add_systems(Startup, |ass: Res<AssetServer>| {
            std::mem::forget(ass.load_untyped("foo.png"))
        })
        .run();
}
