# Maze Generator:

The following code allows you to create random maze using the [Kruskal Algorithm](https://en.wikipedia.org/wiki/Maze_generation_algorithm#Randomized_Kruskal's_algorithm)

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

20x20 maze
![20x20](https://github.com/4nc3str4l/rust-experiments/blob/main/maze-generator/resources/20_20.PNG?raw=true)

40x40 maze
![40x40](https://github.com/4nc3str4l/rust-experiments/blob/main/maze-generator/resources/40_40.PNG?raw=true)



## How to improve the code:

- Minimize the wall sections by checking wich ones could be merged.
- Make the connections in paralel.
- Allow the user to change the size and regenerate with a UI.
- Add entry and exit markers.
- Make the game playable.

....