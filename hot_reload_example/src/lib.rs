use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use hot_reload::{reload_macros::hot_reload_setup, *};

pub fn bevy_main() {
    println!("Creating app");
    let mut app = App::new();
    app.add_plugins(DefaultPlugins)
        .add_plugins(WorldInspectorPlugin::new())
        .add_plugins(HotReloadPlugin::default())
        .add_systems(Startup, setup)
        .add_reloadables::<reloadable>();

    app.run();
}

#[hot_reload_setup]
fn reloadable(app: &mut ReloadableApp) {
    app.add_systems(Update, move_cube);
}

#[derive(Component)]
struct Cube;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // plane
    commands.spawn(PbrBundle {
        mesh: meshes.add(shape::Plane::from_size(5.0).into()),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..default()
    });
    // cube
    commands.spawn((
        Cube,
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            ..default()
        },
    ));
    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
    // camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

fn move_cube(mut cubes: Query<&mut Transform, With<Cube>>, time: Res<Time>) {
    let x_position = 2. * time.elapsed_seconds().sin();

    for mut cube in cubes.iter_mut() {
        cube.translation.x = x_position;
    }
}
