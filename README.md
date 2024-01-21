# echo_rust

**echo_rust** is a tool written in Rust that emulates the behavior of the `echo` command on Unix systems. It provides a straightforward way to print text to the standard output, with the additional option to control the output format.

## Dependencies

This program has been developed using Rust version 1.74.0. Command-line argument management has been simplified thanks to the use of the crate [clap](https://crates.io/crates/clap) in its version 4.4.11. `clap` offers a declarative interface for defining and parsing program arguments, making it easy to create a friendly and efficient user experience.

## Installation

1. Clone the repository: `git clone https://github.com/pepedeulloa/echo_rust.git`
2. Navigate to the project directory: `cd echo_rust`
3. Compile the project: `cargo build --release`

## Options

 - `-n, --newline`: Do not append the arguments on a new line.
 - `-h, --help`: Show the program's help.
 - `-V, --version`: Show the program's version.

## Usage

```bash
# Example of how to execute the program
cd target/release
./echo-rust --newline "Hello" "World"
