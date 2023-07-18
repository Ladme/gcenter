// Released under MIT License.
// Copyright (c) 2023 Ladislav Bartos

//! Rust program for centering any group of atoms into the center of a simulation box.
//! Works with gromacs gro structure files and gromacs xtc trajectories.
//! Only supports orthogonal boxes!
//!
//! ## Basic usage
//! ```
//! gcenter -c system.gro -f trajectory.xtc -o output_trajectory.xtc
//! ```
//!
//! Use `./gcenter --help` for more information about using this program.

use gcenter;
use std::process;

fn main() {
    if let Err(e) = gcenter::run() {
        eprintln!("{}", e);
        process::exit(1);
    }

    process::exit(0);
}
