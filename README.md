# Rust AI Search

This repository is a Rust multi-crate workspace for AI projects and experimentation.

## Workspace Structure
- `Cargo.toml`: workspace configuration and global dependencies
- `crates/`: contains all project crates
	- Each subfolder in `crates/` is an independent crate (e.g., `ai_vec_hybrid`, `ai_vm`, `mycelium`, ...)
	- Each crate has its own `Cargo.toml` and `src/` folder

## Useful Commands
- `cargo build`: build all crates in the workspace
- `cargo run -p <crate_name>`: run the specified crate
- `cargo test -p <crate_name>`: run tests for a crate
- `cargo check`: check compilation without generating a binary

## Prerequisites
- [Rust](https://www.rust-lang.org/tools/install) must be installed

---

Feel free to edit this file to add crate-specific information or global workspace documentation.
