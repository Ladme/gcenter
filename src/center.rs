// Released under MIT License.
// Copyright (c) 2023 Ladislav Bartos

//! Implementation of the centering procedure.

use colored::Colorize;
use groan_rs::errors::ReadTrajError;
use groan_rs::files::FileType;
use groan_rs::prelude::*;
use std::path::Path;

use crate::argparse::Args;

/// Center the reference group and write an output gro or pdb file.
fn center_structure_file(
    system: &mut System,
    output: &str,
    output_type: FileType,
    dimension: Dimension,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    system.atoms_center("Reference", dimension)?;

    match output_type {
        FileType::GRO => system.write_gro(output, system.has_velocities())?,
        FileType::PDB => system.write_pdb(output, system.has_bonds())?,
        _ => panic!("Gcenter error. Output file has unsupported file extension but this should have been handled before."),
    }

    Ok(())
}

/// Check for duplicate frames at trajectory boundaries and center the frame.
fn handle_frame(
    frame: Result<&mut System, ReadTrajError>,
    dimension: Dimension,
    writer: &mut impl TrajWrite,
    last_step: Option<u64>,
    is_first_frame: &mut bool,
    com: bool,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let frame = frame?;

    if *is_first_frame {
        if let Some(step) = last_step {
            // skip this step if it matches the previous one
            if frame.get_simulation_step() == step {
                *is_first_frame = false;
                return Ok(());
            }
        }
        *is_first_frame = false;
    }

    if com {
        frame.atoms_center_mass("Reference", dimension)?;
    } else {
        frame.atoms_center("Reference", dimension)?;
    }

    writer.write_frame()?;

    Ok(())
}

/// Center any trajectory file.
fn center_traj_file<'a, Reader>(
    system: &'a mut System,
    trajectory: impl AsRef<Path>,
    writer: &mut impl TrajWrite,
    start_time: Option<f32>,
    end_time: Option<f32>,
    step: usize,
    dimension: Dimension,
    last_step: Option<u64>,
    is_last_file: bool,
    silent: bool,
    com: bool,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>>
where
    Reader: TrajRead<'a> + TrajRangeRead<'a> + TrajStepRead<'a> + 'a,
    Reader::FrameData: FrameDataTime,
{
    let mut reader = system.traj_iter::<Reader>(&trajectory)?;
    if !silent {
        reader = reader.print_progress(ProgressPrinter::new().with_running_msg("CENTERING".yellow()));
    }

    let mut is_first_frame = true;

    let process_frame = |frame| {
        handle_frame(
            frame,
            dimension,
            writer,
            last_step,
            &mut is_first_frame,
            com,
        )
    };

    match (start_time, end_time, step) {
        (Some(s), _, 1) => match reader.with_range(s, end_time.unwrap_or(f32::MAX)) {
            Ok(mut r) => r.try_for_each(process_frame)?,
            Err(ReadTrajError::StartNotFound(_)) if !is_last_file => (),
            Err(e) => return Err(Box::new(e)),
        },

        (_, Some(e), 1) => match reader.with_range(start_time.unwrap_or(0.0), e) {
            Ok(mut r) => r.try_for_each(process_frame)?,
            Err(ReadTrajError::StartNotFound(_)) if !is_last_file => (),
            Err(e) => return Err(Box::new(e)),
        },

        (None, None, 1) => reader.try_for_each(process_frame)?,

        (Some(s), _, step) => match reader.with_range(s, end_time.unwrap_or(f32::MAX)) {
            Ok(r) => r.with_step(step)?.try_for_each(process_frame)?,
            Err(ReadTrajError::StartNotFound(_)) if !is_last_file => (),
            Err(e) => return Err(Box::new(e)),
        },

        (_, Some(e), step) => match reader.with_range(start_time.unwrap_or(0.0), e) {
            Ok(r) => r.with_step(step)?.try_for_each(process_frame)?,
            Err(ReadTrajError::StartNotFound(_)) if !is_last_file => (),
            Err(e) => return Err(Box::new(e)),
        },

        (None, None, step) => reader.with_step(step)?.try_for_each(process_frame)?,
    };

    Ok(())
}

/// Center all the provided trajectories.
fn center_trajectories(
    system: &mut System,
    args: &Args,
    writer: &mut impl TrajWrite,
    dimension: Dimension,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    for (t, traj) in args.trajectories.iter().enumerate() {
        let input_type = FileType::from_name(traj);

        let last_step = match t {
            0 => None,
            _ => Some(system.get_simulation_step()),
        };

        // check whether this is a last file
        let is_last_file = t == args.trajectories.len() - 1;

        match input_type {
            FileType::XTC => center_traj_file::<XtcReader>(
                system,
                traj,
                writer,
                args.start_time,
                args.end_time,
                args.step,
                dimension,
                last_step,
                is_last_file,
                args.silent,
                args.com,
            )?,

            FileType::TRR => center_traj_file::<TrrReader>(
                system,
                traj,
                writer,
                args.start_time,
                args.end_time,
                args.step,
                dimension,
                last_step,
                is_last_file,
                args.silent,
                args.com
            )?,

            _ => panic!("Gcenter error. Input file has unsupported file extension but this should have been handled before."),
        }
    }

    Ok(())
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
        center_structure_file(system, &args.output, output_type, dimension)?;
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
            _ => panic!("Gcenter error. Output file has unsupported file extension but this should have been handled before."),
        };
    }

    Ok(())
}
