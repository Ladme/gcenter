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

/// Center the reference group and write an output gro or pdb file.
fn center_structure_file(
    system: &mut System,
    output: &str,
    output_type: FileType,
    dimension: Dimension,
    com: bool,
    whole: bool,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    if com {
        system.atoms_center_mass("Reference", dimension)?
    } else {
        system.atoms_center("Reference", dimension)?;
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
    dimension: Dimension,
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

        if args.com {
            frame.atoms_center_mass("Reference", dimension)?;
        } else {
            frame.atoms_center("Reference", dimension)?;
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
    dimension: Dimension,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    if args.trajectories.len() == 1 {
        match FileType::from_name(&args.trajectories[0]) {
            FileType::XTC => {
                let reader = system.xtc_iter(&args.trajectories[0])?;
                center_trajectory::<XtcReader>(reader, args, writer, dimension)
            }
            FileType::TRR => {
                let reader = system.trr_iter(&args.trajectories[0])?;
                center_trajectory::<TrrReader>(reader, args, writer, dimension)
            }
            _ => panic!("\ngcenter: Fatal Error. Input file has unsupported file extension but this should have been handled before."),
        }
    } else {
        match FileType::from_name(&args.trajectories[0]) {
            FileType::XTC => {
                let reader = system.xtc_cat_iter(&args.trajectories)?;
                center_trajectory::<TrajConcatenator<XtcReader>>(reader, args, writer, dimension)
            },
            FileType::TRR => {
                let reader = system.trr_cat_iter(&args.trajectories)?;
                center_trajectory::<TrajConcatenator<TrrReader>>(reader, args, writer, dimension)
            }
            _ => panic!("\ngcenter: Fatal Error. Input file has unsupported file extension but this should have been handled before."),
        }
    }
}

/// Center the structure or trajectory file.
pub fn center(
    system: &mut System,
    args: &Args,
    dimension: Dimension,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // determine type of the output file
    let output_type = FileType::from_name(&args.output);

    if args.trajectories.is_empty() {
        // trajectory file not provided, center the structure file
        center_structure_file(system, &args.output, output_type, dimension, args.com, args.whole)?;
    } else {
        match output_type {
            FileType::XTC => {
                let mut writer = XtcWriter::new(system, &args.output)?;
                center_trajectories(system, args, &mut writer, dimension)?
            }
            FileType::TRR => {
                let mut writer = TrrWriter::new(system, &args.output)?;
                center_trajectories(system, args, &mut writer, dimension)?
            }
            _ => panic!("\ngcenter: Fatal Error. Output file has unsupported file extension but this should have been handled before."),
        };

        if !args.silent {
            println!("\n");
        }
    }

    Ok(())
}
