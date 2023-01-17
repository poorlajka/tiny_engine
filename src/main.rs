mod vec3;
mod collider;
mod cuboid;
mod transform;
mod gjk;
mod epa;
mod collision;
mod phys_state;
mod sphere;
mod body;
mod cylinder;
mod cone;
mod solver;
mod force_generator;
mod ode_solver;
mod oct_tree;

use collider::Collider;
use bevy::prelude::*;

pub const TICK_RATE: f32 = 0.01;

#[derive(Component)]
pub struct Box {
    id: usize,
    timer: Timer,
}

pub fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(phys_state::PhysState::new())
        .add_startup_system(spawn_basic_scene)
        .insert_resource(ClearColor(Color::BLACK))
        .add_startup_system(spawn_camera)
        .add_system(move_boxes)
        .run();
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y), 
        ..default()
    });
}

fn move_boxes(
    mut state: ResMut<phys_state::PhysState>,
    mut keyboard: Res<Input<KeyCode>>,
    mut commands: Commands,
    mut boxes: Query<(&mut Box, &mut Transform)>,
    time: Res<Time>,
) {
    for (mut b, mut transform) in &mut boxes {
        b.timer.tick(time.delta());
        if b.timer.just_finished() {
            if keyboard.pressed(KeyCode::Space) {
                let mut obj = state.get_body(0);
                obj.force += vec3::Vec3{x: 0.0, y: 2.5, z: 0.0};
                
            }
            if keyboard.pressed(KeyCode::A) {
                let mut obj = state.get_body(0);
                obj.force += vec3::Vec3{x: -2.5, y: 0.0, z: 0.0};
            }
            if keyboard.pressed(KeyCode::LShift) {
                let mut obj = state.get_body(0);
                obj.force += vec3::Vec3{x: 0.0, y: -2.5, z: 0.0};
            }
            if keyboard.pressed(KeyCode::D) {
                let mut obj = state.get_body(0);
                obj.force += vec3::Vec3{x: 2.5, y: 0.0, z: 0.0};
            }
            if keyboard.pressed(KeyCode::S) {
                let mut obj = state.get_body(0);
                obj.force += vec3::Vec3{x: 0.0, y: 0.0, z: 2.5};
            }
            if keyboard.pressed(KeyCode::W) {
                let mut obj = state.get_body(0);
                obj.force += vec3::Vec3{x: 0.0, y: 0.0, z: -2.5};
            }
            let obj = state.get_body(b.id);
            let v = obj.transform.position;
            let r = &obj.transform.orientation;
            transform.translation = Vec3{x: v.x, y: v.y, z: v.z};

            transform.rotation = *r;
        }
    }
    state.step(0.016, 1);
}

fn spawn_basic_scene(
    mut state: ResMut<phys_state::PhysState>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>
) { 
    let id1 = state.add_body(Collider::new_cuboid(vec3::Vec3{ x: 0.0, y: 1.0, z: 0.0}, 1.0, 1.0, 1.0), 10.0);
    let id2 = state.add_body(Collider::new_cuboid(vec3::Vec3{ x: 2.0, y: 1.0, z: 0.0}, 1.0, 1.0, 1.0), 10.0);
    //let id3 = state.add_body(Collider::new_sphere(vec3::Vec3{ x: -0.5, y: 1.0, z: 0.1}, 0.5), 10.0);

    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube{size : 1.0})),
        material: materials.add(Color::RED.into()),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..default()
    })
    .insert(Box {id: id1, timer: Timer::from_seconds(TICK_RATE, TimerMode::Repeating)})
    .insert(Name::new("Box1"));

    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube{size : 1.0})),
        material: materials.add(Color::PINK.into()),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..default()
    })
    .insert(Box {id: id2, timer: Timer::from_seconds(TICK_RATE, TimerMode::Repeating)})
    .insert(Name::new("Box1"));

    /*
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Icosphere{radius: 0.5, subdivisions: 12})),
        material: materials.add(Color::BLUE.into()),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..default()
    })
    .insert(Box {id: id3, timer: Timer::from_seconds(TICK_RATE, TimerMode::Repeating)})
    .insert(Name::new("Box1"));
    */

    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform:Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    })
    .insert(Name::new("Light"));
}
