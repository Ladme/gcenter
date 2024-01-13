// Released under MIT License.
// Copyright (c) 2023 Ladislav Bartos

mod argparse;
mod center;
mod errors;
mod reference;

use colored::Colorize;
use groan_rs::prelude::*;
use groan_rs::errors::ElementError;
use std::path::Path;

use argparse::Args;
use errors::RunError;

/// Print options specified for the centering. Non-default values are colored in blue.
fn print_options(args: &Args, system: &System, dim: &Dimension) {
    println!("[STRUCTURE]     {}", &args.structure.bright_blue());

    match args.trajectories.len() {
        0 => (),
        1 => println!("[TRAJECTORY]    {}", args.trajectories[0].bright_blue()),
        _ => {
            print!("[TRAJECTORIES]  ");
            println!("{}", args.trajectories[0].bright_blue());
            for traj in args.trajectories.iter().skip(1) {
                println!("                {}", traj.bright_blue());
            }
        }
    }

    println!("[OUTPUT]        {}", &args.output.bright_blue());

    if args.index.is_some() {
        println!(
            "[INDEX]         {}",
            &args.index.clone().unwrap().bright_blue()
        );
    } else if system.get_n_groups() > 2 {
        println!("[INDEX]         index.ndx");
    }

    if args.reference == "Protein" {
        println!("[REFERENCE]     {}", &args.reference);
    } else {
        println!("[REFERENCE]     {}", &args.reference.bright_blue());
    }

    if !args.xdimension && !args.ydimension && !args.zdimension {
        println!("[DIMENSIONS]    {}", dim);
    } else {
        println!("[DIMENSIONS]    {}", dim.to_string().bright_blue());
    }

    if let Some(s) = args.start_time {
        let time = format!("{} ns", s / 1000.0);
        println!("[START TIME]    {}", time.bright_blue());
    }

    if let Some(e) = args.end_time {
        let time = format!("{} ns", e / 1000.0);
        println!("[END TIME]      {}", time.bright_blue());
    }

    if args.step != 1 {
        println!("[STEP]          {}", &args.step.to_string().bright_blue());
    }

    if args.com {
        println!("[METHOD]        {}", "center of mass".bright_blue());
    } else {
        println!("[METHOD]        {}", "center of geometry");
    }

    println!();
}

/// Perform the centering.
pub fn run() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let args = argparse::parse()?;

    if !args.silent {
        let version = format!("\n >> gcenter {} <<\n", env!("CARGO_PKG_VERSION"));
        println!("{}", version.bold());
    }

    // construct a dimension; if no dimension has been chosen, use all of them
    let dim: Dimension = match [args.xdimension, args.ydimension, args.zdimension].into() {
        Dimension::None => Dimension::XYZ,
        other => other,
    };

    // read structure file
    let mut system = System::from_file(&args.structure)?;

    // check the simulation is valid (defined, non-zero and orthogonal)
    match system.get_box_as_ref() {
        None => return Err(Box::from(RunError::BoxNotDefined)),
        Some(x) => {
            if x.is_zero() {
                return Err(Box::from(RunError::BoxNotValid));
            }

            if !x.is_orthogonal() {
                return Err(Box::from(RunError::BoxNotOrthogonal));
            }
        }
    };

    // read ndx file
    system.read_ndx_with_default(&args.index, "index.ndx")?;

    // print options
    if !args.silent {
        print_options(&args, &system, &dim);
    }

    // backup the output
    if Path::new(&args.output).exists() {
        if !args.overwrite {
            let backup = backitup::backup(&args.output)?;

            if !args.silent {
                println!(
                    "{} backed up '{}' as '{}'\n",
                    "note:".purple().bold(),
                    &args.output.yellow(),
                    backup.to_str().unwrap().yellow()
                );
            }
        } else if !args.silent {
            println!(
                "{} overwriting '{}'\n",
                "warning:".yellow().bold(),
                &args.output.yellow()
            );
        }
    }

    // select reference atoms
    reference::create_reference(&mut system, &args)?;

    // guess elements and assign masses, if needed
    if args.com {
        if !args.silent {
            println!("{} center of mass calculation requested; will guess elements and assign masses...\n", "note:".purple().bold());
        }

        match system.guess_elements(Elements::default()) {
            Ok(_) => (),
            Err(ElementError::ElementGuessWarning(e)) => eprintln!("{}", ElementError::ElementGuessWarning(e)),
            Err(e) => return Err(Box::from(e)),
        }
    }

    // perform centering
    center::center(&mut system, &args, dim)?;

    if !args.silent {
        let result = format!("Successfully written output file '{}'.", &args.output);
        println!("{}", result.green().bold());
    }

    Ok(())
}
