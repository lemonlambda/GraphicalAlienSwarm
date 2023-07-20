//! Ripped straight from https://github.com/bevyengine/bevy/blob/main/crates/bevy_render/src/texture/image_texture_loader.rs#L15C1-L78C2
//! Modified slightly though

use anyhow::Result;
use bevy::prelude::AddAsset;
use bevy::{
    asset::{AssetLoader, LoadContext, LoadedAsset},
    prelude::{App, FromWorld, Image, Plugin, World},
    render::{
        renderer::RenderDevice,
        texture::{CompressedImageFormats, FileTextureError, ImageType, TextureError},
    },
    utils::BoxedFuture,
};
use std::path::Path;
use thiserror::Error;

pub struct CustomImageLoaderPlugin;

impl Plugin for CustomImageLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset_loader::<GASImageTextureLoader>();
    }
}

#[derive(Clone)]
pub struct GASImageTextureLoader {
    supported_compressed_formats: CompressedImageFormats,
}

const FILE_EXTENSIONS: &[&str] = &[
    "basis", "bmp", "png", "dds", "tga", "jpg", "jpeg", "ktx2", "webp", "pam", "pbm", "pgm", "ppm",
];

impl AssetLoader for GASImageTextureLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<()>> {
        Box::pin(async move {
            // use the file extension for the image type
            let ext = load_context.path().extension().unwrap().to_str().unwrap();

            let dyn_img = Image::from_buffer(
                bytes,
                ImageType::Extension(ext),
                self.supported_compressed_formats,
                true,
            );

            // Return missing texture instead
            if dyn_img.is_err() {
                let ext = Path::new("./assets/textures/missing_texture.png")
                    .extension()
                    .unwrap()
                    .to_str()
                    .unwrap();

                let dyn_img = Image::from_buffer(
                    bytes,
                    ImageType::Extension(ext),
                    self.supported_compressed_formats,
                    true,
                )
                .map_err(|err| GASFileTextureError {
                    error: err,
                    path: format!(
                        "{}",
                        Path::new("./assets/textures/missing_texture.png").display()
                    ),
                })?;

                load_context.set_default_asset(LoadedAsset::new(dyn_img));
                return Ok(());
            }

            load_context.set_default_asset(LoadedAsset::new(dyn_img.unwrap()));
            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        FILE_EXTENSIONS
    }
}

impl FromWorld for GASImageTextureLoader {
    fn from_world(world: &mut World) -> Self {
        let supported_compressed_formats = match world.get_resource::<RenderDevice>() {
            Some(render_device) => CompressedImageFormats::from_features(render_device.features()),

            None => CompressedImageFormats::all(),
        };
        Self {
            supported_compressed_formats,
        }
    }
}

/// An error that occurs when loading a texture from a file.
#[derive(Error, Debug)]
pub struct GASFileTextureError {
    error: TextureError,
    path: String,
}
impl std::fmt::Display for GASFileTextureError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(
            f,
            "Error reading image file {}: {}, this is an error in `bevy_render`.",
            self.path, self.error
        )
    }
}
