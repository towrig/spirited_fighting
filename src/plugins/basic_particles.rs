use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use std::f32::consts::PI;
use bevy::render::view::NoFrustumCulling;

use crate::plugins::instancing::{InstanceMaterialData, InstanceData, CustomMaterialPlugin};


pub struct BasicParticlesPlugin;

impl Plugin for BasicParticlesPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(CustomMaterialPlugin)
            .add_systems(Startup, spawn_particles)
            .add_systems(Update, update_particles);
    }
}

fn spawn_particles(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    /*
    commands
    .spawn(RigidBody::Dynamic)
    .insert(Collider::ball(10.0f32))
    .insert(Restitution::coefficient(0.7))
    .insert(TransformBundle::from(Transform::from_xyz(0.0, 4.0, 0.0)))
    .insert(MaterialMeshBundle {
        mesh: meshes.add(Sphere::new(1.0f32)),
        material: materials.add(StandardMaterial {
            base_color: Color::WHITE,
            perceptual_roughness: 1.0,
            ..default()
        }),
        transform: Transform::from_xyz(0.0, 4.2, 0.0).with_rotation(Quat::from_rotation_y(-PI / 2.)).with_scale(Vec3::new(0.1, 0.1, 0.1)),
        ..default()
    });
    */

    commands.spawn((
        meshes.add(Cuboid::new(0.5, 0.5, 0.5)),
        SpatialBundle::INHERITED_IDENTITY,
        InstanceMaterialData(
            (1..=10)
                .flat_map(|x| (1..=10).map(move |y| (x as f32 / 10.0, y as f32 / 10.0)))
                .map(|(x, y)| InstanceData {
                    position: Vec3::new(x * 10.0 - 5.0, y * 10.0 + 6.0, 0.0),
                    scale: 1.0,
                    color: LinearRgba::from(Color::hsla(x * 360., y, 0.5, 1.0)).to_f32_array(),
                })
                .collect(),
        ),
        NoFrustumCulling,
    ));
}

fn update_particles(
    mut commands: Commands,
) {

}

