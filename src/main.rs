use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use std::f32::consts::PI;

mod plugins;
use plugins::camera::CustomCameraPlugin;
use plugins::particle_system::AwesomeParticlesPlugin;

fn main() {
    App::new()
    .insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 1.0 / 5.0f32,
    })
    .add_plugins((DefaultPlugins, CustomCameraPlugin, AwesomeParticlesPlugin))
    .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
    .add_plugins(RapierDebugRenderPlugin::default())
    .add_systems(Startup,  setup)
    .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
        
    /* Create the ground. */
    commands
        .spawn(Collider::cuboid(50.0, 0.5, 50.0))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, -2.0, 0.0)))
        .insert(MaterialMeshBundle {
            mesh: meshes.add(Cuboid::from_size(Vec3::new(100.0, 1.0, 100.0))),
            material: materials.add(StandardMaterial {
                base_color: Color::ORANGE_RED,
                perceptual_roughness: 1.0,
                ..default()
            }),
            ..default()
        });

    /* Create the bouncing man. */
    commands
        .spawn(RigidBody::Dynamic)
        .insert(Collider::capsule_y(10.0f32, 2.0f32))
        .insert(Restitution::coefficient(0.7))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, 4.0, 0.0)))
        .insert(MaterialMeshBundle {
            mesh: asset_server.load("gltf/man_with_armature.gltf#Mesh0/Primitive0"),
            material: materials.add(StandardMaterial {
                base_color: Color::WHITE,
                perceptual_roughness: 1.0,
                ..default()
            }),
            transform: Transform::from_xyz(0.0, 4.2, 0.0).with_rotation(Quat::from_rotation_y(-PI / 2.)).with_scale(Vec3::new(0.1, 0.1, 0.1)),
            ..default()
        });

    // Ambient light
    commands.insert_resource(AmbientLight {
        color: Color::GRAY * 0.2,
        ..default()
    });

    // Sun
    commands.spawn((
        DirectionalLightBundle {
            directional_light: DirectionalLight {
                shadows_enabled: true,
                illuminance: 10_000.,
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(2.0, 2.0, 2.0),
                rotation: Quat::from_euler(EulerRot::XYZ, -PI / 4., PI / 6., 0.),
                ..default()
            },
            ..default()
        },
    ));

}