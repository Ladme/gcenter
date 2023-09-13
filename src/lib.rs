// Released under MIT License.
// Copyright (c) 2023 Ladislav Bartos

use clap::Parser;
use colored::{ColoredString, Colorize};
use std::io::{self, Write};
use std::path::Path;
use thiserror::Error;

use groan_rs::files::FileType;
use groan_rs::prelude::*;

/// Frequency of printing during analysis of an xtc file.
const PRINT_FREQ: u64 = 500000;

// Center Gromacs trajectory or structure file.
#[derive(Parser, Debug)]
#[command(
    author,
    version,
    about,
    long_about = "Center your chosen group within a Gromacs trajectory or structure file effortlessly using the Bai & Breen algorithm.\n
With `gcenter`, you can accurately center atom groups, even when they span multiple molecules that may extend beyond the box boundaries.
Note that `gcenter` does not employ connectivity information, so it doesn't require a tpr file as input. 
However, it also lacks the capability to wrap molecules into the simulation box.
Be aware that `gcenter` exclusively supports orthogonal simulation boxes."
)]
pub struct Args {
    #[arg(
        short = 'c',
        long = "structure",
        help = "Structure file to read",
        long_help = "Gro or pdb file containing the system structure. If trajectory is also supplied, coordinates from the structure file are ignored."
    )]
    structure: String,

    #[arg(
        short = 'f',
        long = "trajectory",
        help = "Trajectory file to read",
        long_help = "Xtc or trr file containing the trajectory which should be manipulated. If not provided, the structure file itself will be centered."
    )]
    trajectory: Option<String>,

    #[arg(
        short = 'n',
        long = "index",
        help = "Index file to read [default: index.ndx]",
        long_help = "Ndx file containing the groups associated with the system.\n\n[default: index.ndx]"
    )]
    index: Option<String>,

    #[arg(
        short = 'o',
        long = "output",
        help = "Name of the output file",
        long_help = "Output gro, pdb (if no trajectory file is provided), xtc, or trr file."
    )]
    output: String,

    #[arg(
        short = 'r',
        long = "reference",
        help = "Group to center",
        default_value = "Protein",
        long_help = "Specification of the group that should be centered. Use VMD-like `groan selection language` to specify the group. Groan selection language supports ndx group names."
    )]
    reference: String,

    #[arg(
        short = 's',
        long = "step",
        help = "Write every <STEP>th frame",
        default_value_t = 1,
        requires = "trajectory",
        long_help = "Only every <STEP>th frame of the trajectory will be centered and written into the output file."
    )]
    step: usize,

    #[arg(
        short = 'x',
        action,
        help = "Center in x dimension",
        default_value_t = false,
        long_help = "Center the selected group in the x-dimension only. Can be combined with other dimensions. If no dimension is selected, defaults to `-xyz`."
    )]
    xdimension: bool,

    #[arg(
        short = 'y',
        action,
        help = "Center in y dimension",
        default_value_t = false,
        long_help = "Center the selected group in the y-dimension only. Can be combined with other dimensions. If no dimension is selected, defaults to `-xyz`."
    )]
    ydimension: bool,

    #[arg(
        short = 'z',
        action,
        help = "Center in z dimension",
        default_value_t = false,
        long_help = "Center the selected group in the z-dimension only. Can be combined with other dimensions. If no dimension is selected, defaults to `-xyz`."
    )]
    zdimension: bool,

    #[arg(
        long = "silent",
        action,
        help = "Do not print any output to stdout",
        default_value_t = false,
        long_help = "\"Be silent! Keep your forked tongue behind your teeth.\" Setting this flag will restrict `gcenter` from writing anything to the standard output, apart from errors which are written into stderr."
    )]
    silent: bool,

    #[arg(
        long = "overwrite",
        action,
        help = "Overwrite any file sharing the name with the output file",
        default_value_t = false,
        long_help = "Setting this flag will restrict `gcenter` from creating backup for the file sharing the name with the output file. Any such file will be overwritten."
    )]
    overwrite: bool,
}

/// Errors originating directly from `gcenter`.
#[derive(Error, Debug, PartialEq, Eq)]
pub enum RunError {
    #[error("{} structure file '{}' is the same file as the output file", "error:".red().bold(), .0.yellow())]
    IOMatchStructure(String),
    #[error("{} trajectory file '{}' is the same file as the output file", "error:".red().bold(), .0.yellow())]
    IOMatchTrajectory(String),
    #[error("{} reference group '{}' is empty", "error:".red().bold(), .0.yellow())]
    EmptyReference(String),
    #[error("{} no protein atoms autodetected", "error:".red().bold())]
    AutodetectionFailed,
    #[error("{} simulation box is not orthogonal; this is not supported, sorry", "error:".red().bold())]
    BoxNotOrthogonal,
    #[error("{} simulation box is not a valid simulation box; some required dimensions are not positive", "error:".red().bold())]
    BoxNotValid,
    #[error("{} output file '{}' has unsupported file extension", "error:".red().bold(), .0.yellow())]
    UnsupportedFileExtension(String),
}

/// Check that the input and output files are not identical.
/// This protects the user from accidentaly overwriting their data.
fn sanity_check_files(args: &Args) -> Result<(), RunError> {
    if args.trajectory.is_none() {
        if args.structure == args.output {
            return Err(RunError::IOMatchStructure(args.structure.to_string()));
        }
    } else if *args.trajectory.as_ref().unwrap() == args.output {
        return Err(RunError::IOMatchTrajectory(
            args.trajectory.as_ref().unwrap().to_string(),
        ));
    }

    Ok(())
}

/// Center the reference group and write an output gro or pdb file.
fn center_structure_file(
    system: &mut System,
    reference: &str,
    output: &str,
    output_type: FileType,
    dimension: Dimension,
) -> Result<(), Box<dyn std::error::Error>> {
    system.atoms_center(reference, dimension)?;

    match output_type {
        FileType::GRO => system.write_gro(output, system.has_velocities())?,
        FileType::PDB => system.write_pdb(output)?,
        _ => {
            return Err(Box::new(RunError::UnsupportedFileExtension(
                output.to_string(),
            )))
        }
    }

    Ok(())
}

/// Print progress of the analysis
fn print_progress(
    sim_step: u64,
    sim_time: u64,
    step_fmt: &ColoredString,
    time_fmt: &ColoredString,
    status: &ColoredString,
) {
    print!(
        "[{: ^9}]   {} {:12} | {} {:9} ps\r",
        status, step_fmt, sim_step, time_fmt, sim_time
    );
    io::stdout().flush().unwrap();
}

/// Center the reference group for every frame of the xdr file and write output xdr file.
fn center_xdr_file<'a, Reader, Writer>(
    system: &'a mut System,
    reference: &str,
    trajectory: impl AsRef<Path>,
    output: impl AsRef<Path>,
    step: usize,
    dimension: Dimension,
    silent: bool,
) -> Result<(), Box<dyn std::error::Error>>
where
    Writer: XdrWriter,
    Reader: XdrReader<'a>,
{
    let mut writer = Writer::new(system, output)?;
    let iterator = Reader::new(system, trajectory)?;

    let colored_step = "Step:".cyan();
    let colored_time = "Time:".bright_purple();
    let colored_running = "CENTERING".yellow();

    // iterate through the input trajectory file
    for (curr_step, raw_frame) in iterator.enumerate() {
        let frame = raw_frame?;

        if !silent && frame.get_simulation_step() % PRINT_FREQ == 0 {
            print_progress(
                frame.get_simulation_step(),
                frame.get_simulation_time() as u64,
                &colored_step,
                &colored_time,
                &colored_running,
            );
        }

        if curr_step % step == 0 {
            frame.atoms_center(reference, dimension)?;
            writer.write_frame()?;
        }
    }

    if !silent {
        println!("[{: ^9}]\n", &"COMPLETED".green());
    }

    Ok(())
}

/// Print options specified for the centering. Non-default values are colored in blue.
fn print_options(args: &Args, system: &System, dim: &Dimension) {
    println!("[STRUCTURE]   {}", &args.structure.bright_blue());

    if args.trajectory.is_some() {
        println!(
            "[TRAJECTORY]  {}",
            &args.trajectory.clone().unwrap().bright_blue()
        );
    }

    println!("[OUTPUT]      {}", &args.output.bright_blue());

    if args.index.is_some() {
        println!(
            "[INDEX]       {}",
            &args.index.clone().unwrap().bright_blue()
        );
    } else if system.get_n_groups() > 2 {
        println!("[INDEX]       index.ndx");
    }

    if args.reference == "Protein" {
        println!("[REFERENCE]   {}", &args.reference);
    } else {
        println!("[REFERENCE]   {}", &args.reference.bright_blue());
    }

    if !args.xdimension && !args.ydimension && !args.zdimension {
        println!("[DIMENSIONS]  {}", dim);
    } else {
        println!("[DIMENSIONS]  {}", dim.to_string().bright_blue());
    }

    if args.step != 1 {
        println!("[STEP]        {}", &args.step.to_string().bright_blue());
    } else {
        println!("[STEP]        {}", &args.step);
    }

    println!();
}

/// Perform the centering.
pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    if !args.silent {
        let version = format!("\n >> gcenter {} <<\n", env!("CARGO_PKG_VERSION"));
        println!("{}", version.bold());
    }

    // check that the input file is not the same as the output file
    sanity_check_files(&args)?;

    // construct a dimension; if no dimension has been chosen, use all of them
    let dim: Dimension = match [args.xdimension, args.ydimension, args.zdimension].into() {
        Dimension::None => Dimension::XYZ,
        other => other,
    };

    // read structure file
    let mut system = System::from_file(&args.structure)?;

    // check that box has positive dimensions
    if !system.get_box_as_ref().is_valid() {
        return Err(Box::from(RunError::BoxNotValid));
    }

    // check that the simulation box is orthogonal
    if !system.get_box_as_ref().is_orthogonal() {
        return Err(Box::from(RunError::BoxNotOrthogonal));
    }

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
    let autodetect = match system.group_create("Reference", &args.reference) {
        Ok(_) => false,
        Err(e) => {
            if &args.reference == "Protein" {
                if !args.silent {
                    println!(
                        "{} group '{}' not found. Autodetecting protein atoms...\n",
                        "warning:".yellow().bold(),
                        "Protein".yellow()
                    );
                }

                system
                    .group_create("Reference", "@protein")
                    .expect("gcenter: Fatal Error. Autodetection failed.");
                true
            } else {
                return Err(e);
            }
        }
    };

    // check that the reference group is not empty
    if system.group_get_n_atoms("Reference").unwrap() == 0 {
        if !autodetect {
            return Err(Box::new(RunError::EmptyReference(args.reference)));
        } else {
            return Err(Box::new(RunError::AutodetectionFailed));
        }
    }

    // determine type of the output file
    let output_type = FileType::from_name(&args.output);

    match args.trajectory {
        // trajectory file not provided, use gro or pdb file
        None => center_structure_file(&mut system, "Reference", &args.output, output_type, dim)?,

        // use trajectory file
        Some(ref file) => {
            // determine the type of the input trajectory file
            let input_type = FileType::from_name(file);

            match (input_type, output_type) {
                (FileType::XTC, FileType::XTC) => center_xdr_file::<XtcReader, XtcWriter>(
                    &mut system,
                    "Reference",
                    file,
                    &args.output,
                    args.step,
                    dim,
                    args.silent,
                )?,

                (FileType::XTC, FileType::TRR) => center_xdr_file::<XtcReader, TrrWriter>(
                    &mut system,
                    "Reference",
                    file,
                    &args.output,
                    args.step,
                    dim,
                    args.silent,
                )?,

                (FileType::TRR, FileType::XTC) => center_xdr_file::<TrrReader, XtcWriter>(
                    &mut system,
                    "Reference",
                    file,
                    &args.output,
                    args.step,
                    dim,
                    args.silent,
                )?,

                (FileType::TRR, FileType::TRR) => center_xdr_file::<TrrReader, TrrWriter>(
                    &mut system,
                    "Reference",
                    file,
                    &args.output,
                    args.step,
                    dim,
                    args.silent,
                )?,

                _ => return Err(Box::new(RunError::UnsupportedFileExtension(args.output))),
            }
        }
    }

    if !args.silent {
        let result = format!("Successfully written output file '{}'.", &args.output);
        println!("{}", result.green().bold());
    }

    Ok(())
}
