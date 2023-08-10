use bevy::prelude::*;
use bevy::render::camera::Viewport;

use crate::common::AppState;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Finished), setup_camera);
        app.add_systems(Update, camera_movement_system.run_if(
            in_state(AppState::Finished)
        ));
    }
}

/// 3D Orthographic camera setup
fn setup_camera(mut commands: Commands) {
    // commands.spawn(Camera2dBundle {
    //     camera: Camera {
    //         viewport: Some(Viewport{
    //             physical_position: UVec2::new(0, 0),
    //             physical_size: UVec2::new(1000, 1000),
    //             ..default()
    //         }),
    //         ..default()
    //     },
    //     ..default()
    // });
    commands.spawn(Camera2dBundle::default());
}

pub fn camera_movement_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Transform, With<Camera>>,
    time: Res<Time>,
) {
    if let Ok(mut transform) = query.get_single_mut() {
        let mut direction = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::W) {
            direction.y += 1.0;
        }

        if keyboard_input.pressed(KeyCode::S) {
            direction.y -= 1.0;
        }

        if keyboard_input.pressed(KeyCode::A) {
            direction.x -= 1.0;
        }

        if keyboard_input.pressed(KeyCode::D) {
            direction.x += 1.0;
        }

        let translation = &mut transform.translation;
        *translation += time.delta_seconds() * 500.0 * direction;
    }
}