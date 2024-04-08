use bevy::{input::mouse::MouseMotion, prelude::*};
use bevy::window::{CursorGrabMode, PrimaryWindow};

pub struct CustomCameraPlugin;
const CAMERA_SPEED: f32 = 2.0;
const CAMERA_SENSITIVITY: f32 = 0.00025;

impl Plugin for CustomCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (add_camera, cursor_grab))
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

fn cursor_grab(
    mut q_windows: Query<&mut Window, With<PrimaryWindow>>,
) {
    let mut primary_window = q_windows.single_mut();

    // if you want to use the cursor, but not let it leave the window,
    // use `Confined` mode:
    //primary_window.cursor.grab_mode = CursorGrabMode::Confined;

    // for a game that doesn't use the cursor (like a shooter):
    // use `Locked` mode to keep the cursor in one place
    primary_window.cursor.grab_mode = CursorGrabMode::Locked;

    // also hide the cursor
    primary_window.cursor.visible = false;
}

fn camera_controller(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut ev_motion: EventReader<MouseMotion>,
    mut query: Query<(&mut Transform, &Camera)>
){
    //Capture mouse movement events
    let mut rotation_move = Vec2::ZERO;
    for ev in ev_motion.read() {
        rotation_move += ev.delta;
    }

    //Do changes for camera
    for (mut camera_transform, _) in query.iter_mut() {

        //Rotation
        let delta_x =  rotation_move.x * CAMERA_SENSITIVITY * std::f32::consts::PI * 2.0;
        let delta_y = rotation_move.y * CAMERA_SENSITIVITY * std::f32::consts::PI;
        let yaw = Quat::from_rotation_y(-delta_x);
        let pitch = Quat::from_rotation_x(-delta_y);
        camera_transform.rotation = yaw * camera_transform.rotation; // rotate around global y axis
        camera_transform.rotation = camera_transform.rotation * pitch; // rotate around local x axis

        //Movement
        if keyboard_input.pressed(KeyCode::KeyW) {
            let direction = camera_transform.forward();
            camera_transform.translation += direction * CAMERA_SPEED * time.delta_seconds();
        }
        if keyboard_input.pressed(KeyCode::KeyS) {
            let direction = camera_transform.back();
            camera_transform.translation += direction * CAMERA_SPEED * time.delta_seconds();
        }
        if keyboard_input.pressed(KeyCode::KeyA) {
            let direction = camera_transform.left();
            camera_transform.translation += direction * CAMERA_SPEED * time.delta_seconds();
        }
        if keyboard_input.pressed(KeyCode::KeyD) {
            let direction = camera_transform.right();
            camera_transform.translation += direction * CAMERA_SPEED * time.delta_seconds();
        }
    }
}