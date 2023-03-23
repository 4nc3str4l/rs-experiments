//! Shows how to render simple primitive shapes with a single color.
use bevy::{prelude::*, sprite::MaterialMesh2dBundle, window::PresentMode};

const WINDOW_WIDTH: f32 = 640.;
const WINDOW_HEIGHT: f32 = 640.;

const MATRIX_WIDTH: usize = 15;
const MATRIX_HEIGHT: usize = 15;

const WALL_WIDTH: f32 = 4.0;

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

    let matrix = construct_matrix();

    for y in 0..MATRIX_HEIGHT {
        for x in 0..MATRIX_WIDTH {
            // North Wall
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

            // East Wall
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

fn construct_matrix() -> Vec<usize> {
    let mut data = Vec::with_capacity((MATRIX_HEIGHT) * (MATRIX_WIDTH));
    let mut connections: Vec<(usize, usize)> = Vec::with_capacity(data.capacity() * 4);
    for y in 0..MATRIX_HEIGHT {
        for x in 0..MATRIX_WIDTH {
            let current_idx = xy_to_index(x, y); 
            data.push(current_idx);
            
            if x < MATRIX_WIDTH - 1 {
                connections.push((current_idx, xy_to_index(x + 1, y)));
            }

            if y < MATRIX_HEIGHT - 1 {
                connections.push((current_idx, xy_to_index(x, y + 1)));
            }
        }
    }

    data
}

fn xy_to_index(x: usize, y: usize) -> usize {
    x + y * MATRIX_WIDTH
}
