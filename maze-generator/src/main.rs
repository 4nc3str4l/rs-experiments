//! Shows how to render simple primitive shapes with a single color.

use bevy::{prelude::*, sprite::MaterialMesh2dBundle, window::PresentMode};
use rand::seq::SliceRandom;

const WINDOW_WIDTH: f32 = 640.;
const WINDOW_HEIGHT: f32 = 640.;

const MATRIX_WIDTH: usize = 15;
const MATRIX_HEIGHT: usize = 15;

const WALL_WIDTH: f32 = 4.0;

struct MazeCell {
    x: usize,
    y: usize,
    n_wall: bool,
    w_wall: bool,
    s_wall: bool,
    e_wall: bool
}

impl MazeCell {
    
    fn new(x: usize, y: usize) -> Self {
        Self {x, y, n_wall: true, w_wall: true, s_wall: true, e_wall: true}
    }

    fn process_connection(&mut self, connection: &(((usize, usize), (usize, usize)), bool)) {
        let mut other = (0 as usize, 0 as usize);
        if connection.0.0.0 == self.x && connection.0.0.1 == self.y {
            other = (connection.0.1.0, connection.0.1.1);
        } else {
            other = (connection.0.0.0, connection.0.0.1);
        }
        if self.x == other.0 {
            if self.y > other.1 {
                self.s_wall = false;
            }
            else {
                self.n_wall = false;
            }
        }
        else {
            if self.x > other.0 {
                self.e_wall = false;
            }
            else {
                self.w_wall = false;
            }
        }
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Maze Generator".into(),
                resolution: (640., 640.).into(),
                present_mode: PresentMode::AutoNoVsync,
                ..default()
            }),
            ..default()
        }))
        .insert_resource(ClearColor(Color::WHITE))
        .add_startup_system(setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());

    let tile_width = WINDOW_WIDTH / MATRIX_WIDTH as f32;
    let tile_height = WINDOW_HEIGHT / MATRIX_HEIGHT as f32;

    let connections = get_connections();
    let cells = construct_cells(&connections);

    for cell in cells {
        let x = cell.x;
        let y = cell.y;

        if cell.n_wall {
            commands.spawn(MaterialMesh2dBundle {
                mesh: meshes
                    .add(shape::Quad::new(Vec2::new(tile_width, WALL_WIDTH)).into())
                    .into(),
                material: materials.add(ColorMaterial::from(Color::BLACK)),
                transform: Transform::from_translation(Vec3::new(
                    -WINDOW_WIDTH / 2. + tile_width / 2.0 + tile_width * x as f32,
                    -WINDOW_HEIGHT / 2. + tile_height + tile_height * y as f32,
                    0.,
                )),
                ..default()
            });
        }
        if cell.s_wall {
            commands.spawn(MaterialMesh2dBundle {
                mesh: meshes
                    .add(shape::Quad::new(Vec2::new(tile_width, WALL_WIDTH)).into())
                    .into(),
                material: materials.add(ColorMaterial::from(Color::BLACK)),
                transform: Transform::from_translation(Vec3::new(
                    -WINDOW_WIDTH / 2. + tile_width / 2.0 + tile_width * x as f32,
                    -WINDOW_HEIGHT / 2. + tile_height * y as f32,
                    0.,
                )),
                ..default()
            });
        }
        if cell.e_wall {
            commands.spawn(MaterialMesh2dBundle {
                mesh: meshes
                    .add(shape::Quad::new(Vec2::new(WALL_WIDTH, tile_height)).into())
                    .into(),
                material: materials.add(ColorMaterial::from(Color::BLACK)),
                transform: Transform::from_translation(Vec3::new(
                    -WINDOW_HEIGHT / 2. + tile_width * x as f32,
                    -WINDOW_HEIGHT / 2. + tile_height / 2.0 + tile_height * y as f32,
                    0.,
                )),
                ..default()
            });
        }
        if cell.w_wall {
            commands.spawn(MaterialMesh2dBundle {
                mesh: meshes
                    .add(shape::Quad::new(Vec2::new(WALL_WIDTH, tile_height)).into())
                    .into(),
                material: materials.add(ColorMaterial::from(Color::BLACK)),
                transform: Transform::from_translation(Vec3::new(
                    -WINDOW_HEIGHT / 2. + tile_width + tile_width * x as f32,
                    -WINDOW_HEIGHT / 2. + tile_height / 2.0 + tile_height * y as f32,
                    0.,
                )),
                ..default()
            });
        }
        
    }
}

fn construct_cells(connections: &Vec<(((usize, usize), (usize, usize)), bool)>) -> Vec<MazeCell> {
    let mut to_return = Vec::with_capacity(MATRIX_WIDTH * MATRIX_HEIGHT);
    for y in 0..MATRIX_HEIGHT {
        for x in 0..MATRIX_WIDTH {
            to_return.push(MazeCell::new(x, y));
        }
    }
    for c in connections {
        if !c.1 {
            to_return[xy_to_idx(c.0.0.0, c.0.0.1)].process_connection(c);
            to_return[xy_to_idx(c.0.1.0, c.0.1.1)].process_connection(c);
        }
    }
    to_return
}

fn get_connections() -> Vec<(((usize, usize), (usize, usize)), bool)> {
    let mut data = Vec::with_capacity(MATRIX_HEIGHT * MATRIX_WIDTH);
    let mut connections: Vec<(((usize, usize), (usize, usize)), bool)> =
        Vec::with_capacity(data.capacity() * 2);
    for y in 0..MATRIX_HEIGHT {
        for x in 0..MATRIX_WIDTH {
            let current_idx = xy_to_idx(x, y);
            data.push(current_idx);

            if x < MATRIX_WIDTH - 1 {
                connections.push((((x, y), (x + 1, y)), true));
            }

            if y < MATRIX_HEIGHT - 1 {
                connections.push((((x, y), (x, y + 1)), true));
            }
        }
    }

    // Merge the adjacency list
    connections.shuffle(&mut rand::thread_rng());
    println!("{:?}", &data);

    while !all_connected(&data) {
        for conn in connections.iter_mut() {
            let el1 = xy_to_idx(conn.0 .0 .0, conn.0 .0 .1);
            let el2 = xy_to_idx(conn.0 .1 .0, conn.0 .1 .1);
            let v1 = *data.get(el1).unwrap();
            let v2 = *data.get(el2).unwrap();
            if v1 != v2 {
                conn.1 = false;
                replace_all(data.as_mut(), v2, v1);
            }
            if all_connected(&data) {
                break;
            }
        }
    }

    connections
}

fn all_connected(matrix: &[usize]) -> bool {
    let mut last = &matrix[0];
    for el in matrix.iter().skip(1) {
        if *el != *last {
            return false;
        }
        last = el;
    }
    true
}

fn replace_all(matrix: &mut Vec<usize>, n: usize, to: usize) {
    for el in matrix.iter_mut() {
        if *el == n {
            *el = to;
        }
    }
}

fn xy_to_idx(x: usize, y: usize) -> usize {
    x + y * MATRIX_WIDTH
}
