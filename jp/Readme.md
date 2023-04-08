# Code Description

This is a Rust program that searches for a directory on the user's desktop and opens it in Visual Studio Code. It utilizes the `walkdir` and `winreg` crates to search for the directory and the `std::process` module to launch Visual Studio Code.

## Instructions

To run the program, you need to have Rust and Visual Studio Code installed on your system.

1. Clone the repository to your local machine.
2. Open a terminal and navigate to the cloned repository.
3. Run `cargo build --release` to build the project with optimizations.
4. Run `./target/release/jp "<directory_name>"` to search for the directory on your desktop and open it in Visual Studio Code. Replace `<directory_name>` with the name of the directory you want to search for on your desktop.

**Note**: The program expects Visual Studio Code to be installed at a specific location, which is defined by the `VSCODE_LOCATION` constant in the code. If Visual Studio Code is installed in a different location on your system, you need to update the constant with the correct path.

You can also install the program globally on your system by running `cargo install --path .` in the root directory of the repository. Once installed, you can run the program from any directory by typing `jp` in the terminal.

## Contributing

If you want to contribute to the project, feel free to submit a pull request or open an issue on the GitHub repository.
