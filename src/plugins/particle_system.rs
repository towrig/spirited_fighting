use bevy::{
    prelude::*,
    reflect::TypePath,
    render::render_resource::{AsBindGroup, ShaderRef},
};

pub struct AwesomeParticlesPlugin;

impl Plugin for AwesomeParticlesPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_plugins(MaterialPlugin::<CustomMaterial>::default())
        .add_systems(Startup, spawn);
    }
}

fn spawn(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<CustomMaterial>>,
){
    // spawn Animated
    commands.spawn(MaterialMeshBundle {
        mesh: meshes.add(Cuboid::default()),
        transform: Transform::from_xyz(10.0, 10.0, 0.0),
        material: materials.add(CustomMaterial {}),
        ..default()
    });

}


#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct CustomMaterial {}

impl Material for CustomMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/animate_shader.wgsl".into()
    }
}