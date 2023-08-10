use bevy::prelude::*;
use bevy::render::camera::Viewport;

use crate::common::AppState;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Finished), setup_camera);
    }
}

/// 3D Orthographic camera setup
fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle {
        camera: Camera {
            viewport: Some(Viewport{
                physical_position: UVec2::new(0, 0),
                physical_size: UVec2::new(1000, 1000),
                ..default()
            }),
            ..default()
        },
        ..default()
    });
}