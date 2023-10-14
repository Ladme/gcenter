// Released under MIT License.
// Copyright (c) 2023 Ladislav Bartos

//! Implementation of the centering procedure.

use colored::{ColoredString, Colorize};
use groan_rs::errors::ReadTrajError;
use groan_rs::files::FileType;
use groan_rs::prelude::*;
use std::io::{self, Write};
use std::path::Path;

use crate::argparse::Args;

#[derive(Debug, Clone, Copy, PartialEq)]
enum ProgressStatus {
    Running,
    Completed,
    Failed,
}

/// Structure containing formatting for the printing of the centering progress.
#[derive(Debug, PartialEq)]
struct ProgressPrinter {
    status: ProgressStatus,
    print_freq: u64,
    silent: bool,
    step_fmt: ColoredString,
    time_fmt: ColoredString,
    running_fmt: ColoredString,
    completed_fmt: ColoredString,
    failed_fmt: ColoredString,
}

impl ProgressPrinter {
    /// Create default instance of the `ProgressPrinter`.
    fn new(silent: bool) -> Self {
        ProgressPrinter {
            silent,
            print_freq: 500000,
            status: ProgressStatus::Running,
            step_fmt: "Step:".cyan(),
            time_fmt: "Time:".bright_purple(),
            running_fmt: "CENTERING".yellow(),
            completed_fmt: "COMPLETED".green(),
            failed_fmt: " FAILED! ".red(),
        }
    }

    /// Print progress info with formatting from `ProgressPrinter`.
    fn print(&self, sim_step: u64, sim_time: u64) {
        if !self.silent
            && (self.status != ProgressStatus::Running || sim_step % self.print_freq == 0)
        {
            match self.status {
                ProgressStatus::Running => print!("[{: ^9}]   ", self.running_fmt),
                ProgressStatus::Completed => print!("[{: ^9}]   ", self.completed_fmt),
                ProgressStatus::Failed => print!("[{: ^9}]   ", self.failed_fmt),
            }

            print!(
                "{} {:12} | {} {:12} ps\r",
                self.step_fmt, sim_step, self.time_fmt, sim_time
            );

            io::stdout().flush().unwrap();
        }
    }

    fn set_status(&mut self, status: ProgressStatus) {
        self.status = status;
    }
}

/// Center the reference group and write an output gro or pdb file.
fn center_structure_file(
    system: &mut System,
    output: &str,
    output_type: FileType,
    dimension: Dimension,
) -> Result<(), Box<dyn std::error::Error>> {
    system.atoms_center("Reference", dimension)?;

    match output_type {
        FileType::GRO => system.write_gro(output, system.has_velocities())?,
        FileType::PDB => system.write_pdb(output)?,
        _ => panic!("Gcenter error. Output file has unsupported file extension but this should have been handled before."),
    }

    Ok(())
}

/// Center a single simulation frame.
fn center_frame(
    frame: &mut System,
    dimension: Dimension,
    writer: &mut impl TrajWrite,
    printer: &ProgressPrinter,
) -> Result<(), Box<dyn std::error::Error>> {
    printer.print(
        frame.get_simulation_step(),
        frame.get_simulation_time() as u64,
    );
    frame.atoms_center("Reference", dimension)?;
    writer.write_frame()?;

    Ok(())
}

/// Check for duplicate frames at trajectory boundaries and center the frame.
fn handle_frame(
    frame: Result<&mut System, ReadTrajError>,
    dimension: Dimension,
    writer: &mut impl TrajWrite,
    printer: &ProgressPrinter,
    last_step: Option<u64>,
    is_first_frame: &mut bool,
) -> Result<(), Box<dyn std::error::Error>> {
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

    center_frame(frame, dimension, writer, printer)
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
    printer: &ProgressPrinter,
    last_step: Option<u64>,
    is_last_file: bool,
) -> Result<(), Box<dyn std::error::Error>>
where
    Reader: TrajRead<'a> + TrajRangeRead<'a> + TrajStepRead<'a> + 'a,
    Reader::FrameData: FrameDataTime,
{
    let mut reader = system.traj_iter::<Reader>(&trajectory)?;

    let mut is_first_frame = true;

    let process_frame = |frame| {
        handle_frame(
            frame,
            dimension,
            writer,
            printer,
            last_step,
            &mut is_first_frame,
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
    printer: &ProgressPrinter,
) -> Result<(), Box<dyn std::error::Error>> {
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
                printer,
                last_step,
                is_last_file,
            )?,

            FileType::TRR => center_traj_file::<TrrReader>(
                system,
                traj,
                writer,
                args.start_time,
                args.end_time,
                args.step,
                dimension,
                printer,
                last_step,
                is_last_file,
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
) -> Result<(), Box<dyn std::error::Error>> {
    // determine type of the output file
    let output_type = FileType::from_name(&args.output);

    if args.trajectories.is_empty() {
        // trajectory file not provided, center the structure file
        center_structure_file(system, &args.output, output_type, dimension)?;
    } else {
        let mut printer = ProgressPrinter::new(args.silent);

        let return_type = match output_type {
            FileType::XTC => {
                let mut writer = XtcWriter::new(system, &args.output)?;
                center_trajectories(system, args, &mut writer, dimension, &printer)
            }
            FileType::TRR => {
                let mut writer = TrrWriter::new(system, &args.output)?;
                center_trajectories(system, args, &mut writer, dimension, &printer)
            }
            _ => panic!("Gcenter error. Output file has unsupported file extension but this should have been handled before."),
        };

        match return_type {
            Ok(_) => {
                printer.set_status(ProgressStatus::Completed);
                printer.print(
                    system.get_simulation_step(),
                    system.get_simulation_time() as u64,
                );

                if !args.silent {
                    println!("\n")
                }
            }
            Err(e) => {
                printer.set_status(ProgressStatus::Failed);
                printer.print(
                    system.get_simulation_step(),
                    system.get_simulation_time() as u64,
                );

                if !args.silent {
                    println!("\n")
                }

                return Err(e);
            }
        }
    }

    Ok(())
}
