//! 3D scene setup and management.

use bevy::prelude::*;

// ============================================================================
// APP/PLUGIN SETUP
// ============================================================================

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Startup, (spawn_basic_scene, spawn_player, spawn_camera))
        .add_systems(Update, (player_movement, camera_follow).chain());
}

// ============================================================================
// CONSTANTS
// ============================================================================

const PLAYER_SPEED: f32 = 5.0;
const CAMERA_OFFSET: Vec3 = Vec3::new(-2.0, 2.5, 5.0);

// ============================================================================
// RESOURCES
// ============================================================================

/// Player resource to track the player entity
#[derive(Resource)]
struct Player(Entity);

// ============================================================================
// STARTUP SYSTEMS
// ============================================================================

/// Spawns a basic 3D scene with a plane and lighting.
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

    // Add lighting
    commands.spawn((
        DirectionalLight {
            illuminance: 1_500.,
            ..default()
        },
        Transform::from_xyz(50.0, 50.0, 50.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

/// Spawns the player cube.
fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let player_entity = commands.spawn((
        Name::new("Player"),
        Mesh3d(meshes.add(Cuboid::default().mesh())),
        MeshMaterial3d(materials.add(Color::srgb(0.3, 0.5, 0.3))),
        Transform::from_xyz(0.0, 0.5, 0.0),
        Visibility::default(),
    )).id();

    commands.insert_resource(Player(player_entity));
}

/// Spawns a 3D camera positioned to view the scene.
fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Name::new("3D Camera"),
        Camera3d::default(),
        Transform::from_xyz(CAMERA_OFFSET.x, CAMERA_OFFSET.y, CAMERA_OFFSET.z).looking_at(Vec3::ZERO, Vec3::Y),
        Visibility::default(),
    ));
}

// ============================================================================
// UPDATE SYSTEMS
// ============================================================================

/// Handles player movement with WASD keys.
fn player_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut player_query: Query<&mut Transform, With<Name>>,
    player: Res<Player>,
) {
    if let Ok(mut transform) = player_query.get_mut(player.0) {
        let mut direction = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::KeyW) {
            direction.z -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyS) {
            direction.z += 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyA) {
            direction.x -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyD) {
            direction.x += 1.0;
        }

        if direction != Vec3::ZERO {
            direction = direction.normalize();
            transform.translation += direction * PLAYER_SPEED * time.delta_secs();
        }
    }
}

/// Makes the camera follow the player.
fn camera_follow(
    mut player_query: Query<(&Transform, &Name), Without<Camera3d>>,
    mut camera_query: Query<(&mut Transform, &Name), With<Camera3d>>,
) {
    // Find player by name
    let player_transform = player_query.iter_mut().find(|(_, name)| {
        name.as_str() == "Player"
    });

    // Find camera by name
    let camera_transform = camera_query.iter_mut().find(|(_, name)| {
        name.as_str() == "3D Camera"
    });

    if let (Some((player_transform, _)), Some((mut camera_transform, _))) =
        (player_transform, camera_transform) {

        // Update camera position to maintain offset from player
        camera_transform.translation = player_transform.translation + CAMERA_OFFSET;
    }
}
