use bevy::prelude::*;

pub struct CustomCameraPlugin;
const CAMERA_SPEED: f32 = 0.1;

impl Plugin for CustomCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, add_camera)
            .add_systems(Update, camera_controller);
    }
}

fn add_camera(
    mut commands: Commands,
){
     // spawn camera
     commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-3.0, 3.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });
}

fn camera_controller(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Transform, &Camera)>
){
    for (mut camera_transform, _) in query.iter_mut() {
        if keyboard_input.pressed(KeyCode::W) {
            camera_transform.translation += Vec3::new(0f32, 1f32, 0f32) * CAMERA_SPEED;
        }
        if keyboard_input.pressed(KeyCode::S) {
            camera_transform.translation += Vec3::new(0f32, -1f32, 0f32) * CAMERA_SPEED;
        }
        if keyboard_input.pressed(KeyCode::A) {
            let direction = camera_transform.left();
            camera_transform.translation += direction * CAMERA_SPEED;
        }
        if keyboard_input.pressed(KeyCode::D) {
            let direction = camera_transform.right();
            camera_transform.translation += direction * CAMERA_SPEED;
        }
    }
}