use bevy::asset::HandleId;
use bevy::asset::LoadState;
use bevy::prelude::*;
use colored::Colorize;

pub struct UtilsPlugin;

impl Plugin for UtilsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, missing_texture_startup)
            .add_systems(PostUpdate, missing_texture);
    }
}

#[derive(Component)]
struct MissingTexture(Handle<Image>);

fn missing_texture_startup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_empty().insert(MissingTexture(
        asset_server.load("textures/missing_texture.png"),
    ));
}

fn missing_texture(
    asset_server: Res<AssetServer>,
    mut image_event: EventReader<AssetEvent<Image>>,
    mut images: ResMut<Assets<Image>>,
    missing_texture: Query<&MissingTexture>,
) {
    let mut stored = vec![];
    let missing_texture = missing_texture.get_single().unwrap();
    for handle in images.ids() {
        info!("{}", "------".green());
        info!("{handle:?} {:?}", missing_texture.0);
        // if handle == missing_texture.0.id {
        //     warn!("Early return");
        //     return;
        // }
        stored.push(missing_texture_internal(
            &asset_server,
            &images,
            handle.clone(),
            missing_texture,
        ));
        info!("{}", "------".red());
    }

    for x in stored {
        let Some(x) = x else { continue };
        info!("{:?}", x.0);
        images.set(x.0, x.1);
    }
}

fn missing_texture_internal(
    asset_server: &AssetServer,
    images: &Assets<Image>,
    handle: HandleId,
    missing_texture: &MissingTexture,
) -> Option<(Handle<Image>, Image)> {
    info!("Event: {handle:?}");
    let loadstate = asset_server.get_group_load_state(vec![handle.clone()]);
    match loadstate {
        LoadState::Failed | LoadState::NotLoaded => {
            info!("{}: {loadstate:?}", "Did Not Load Succesfully".red());
            let image = images.get(&missing_texture.0).unwrap();
            return Some((Handle::weak(handle), image.clone()));
        }
        _ => {
            info!("{}: {loadstate:?}", "Loaded Successfully".green());
        }
    };
    None
}
