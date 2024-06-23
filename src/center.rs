// Released under MIT License.
// Copyright (c) 2023-2024 Ladislav Bartos

//! Implementation of the centering procedure.

use colored::Colorize;
use groan_rs::files::FileType;
use groan_rs::io::traj_cat::TrajConcatenator;
use groan_rs::io::traj_io::{
    FrameDataTime, TrajMasterRead, TrajRangeRead, TrajRead, TrajReader, TrajStepRead, TrajWrite,
};
use groan_rs::io::trr_io::{TrrReader, TrrWriter};
use groan_rs::io::xtc_io::{XtcReader, XtcWriter};
use groan_rs::progress::ProgressPrinter;
use groan_rs::structures::dimension::Dimension;
use groan_rs::system::System;

use crate::argparse::Args;
use crate::errors::RunError;

/// Check that the simulation is valid (defined, non-zero and orthogonal).
fn check_simulation_box(system: &System) -> Result<(), RunError> {
    match system.get_box_as_ref() {
        None => return Err(RunError::BoxNotDefined),
        Some(x) => {
            if !x.is_orthogonal() {
                return Err(RunError::BoxNotOrthogonal);
            }

            if x.x <= 0.0 || x.y <= 0.0 || x.z <= 0.0 {
                return Err(RunError::BoxNotValid);
            }
        }
    };

    Ok(())
}

/// Ignore error returned by `check_simulation_box` and print a warning instead.
/// Used when centering a trajectory.
fn simbox_error_to_warning(error: Result<(), RunError>, silent: bool) {
    if !silent {
        match error {
            Ok(_) => (),
            Err(RunError::BoxNotDefined) => eprintln!("{} input structure file has an undefined simulation box.\n", "warning:".yellow().bold()),
            Err(RunError::BoxNotValid) => eprintln!("{} input structure file has an invalid simulation box (some dimensions are not positive).\n", "warning:".yellow().bold()),
            Err(RunError::BoxNotOrthogonal) => eprintln!("{} input structure file has a non-orthogonal simulation box.\n", "warning:".yellow().bold()),
            Err(_) => panic!("\ngcenter: Fatal Error. Unexpected error type returned when checking the simulation box."),
        }
    }
}

/// Center the reference group and write an output gro or pdb file.
fn center_structure_file(
    system: &mut System,
    output: &str,
    output_type: FileType,
    operations: Vec<(String, Dimension)>,
    com: bool,
    whole: bool,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    check_simulation_box(system)?;

    for (group, dims) in operations.iter() {
        if com {
            system.atoms_center_mass(group, *dims)?
        } else {
            system.atoms_center(group, *dims)?
        }
    }

    if whole {
        system.make_molecules_whole()?;
    }

    match output_type {
        FileType::GRO => system.write_gro(output, system.has_velocities())?,
        FileType::PDB => system.write_pdb(output, system.has_bonds())?,
        _ => panic!("\ngcenter: Fatal Error. Output file has unsupported file extension but this should have been handled before."),
    }

    Ok(())
}

/// Center a trajectory.
fn center_trajectory<'a, Read>(
    reader: TrajReader<'a, Read>,
    args: &Args,
    writer: &mut impl TrajWrite,
    operations: Vec<(String, Dimension)>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>>
where
    Read: TrajRead<'a> + TrajRangeRead<'a> + TrajStepRead<'a>,
    <Read as TrajRead<'a>>::FrameData: FrameDataTime,
{
    let reader = match (args.start_time, args.end_time) {
        (None, None) => reader.with_range(0.0, f32::MAX)?,
        (Some(start), None) => reader.with_range(start, f32::MAX)?,
        (None, Some(end)) => reader.with_range(0.0, end)?,
        (Some(start), Some(end)) => reader.with_range(start, end)?,
    };
    let mut reader = reader.with_step(args.step)?;

    if !args.silent {
        reader = reader.print_progress(
            ProgressPrinter::new()
                .with_running_msg("CENTERING".yellow())
                .with_newline_at_end(false),
        );
    }

    for frame in reader {
        let frame = frame?;

        for (group, dims) in operations.iter() {
            if args.com {
                frame.atoms_center_mass(group, *dims)?
            } else {
                frame.atoms_center(group, *dims)?
            }
        }

        if args.whole {
            frame.make_molecules_whole()?;
        }

        writer.write_frame()?;
    }

    Ok(())
}

/// Center all the provided trajectories.
fn center_trajectories(
    system: &mut System,
    args: &Args,
    writer: &mut impl TrajWrite,
    operations: Vec<(String, Dimension)>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    simbox_error_to_warning(check_simulation_box(system), args.silent);

    if args.trajectories.len() == 1 {
        match FileType::from_name(&args.trajectories[0]) {
            FileType::XTC => {
                let reader = system.xtc_iter(&args.trajectories[0])?;
                center_trajectory::<XtcReader>(reader, args, writer, operations)
            }
            FileType::TRR => {
                let reader = system.trr_iter(&args.trajectories[0])?;
                center_trajectory::<TrrReader>(reader, args, writer, operations)
            }
            _ => panic!("\ngcenter: Fatal Error. Input file has unsupported file extension but this should have been handled before."),
        }
    } else {
        match FileType::from_name(&args.trajectories[0]) {
            FileType::XTC => {
                let reader = system.xtc_cat_iter(&args.trajectories)?;
                center_trajectory::<TrajConcatenator<XtcReader>>(reader, args, writer, operations)
            },
            FileType::TRR => {
                let reader = system.trr_cat_iter(&args.trajectories)?;
                center_trajectory::<TrajConcatenator<TrrReader>>(reader, args, writer, operations)
            }
            _ => panic!("\ngcenter: Fatal Error. Input file has unsupported file extension but this should have been handled before."),
        }
    }
}

/// Center the structure or trajectory file.
pub fn center(
    system: &mut System,
    args: &Args,
    operations: Vec<(String, Dimension)>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // determine type of the output file
    let output_type = FileType::from_name(&args.output);

    if args.trajectories.is_empty() {
        // trajectory file not provided, center the structure file
        center_structure_file(
            system,
            &args.output,
            output_type,
            operations,
            args.com,
            args.whole,
        )?;
    } else {
        match output_type {
            FileType::XTC => {
                let mut writer = XtcWriter::new(system, &args.output)?;
                center_trajectories(system, args, &mut writer, operations)?
            }
            FileType::TRR => {
                let mut writer = TrrWriter::new(system, &args.output)?;
                center_trajectories(system, args, &mut writer, operations)?
            }
            _ => panic!("\ngcenter: Fatal Error. Output file has unsupported file extension but this should have been handled before."),
        };

        if !args.silent {
            println!("\n");
        }
    }

    Ok(())
}
