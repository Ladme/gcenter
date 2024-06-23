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

const MAIN_REFERENCE: &str = "CNTR-Main";
const X_REFERENCE: &str = "CNTR-X";
const Y_REFERENCE: &str = "CNTR-Y";
const Z_REFERENCE: &str = "CNTR-Z";

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

    if args.xreference.is_none() && args.yreference.is_none() && args.zreference.is_none() {
        if args.reference == "Protein" {
            println!("[REFERENCE]     {}", &args.reference);
        } else {
            println!("[REFERENCE]     {}", &args.reference.bright_blue());
        }
    } else {
        for ((reference, name), dimension) in [&args.xreference, &args.yreference, &args.zreference]
            .into_iter()
            .zip(["[XREFERENCE]", "[YREFERENCE]", "[ZREFERENCE]"].into_iter())
            .zip([dim.is_x(), dim.is_y(), dim.is_z()].into_iter())
        {
            if !dimension {
                continue;
            }

            match reference {
                None => {
                    if args.reference == "Protein" {
                        println!("{}    {}", name, &args.reference);
                    } else {
                        println!("{}    {}", name, &args.reference.bright_blue());
                    }
                }
                Some(query) => println!("{}    {}", name, query.bright_blue()),
            }
        }
    }

    if !args.xdimension
        && !args.ydimension
        && !args.zdimension
        && args.xreference.is_none()
        && args.yreference.is_none()
        && args.zreference.is_none()
    {
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

/// Guess elements for target system printing warnings (if not silent) and returning errors.
fn guess_elements(
    system: &mut System,
    silent: bool,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    match system.guess_elements(Elements::default()) {
        Ok(_) => Ok(()),
        Err(ElementError::ElementGuessWarning(e)) => {
            if !silent {
                eprintln!("{}", ElementError::ElementGuessWarning(e));
            }
            Ok(())
        }
        Err(e) => Err(Box::from(e)),
    }
}

/// Get the dimensions and reference selections in which the centering should be performed.
fn construct_dimensions(args: &Args) -> Dimension {
    let refbool = [&args.xreference, &args.yreference, &args.zreference]
        .iter()
        .map(|x| x.is_some())
        .collect::<Vec<bool>>();

    match (
        [refbool[0], refbool[1], refbool[2]].into(),
        [args.xdimension, args.ydimension, args.zdimension].into(),
    ) {
        // if neither of the possible dimension specifications has been supplied, default to XYZ
        (Dimension::None, Dimension::None) => Dimension::XYZ,
        (a, b) => [
            a.is_x() || b.is_x(),
            a.is_y() || b.is_y(),
            a.is_z() || b.is_z(),
        ]
        .into(),
    }
}

/// Returns true if a query contains the "element" keyword or its alternatives.
fn query_contains_element(query: &str) -> bool {
    query.contains("element") || query.contains("elname") || query.contains("elsymbol")
}

/// Assign elements and masses to atoms if this is required.
fn guess_elements_masses(
    system: &mut System,
    args: &Args,
    input_file: FileType,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    if input_file == FileType::TPR {
        return Ok(());
    }

    if args.com {
        if !args.silent {
            println!("{} center of mass calculation requested; will guess elements and assign masses...\n", "note:".purple().bold());
        }

        return guess_elements(system, args.silent);
    }

    for reference in [&args.xreference, &args.yreference, &args.zreference]
        .into_iter()
        .flatten()
    {
        if query_contains_element(reference) {
            if !args.silent {
                println!(
                    "{} element keyword detected in a query; will guess elements...\n",
                    "note:".purple().bold()
                );
            }

            return guess_elements(system, args.silent);
        }
    }

    if query_contains_element(&args.reference) {
        if !args.silent {
            println!(
                "{} element keyword detected in a query; will guess elements...\n",
                "note:".purple().bold()
            );
        }

        return guess_elements(system, args.silent);
    }

    Ok(())
}

/// Perform the centering.
pub fn run() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let args = argparse::parse()?;

    if !args.silent {
        let version = format!("\n >> gcenter {} <<\n", env!("CARGO_PKG_VERSION"));
        println!("{}", version.bold());
    }

    let dim = construct_dimensions(&args);

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
    guess_elements_masses(&mut system, &args, input_file_type)?;

    // select reference atoms
    let operations = reference::create_references(&mut system, dim, &args)?;

    // perform centering
    center::center(&mut system, &args, operations)?;

    if !args.silent {
        let result = format!("Successfully written output file '{}'.", &args.output);
        println!("{}", result.green().bold());
    }

    Ok(())
}
