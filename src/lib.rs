// Released under MIT License.
// Copyright (c) 2023-2024 Ladislav Bartos

mod argparse;
mod center;
mod errors;
mod reference;

use colored::Colorize;
use groan_rs::errors::ElementError;
use groan_rs::files::FileType;
use groan_rs::structures::dimension::Dimension;
use groan_rs::structures::element::Elements;
use groan_rs::system::System;
use std::path::Path;

use argparse::Args;

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
    }

    if args.whole {
        println!("[WHOLE]         {}", "molecules".bright_blue())
    }

    println!();
}

/// Guess elements for target system printing warnings and returning errors.
fn guess_elements(system: &mut System) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    match system.guess_elements(Elements::default()) {
        Ok(_) => Ok(()),
        Err(ElementError::ElementGuessWarning(e)) => {
            eprintln!("{}", ElementError::ElementGuessWarning(e));
            Ok(())
        }
        Err(e) => Err(Box::from(e)),
    }
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

    // guess elements and assign masses, if needed
    let input_file_type = FileType::from_name(&args.structure);
    if input_file_type != FileType::TPR {
        if args.com {
            if !args.silent {
                println!("{} center of mass calculation requested; will guess elements and assign masses...\n", "note:".purple().bold());
            }

            guess_elements(&mut system)?;
        } else if args.reference.contains("element")
            || args.reference.contains("elname")
            || args.reference.contains("elsymbol")
        {
            if !args.silent {
                println!(
                    "{} element keyword detected in query; will guess elements...\n",
                    "note:".purple().bold()
                );
            }

            guess_elements(&mut system)?;
        }
    }

    // select reference atoms
    reference::create_reference(&mut system, &args)?;

    // perform centering
    center::center(&mut system, &args, dim)?;

    if !args.silent {
        let result = format!("Successfully written output file '{}'.", &args.output);
        println!("{}", result.green().bold());
    }

    Ok(())
}
