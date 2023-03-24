# Maze Generator:

The following code allows you to create random maze using the [https://en.wikipedia.org/wiki/Maze_generation_algorithm#Randomized_Kruskal's_algorithm](Kruskal Algorithm)

You can change the following parameters:

```rust

// Dimensions of the window (application)
const WINDOW_WIDTH: f32 = 640.;
const WINDOW_HEIGHT: f32 = 640.;

// Dimensions of the maze num columns and rows
const MATRIX_WIDTH: usize = 20;
const MATRIX_HEIGHT: usize = 20;

// How big are the walls
const WALL_WIDTH: f32 = 2.0;

```

The algorithm produces the following results:


