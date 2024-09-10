#![allow(unused)]

use std::usize;

use bevy::sprite::{Wireframe2dConfig, Wireframe2dPlugin};
use bevy::window::WindowTheme;
use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};

fn main() {
    let mut app = App::new();
    app.add_plugins((
        DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Game Of Life".into(),
                resolution: SCREEN_RESOLUTION.into(),
                window_theme: Some(WindowTheme::Dark),
                resizable: true,
                ..default()
            }),
            ..default()
        }),
        Wireframe2dPlugin,
        CellularAutomataPlugin,
    ))
    .add_systems(Startup, setup)
    .run();
}

const SCREEN_RESOLUTION: (f32, f32) = (900., 900.);
const SQUER_SIZE: f32 = 10.0;
const NUMBER: u16 = 101;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut row: ResMut<ActiveRow>,
) {
    commands.spawn(Camera2dBundle::default());
    drew_row(&mut commands, &mut meshes, &mut materials, &row.0, 0);
}

#[derive(Resource)]
struct RowCount(usize);

#[derive(Resource)]
struct ActiveRow(Vec<u8>);

impl ActiveRow {
    fn new() -> Self {
        let n_squer = (SCREEN_RESOLUTION.0 / SQUER_SIZE) as usize;
        let mut base: Vec<u8> = vec![0_u8; n_squer];
        let len = base.len();
        base[(len / 2) as usize] = 1;
        Self(base)
    }
}

#[derive(Resource)]
struct RefreshTimer(Timer);

struct CellularAutomataPlugin;

impl Plugin for CellularAutomataPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ActiveRow::new())
            .insert_resource(RowCount(1))
            .add_systems(Update, add_row);
    }
}

fn add_row(
    mut commands: Commands,
    mut row: ResMut<ActiveRow>,
    mut row_count: ResMut<RowCount>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let new_row = compute_new_row(&row.0);
    drew_row(
        &mut commands,
        &mut meshes,
        &mut materials,
        &new_row,
        row_count.0,
    );
    row_count.0 += 1;
    row.0 = new_row.clone();
}

fn drew_row(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    row: &Vec<u8>,
    row_count: usize,
) {
    for (i, s) in row.iter().enumerate() {
        let mesh = Mesh2dHandle(meshes.add(Rectangle::new(SQUER_SIZE, SQUER_SIZE)));
        let material = materials.add(Color::hsl(0., 1., 1. - (*s as f32)));
        let (x, y) = compute_location(i, row_count);
        let transform = Transform::from_xyz(x, y, 0.);
        commands.spawn(MaterialMesh2dBundle {
            mesh,
            material,
            transform,
            ..default()
        });
    }
}

fn compute_location(x: usize, y: usize) -> (f32, f32) {
    let offset = (SCREEN_RESOLUTION.0 / 2.) - SQUER_SIZE;
    (
        -1. * offset + (SQUER_SIZE * (x as f32)),
        offset - (SQUER_SIZE * (y as f32)),
    )
}

fn compute_new_row(row: &Vec<u8>) -> Vec<u8> {
    let role: Vec<u8> = format!("{:08b}", NUMBER)
        .chars()
        .map(|i| i.to_digit(2).unwrap() as u8)
        .collect();
    let mut new: Vec<u8> = Vec::from([0]);
    for (i, n) in row[1..row.len() - 1].iter().enumerate() {
        let slice = [row[i], row[i + 1], row[i + 2]];
        let index = slice_to_usize(&slice);
        new.push(role[7 - index])
    }
    new.push(0);
    new
}

fn slice_to_usize(slice: &[u8]) -> usize {
    let binary_str = format!("{}{}{}", slice[0], slice[1], slice[2]);
    usize::from_str_radix(&binary_str, 2).unwrap()
}
