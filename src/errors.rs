// Released under MIT License.
// Copyright (c) 2023-2024 Ladislav Bartos

//! Implementation of errors originating from the `gcenter` program.

use colored::Colorize;
use thiserror::Error;

/// Errors originating directly from `gcenter`.
#[derive(Error, Debug, PartialEq, Eq)]
pub enum RunError {
    #[error("{} invalid value '{}' for '{}': output path matches input path\n\nFor more information, try '{}'.", "error:".red().bold(), .0.yellow(), "--output <OUTPUT>".bold(), "--help".bold())]
    IOMatch(String),
    #[error("{} invalid value '{}' for '{}': unsupported file extension\n\nFor more information, try '{}'.", "error:".red().bold(), .0.yellow(), "--output <OUTPUT>".bold(), "--help".bold())]
    OutputUnsupported(String),
    #[error("{} invalid values '{}' and '{}' for '{}': paths correspond to the same file\n\nFor more information, try '{}'.", "error:".red().bold(), .0.yellow(), .1.yellow(), "--trajectory [<TRAJECTORIES>...]".bold(), "--help".bold())]
    IdenticalInputFiles(String, String),
    #[error("{} invalid value '{}' for '{}': input structure file does not exist\n\nFor more information, try '{}'.", "error:".red().bold(), .0.to_string().yellow(), "--structure <STRUCTURE>".bold(), "--help".bold())]
    InputStructureNotFound(String),
    #[error("{} invalid value '{}' for '{}': input trajectory file does not exist\n\nFor more information, try '{}'.", "error:".red().bold(), .0.to_string().yellow(), "--trajectory [<TRAJECTORIES>...]".bold(), "--help".bold())]
    InputTrajectoryNotFound(String),
    #[error("{} invalid value '{}' for '{}': tpr file can only be used as input structure file when trajectory is also provided\n\nFor more information, try '{}'.", "error:".red().bold(), .0.to_string().yellow(), "--structure <STRUCTURE>".bold(), "--help".bold())]
    TprWithoutTrajectory(String),
    #[error("{} invalid value '{}' for '{}': query contains `molecule with` keyword; this is only supported if a tpr file is provided\n\nFor more information, try '{}'.", "error:".red().bold(), .0.to_string().yellow(), "--reference <REFERENCE>".bold(), "--help".bold())]
    UnsupportedQuery(String),
    #[error("{} invalid values '{}' and '{}' for '{}': all trajectory files must have the same file format\n\nFor more information, try '{}'.", "error:".red().bold(), .0.to_string().yellow(), .1.to_string().yellow(), "--trajectory [<TRAJECTORIES>...]".bold(), "--help".bold())]
    InconsistentTrajectoryFiles(String, String),
    #[error("{} invalid argument '{}': this option is only supported when a tpr file is provided\n\nFor more information, try '{}'.", "error:".red().bold(), "--whole".bold(), "--help".bold())]
    WholeRequiresTprFile,
    #[error("{} reference group '{}' is empty\n", "error:".red().bold(), .0.yellow())]
    EmptyReference(String),
    #[error("{} no protein atoms autodetected\n", "error:".red().bold())]
    AutodetectionFailed,
    #[error("{} simulation box is not orthogonal; this is not supported, sorry\n", "error:".red().bold())]
    BoxNotOrthogonal,
    #[error("{} simulation box is not a valid simulation box; some required dimensions are not positive\n", "error:".red().bold())]
    BoxNotValid,
    #[error("{} simulation box is not defined\n", "error:".red().bold())]
    BoxNotDefined,
}
