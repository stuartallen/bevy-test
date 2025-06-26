//! 3D scene setup and management.

use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Startup, (spawn_basic_scene, spawn_camera));
}

/// Spawns a basic 3D scene with a plane and a cube.
fn spawn_basic_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Spawn a plane
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(5.0, 5.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.67, 0.84, 0.92))),
        Transform::default(),
        Visibility::default(),
    ));

    // Spawn a cube
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::default().mesh())),
        MeshMaterial3d(materials.add(Color::srgb(0.3, 0.5, 0.3))),
        Transform::from_xyz(0.0, 0.5, 0.0),
        Visibility::default(),
    ));

    // Add lighting
    commands.spawn((
        DirectionalLight {
            illuminance: 1_500.,
            ..default()
        },
        Transform::from_xyz(50.0, 50.0, 50.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

/// Spawns a 3D camera positioned to view the scene.
fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Name::new("3D Camera"),
        Camera3d::default(),
        Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        Visibility::default(),
    ));
} 