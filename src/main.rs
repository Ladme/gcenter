// Released under MIT License.
// Copyright (c) 2023 Ladislav Bartos

use gcenter;
use std::process;

fn main() {
    if let Err(e) = gcenter::run() {
        eprintln!("{}", e);
        process::exit(1);
    }

    process::exit(0);
}
