use bevy::input::mouse::MouseWheel;
use bevy::prelude::*;

use crate::{tiles::layers::LayerManager, CameraComponent};

use partial_min_max::{max, min};

const W: ScanCode = ScanCode(17);
const S: ScanCode = ScanCode(31);
const A: ScanCode = ScanCode(30);
const D: ScanCode = ScanCode(32);
const E: ScanCode = ScanCode(18);

pub fn move_camera(
    time: Res<Time>,
    key: Res<Input<ScanCode>>,
    mut scroll: EventReader<MouseWheel>,
    mut camera_pos: Query<
        (
            &mut CameraComponent,
            &mut OrthographicProjection,
            &mut Transform,
        ),
        With<CameraComponent>,
    >,
) {
    let (mut camera_comp, mut projection, mut transform) = camera_pos.single_mut();

    if key.pressed(W) {
        transform.translation.y += camera_comp.held_down_mult * 100.0 * time.delta_seconds();
    }
    if key.pressed(S) {
        transform.translation.y -= camera_comp.held_down_mult * 100.0 * time.delta_seconds();
    }
    if key.pressed(D) {
        transform.translation.x += camera_comp.held_down_mult * 100.0 * time.delta_seconds();
    }
    if key.pressed(A) {
        transform.translation.x -= camera_comp.held_down_mult * 100.0 * time.delta_seconds();
    }

    if key.any_pressed([W, S, A, D]) {
        camera_comp.held_down_mult = max(
            1.0,
            min(
                5.0,
                camera_comp.held_down_mult + (1.0 * time.delta_seconds()),
            ),
        );
    } else {
        camera_comp.held_down_mult = 1.0;
    }

    for ev in scroll.iter() {
        projection.scale -= ev.y * time.delta_seconds() * 10.0;
    }
    projection.scale = max(0.1, min(2.0, projection.scale));
}

pub fn move_layer_pos(
    mut commands: Commands,
    key: Res<Input<ScanCode>>,
    mut layer_manager: ResMut<LayerManager>,
) {
    if key.pressed(E) {
        layer_manager.move_down(&mut commands);
    }
}
