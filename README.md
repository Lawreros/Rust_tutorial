# Rust_tutorial

## Useful Functions/Libraries I've added to Rust
- [Cargo-sweep](https://github.com/holmgr/cargo-sweep): Cleans up old fingerpring and compiling files that can take up a lot of space on projects that are frequently built
- 

## Important command line commands
- ```cargo new``` makes a new cargo project with accompanying git files (.gitignore the target directory). If inside of existing github repo, will skip git steps
- ```cargo run``` does both ```cargo build``` and runs the resulting program
- ```cargo check``` quickly checks code to make sure ti compiles, but doesn't produce an executible
- ```cargo build --release``` compiles program with optimizations and puts the executable in the target/release directory
- ```cargo doc --open``` builds documentation provided by all of the dependencies locally and opens it in a browser