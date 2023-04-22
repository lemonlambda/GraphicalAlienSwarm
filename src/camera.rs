use bevy::prelude::*;
use bevy::input::mouse::MouseWheel;
use bevy::render::primitives::Frustum;

use crate::CameraComponent;

use partial_min_max::{min, max};

pub fn move_camera(
    time: Res<Time>,
    key: Res<Input<ScanCode>>,
    mut scroll: EventReader<MouseWheel>,
    mut camera_pos: Query<&mut OrthographicProjection>,
) {
    let mut projection = camera_pos.single_mut();

    for ev in scroll.iter() {
        projection.scale -= ev.y * time.delta_seconds();
}
    projection.scale = max(0.1, min(2.0, projection.scale));
}
