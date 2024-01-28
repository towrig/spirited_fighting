use bevy::prelude::*;

pub struct CustomCameraPlugin;

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
    //query: Query<&Name, With<Person>>
){

}