use bevy::prelude::*;
use bevy_toon_shader::{ToonShaderMainCamera, ToonShaderMaterial, ToonShaderPlugin, ToonShaderSun};
use std::f32::consts::PI;

fn main() {
    App::new()
    .insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 1.0 / 5.0f32,
    })
    .add_plugins(DefaultPlugins)
    .add_plugins(ToonShaderPlugin)
    .add_systems(Startup,  setup)
    .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut toon_materials: ResMut<Assets<ToonShaderMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let toon_material = toon_materials.add(ToonShaderMaterial::default());
    // plane
    commands.spawn(MaterialMeshBundle {
        mesh: meshes.add(shape::Plane::from_size(5.0).into()),
        material: toon_material.clone(),
        ..default()
    });
    // cube
    commands.spawn(MaterialMeshBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 2.0 })),
        material: toon_material.clone(),
        transform: Transform::from_xyz(3.0, 1.0, 0.0),
        ..default()
    });
    
    // man
    let colored_toon_material = toon_materials.add(ToonShaderMaterial {
        color: Color::rgb(0.991f32, 0.6353f32, 0.5647f32),
        ..default()
    });
    commands.spawn(MaterialMeshBundle {
        mesh: asset_server.load("gltf/man_with_armature.gltf#Mesh0/Primitive0"),
        material: colored_toon_material.clone(),
        transform: Transform::from_xyz(0.0, 1.2, 0.0).with_rotation(Quat::from_rotation_y(-PI / 2.)).with_scale(Vec3::new(0.1, 0.1, 0.1)),
        ..default()
    });

    
    // Ambient light
    commands.insert_resource(AmbientLight {
        color: Color::GRAY * 0.2,
        ..default()
    });
    // light, but should be DirectionalLightBundle to work with toonshader...
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
        ToonShaderSun,
    ));
    // camera
    commands.spawn((
        Camera3dBundle {
            camera: Camera {
                hdr: true,
                ..default()
            },
            transform: Transform::from_xyz(0.0, 5., 10.0)
                .looking_at(Vec3::new(0., 1., 0.), Vec3::Y),
            ..default()
        },
        ToonShaderMainCamera,
    ));
}