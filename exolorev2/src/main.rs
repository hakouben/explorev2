use bevy::prelude::*;
use bevy::window::PresentMode;
use bevy::window::WindowTheme;
use bevy::{
    math::vec2,
    window::{PrimaryWindow, Window},
};
use rand::random;
use rand::seq::SliceRandom;
use std::collections::HashSet;
pub const NUMBER_OF_OBSTACLES: usize = 5;
pub const ROBOT_SPEED: f32 = 100.0;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "I am a window!".into(),
                name: Some("bevy.app".into()),
                resolution: (1920., 1080.).into(),
                present_mode: PresentMode::AutoVsync,

                window_theme: Some(WindowTheme::Dark),

                visible: true,
                ..default()
            }),
            ..default()
        }),))
        .add_systems(Startup, spawn_station)
        .add_systems(Startup, spawn_camera)
        .add_systems(Startup, spawn_obstacle)
        .add_systems(Startup, spawn_robot)
        .add_systems(Update, robot_targeting)
        .add_systems(Update, robot_movement)
        .run();
}

#[derive(Component)]
pub struct Robot {
    id: u32,
    direction: Vec2,
    position: (f32, f32),
    energie: f32,
    minerais: f32,
    points_interet_scientifiques: Vec<(f32, f32)>,
    modules: Vec<Module>,
    target_obstacle: Option<Entity>,
}

#[derive(Component)]
pub enum Module {
    AnalyseChimique,
    Forage,
    ImagerieHauteResolution,
}

#[derive(Component)]
pub struct Station {
    position: Vec2,
    energie: f32,
    minerais: f32,
    donnees_scientifiques: Vec<Donnee>,
    robots: Vec<Robot>,
}

#[derive(Component)]
pub struct Obstacle {
    position: Vec2,
    direction: Vec2,
    taille: (f32, f32),
}

#[derive(Component)]
pub struct Donnee {
    id: u32,
    valeur: String,
    id_robot: u32,
}

pub fn spawn_station(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window: &Window = window_query.get_single().unwrap();

    commands.spawn(SpriteBundle {
        texture: asset_server.load(".\\station.png"),

        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        ..Default::default()
    });
}
pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window: &Window = window_query.get_single().unwrap();

    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        ..Default::default()
    });
}

pub fn spawn_obstacle(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window: &Window = window_query.get_single().unwrap();
    for _ in 0..NUMBER_OF_OBSTACLES {
        let random_x = random::<f32>() * window.width();
        let random_y = random::<f32>() * window.height();

        commands.spawn(SpriteBundle {
            texture: asset_server.load(".\\40789.png"),

            transform: Transform::from_xyz(random_x, random_y, 0.0),
            ..Default::default()
        });
    }
}
pub fn spawn_robot(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window: &Window = window_query.get_single().unwrap();

    commands
        .spawn(SpriteBundle {
            texture: asset_server.load(".\\robot 2 pink .png"),

            transform: Transform::from_xyz(window.width() / 3.0, window.height() / 2.0, 0.0),
            ..Default::default()
        })
        .insert(Robot {
            position: (window.width() / 2.0, window.height() / 2.0),
            id: 0,
            direction: vec2(1.0, 1.0),
            energie: 100.0,
            minerais: 0.0,
            points_interet_scientifiques: vec![],
            modules: vec![],
            target_obstacle: Default::default(),
        });
}
fn get_nearest_obstacle(robot_position: Vec3, obstacles: &[(Transform, Obstacle)]) -> Vec3 {
    obstacles
        .iter()
        .min_by(|a, b| {
            let distance_a = a.0.translation.distance(robot_position);
            let distance_b = b.0.translation.distance(robot_position);
            distance_a.partial_cmp(&distance_b).unwrap()
        })
        .unwrap()
        .0
        .translation
}

fn robot_targeting(
    mut robot_query: Query<(&mut Robot, &Transform)>,
    obstacle_query: Query<(Entity, &Transform), With<Obstacle>>,
) {
    let obstacles: Vec<(Entity, Vec3)> = obstacle_query
        .iter()
        .map(|(entity, transform)| (entity, transform.translation))
        .collect();

    let mut targeted_obstacles = HashSet::new();

    for (mut robot, transform) in robot_query.iter_mut() {
        if let Some(target) = robot.target_obstacle {
            if targeted_obstacles.contains(&target) {
                robot.target_obstacle = None;
            } else {
                targeted_obstacles.insert(target);
            }
        }

        if robot.target_obstacle.is_none() {
            let available_obstacles = obstacles
                .iter()
                .filter(|(entity, _)| !targeted_obstacles.contains(entity))
                .collect::<Vec<_>>();

            if let Some(&(target, _)) = available_obstacles.choose(&mut rand::thread_rng()) {
                robot.target_obstacle = Some(*target);
                targeted_obstacles.insert(*target);
            }
        }
    }
}

fn robot_movement(
    time: Res<Time>,
    mut robot_query: Query<(&mut Transform, &Robot)>,
    obstacle_query: Query<(Entity, &Transform), With<Obstacle>>,
) {
    let obstacles: Vec<(Entity, Vec3)> = obstacle_query
        .iter()
        .map(|(entity, transform)| (entity, transform.translation))
        .collect();

    for (mut transform, robot) in robot_query.iter_mut() {
        if let Some(target) = robot.target_obstacle {
            let target_position = obstacles
                .iter()
                .find(|&(entity, _)| *entity == target)
                .map(|&(_, position)| position)
                .unwrap();
            let direction = (target_position - transform.translation).normalize();
            transform.translation += direction * ROBOT_SPEED * time.delta_seconds();
        }
    }
}
