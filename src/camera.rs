use bevy::prelude::*;
use bevy::render::primitives::Frustum;

use crate::CameraComponent;

use partial_min_max::{min, max};

pub fn move_camera(
    time: Res<Time>,
    key: Res<Input<ScanCode>>,
    mut camera_pos: Query<&mut OrthographicProjection>
) {
    let mut projection = camera_pos.single_mut();

   
    if key.pressed(ScanCode(17)) {
        projection.scale -= 1.0 * time.delta_seconds();
    }
    if key.pressed(ScanCode(31)) {
        projection.scale += 1.0 * time.delta_seconds();
    }
    projection.scale = max(0.1, min(2.0, projection.scale));
}
